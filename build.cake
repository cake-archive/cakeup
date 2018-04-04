#load "./scripts/version.cake"
#load "./scripts/azure.cake"
#load "./scripts/git.cake"
#load "./scripts/utils.cake"
#load "./scripts/rust.cake"

using System.Text.RegularExpressions;

///////////////////////////////////////////////////////////////////////////////
// ARGUMENTS
///////////////////////////////////////////////////////////////////////////////

var target = Argument("target", "Default");
var configuration = Argument("configuration", "Release");
var musl = !HasArgument("skip-musl");

///////////////////////////////////////////////////////////////////////////////
// VARIABLES
///////////////////////////////////////////////////////////////////////////////

var version = "0.0.0";
var deploy = false;

///////////////////////////////////////////////////////////////////////////////
// SETUP/TEARDOWN
///////////////////////////////////////////////////////////////////////////////

Setup(context => 
{
    // Get the version.
    version = CakeVersion.Calculate(context);
    if(BuildSystem.AppVeyor.IsRunningOnAppVeyor) {
        // Update the AppVeyor version number.
        BuildSystem.AppVeyor.UpdateBuildVersion(version);
    }

    // Determine if we should deploy or not.
    var branch = GitUtils.GetBranch(context);
    if(branch.Equals("master", StringComparison.OrdinalIgnoreCase)) {
        deploy = !BuildSystem.IsLocalBuild;
    }

    Information("Version: {0}", version);
});

///////////////////////////////////////////////////////////////////////////////
// TASKS
///////////////////////////////////////////////////////////////////////////////

Task("Patch-Version")
    .WithCriteria(() => deploy)
    .Does(() => 
{
    var path = File("./Cargo.toml").Path;
    var input = System.IO.File.ReadAllText(path.FullPath);
    var result = new Regex("version = \"[0-9]\\.[0-9]\\.[0-9]\"").Replace(input, $"version = \"{version}\"");
    System.IO.File.WriteAllText(path.FullPath, result);
});

Task("Set-Nightly-Compiler")
    .Does(context => 
{
    StartProcess("rustup", new ProcessSettings {
        Arguments = new ProcessArgumentBuilder()
            .Append("default")
            .Append("nightly")
    });
});

Task("Build-OpenSSL")
    .IsDependentOn("Set-Nightly-Compiler")
    .WithCriteria(() => Context.Environment.Platform.Family == PlatformFamily.Linux && musl)
    .Does(context => 
{
    EnsureEnvironmentVariable(context, "OPENSSL_DIR");
    var location = new DirectoryPath(context.EnvironmentVariable("OPENSSL_DIR"));
    if(DirectoryExists(location)) 
    {
        Information("Found OpenSSL, so no need to build it.");
        return;
    }

    Information("Building OpenSSL ({0}). This will take a while...", location);
    var process = System.Diagnostics.Process.Start(new System.Diagnostics.ProcessStartInfo
    {
        FileName = "/bin/bash",
        UseShellExecute = true,
        Arguments = string.Format("-c \"sudo -E {0} {1}\"", 
            MakeAbsolute(Directory("./scripts/shell/build_openssl_musl.sh")).FullPath,
            location)
    });

    process.WaitForExit();
});

Task("Build")
    .IsDependentOn("Patch-Version")
    .IsDependentOn("Set-Nightly-Compiler")
    .IsDependentOn("Build-OpenSSL")
    .Does(context => 
{
    Rust.Build(context);

    if(context.Environment.Platform.Family == PlatformFamily.Linux && musl)
    {
        // Build for MUSL.
        EnsureEnvironmentVariable(context, "OPENSSL_STATIC", "1");
        EnsureEnvironmentVariable(context, "OPENSSL_DIR");
        Rust.Build(context, "--target=x86_64-unknown-linux-musl");
    }
});

Task("Deploy")
    .WithCriteria(() => deploy)
    .IsDependentOn("Build")
    .Does(async context => 
{
    await AzureFileClient.UploadArtifacts(context, version);

    // Building on Linux?
    if(context.Environment.Platform.Family == PlatformFamily.Linux && musl)
    {
        // Upload MUSL artifacts as well.
        await AzureFileClient.UploadArtifacts(context, version, "--target=x86_64-unknown-linux-musl");
    }
});

///////////////////////////////////////////////////////////////////////////////
// TARGETS
///////////////////////////////////////////////////////////////////////////////

Task("Default")
    .IsDependentOn("Deploy");

///////////////////////////////////////////////////////////////////////////////
// EXECUTION
///////////////////////////////////////////////////////////////////////////////

RunTarget(target);