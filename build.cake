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
    .WithCriteria(() => deploy, "Not patching version since this is a local build.")
    .Does(() => 
{
    var path = File("./Cargo.toml").Path;
    var input = System.IO.File.ReadAllText(path.FullPath);
    var result = new Regex("version = \"[0-9]\\.[0-9]\\.[0-9]\"").Replace(input, $"version = \"{version}\"");
    System.IO.File.WriteAllText(path.FullPath, result);
});

Task("Build-OpenSSL")
    .WithCriteria(() => Context.Environment.Platform.Family == PlatformFamily.Linux, "Not on Linux.")
    .WithCriteria(() => musl, "Argument --skip-musl was set.")
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
    .IsDependentOn("Build-OpenSSL")
    .Does(context => 
{
    Rust.Build(context);

    if(context.Environment.Platform.Family == PlatformFamily.Linux && musl)
    {
        // Build for MUSL.
        EnsureEnvironmentVariable(context, "OPENSSL_STATIC", "1");
        EnsureEnvironmentVariable(context, "OPENSSL_DIR");
        Rust.Build(context, "x86_64-unknown-linux-musl");
    }
});

Task("Smoke-Tests")
    .IsDependentOn("Build")
    .Does(context => 
{
    var root = GetTargetDirectory(context);
    var filename = GetTargetFilename(context);
    var path = MakeAbsolute(root.CombineWithFilePath(filename));

    var exitCode = context.StartProcess(path, new ProcessSettings {
        WorkingDirectory = root,
        Arguments = new ProcessArgumentBuilder()
            .Append("--trace")
            .Append("run")
            .Append("--cake=0.28.1")
            .Append("--nuget=latest")
            .Append("--sdk=2.1.4")
            .Append("--coreclr")
    });

    if(exitCode != 0)
    {
        throw new CakeException("Smoke tests failed. See log for more information.");
    }
});

Task("Deploy")
    .WithCriteria(() => deploy, "Not deploying since this is a local build.")
    .IsDependentOn("Smoke-Tests")
    .Does(async context => 
{
    await AzureFileClient.UploadArtifacts(context, version);

    // Building on Linux?
    if(context.Environment.Platform.Family == PlatformFamily.Linux && musl)
    {
        // Upload MUSL artifacts as well.
        await AzureFileClient.UploadArtifacts(context, version, "x86_64-unknown-linux-musl");
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