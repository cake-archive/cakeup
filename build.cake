#load "./scripts/version.cake"
#load "./scripts/azure.cake"
#load "./scripts/git.cake"
#load "./scripts/utils.cake"

using System.Text.RegularExpressions;

///////////////////////////////////////////////////////////////////////////////
// ARGUMENTS
///////////////////////////////////////////////////////////////////////////////

var target = Argument("target", "Default");
var configuration = Argument("configuration", "Release");

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
    .WithCriteria(() => Context.Environment.Platform.Family == PlatformFamily.Linux)
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

Task("Build-Linux")
    .IsDependentOn("Patch-Version")
    .IsDependentOn("Build-OpenSSL")
    .WithCriteria(() => Context.Environment.Platform.Family == PlatformFamily.Linux)
    .Does(context => 
{
    // Build Cakeup for Linux.
    Information("Building for Linux...");
    StartProcess("cargo", new ProcessSettings {
        Arguments = new ProcessArgumentBuilder()
            .Append("build")
            .Append("--release")
    });

    // Remove inessential information from executable.
    // This way we make the binary size smaller.
    var path = GetTargetDirectory(context);
    StartProcess("strip", new ProcessSettings {
        Arguments = new ProcessArgumentBuilder()
            .Append(path.CombineWithFilePath("cakeup").FullPath)
    });

    EnsureEnvironmentVariable(context, "OPENSSL_STATIC", "1");
    EnsureEnvironmentVariable(context, "OPENSSL_DIR");

    // Ensure MUSL target is installed.
    StartProcess("rustup", new ProcessSettings {
        Arguments = new ProcessArgumentBuilder()
            .Append("target")
            .Append("add")
            .Append("x86_64-unknown-linux-musl")
    });

    // Build Cakeup for Linux.
    Information("Building for Linux (MUSL)...");
    StartProcess("cargo", new ProcessSettings {
        Arguments = new ProcessArgumentBuilder()
            .Append("build")
            .Append("--target=x86_64-unknown-linux-musl")
            .Append("--release")
    });

    // Remove inessential information from executable.
    // This way we make the binary size smaller.
    path = GetTargetDirectory(context);
    StartProcess("strip", new ProcessSettings {
        Arguments = new ProcessArgumentBuilder()
            .Append(path.CombineWithFilePath("cakeup").FullPath)
    });
});

Task("Build")
    .IsDependentOn("Patch-Version")
    .IsDependentOn("Set-Nightly-Compiler")
    .WithCriteria(() => Context.Environment.Platform.Family != PlatformFamily.Linux)
    .Does(context => 
{
    // Build Cakeup for Windows or MacOS.
    StartProcess("cargo", new ProcessSettings {
        Arguments = new ProcessArgumentBuilder()
            .Append("build")
            .Append("--release")
    });

    // Not running on Windows?
    if(context.Environment.Platform.Family != PlatformFamily.Windows)
    {
        // Remove inessential information from executable.
        // This way we make the binary size smaller.
        var path = GetTargetDirectory(context);
        StartProcess("strip", new ProcessSettings {
            Arguments = new ProcessArgumentBuilder()
                .Append(path.CombineWithFilePath("cakeup").FullPath)
        });
    }
});

Task("Deploy")
    .WithCriteria(() => deploy)
    .IsDependentOn("Build")
    .IsDependentOn("Build-Linux")
    .Does(async context => 
{
    AzureFileClient.UploadArtifacts(context, version);

    // Building on Linux?
    if(context.Environment.Platform.Family == PlatformFamily.Linux)
    {
        // Upload MUSL artifacts as well.
        AzureFileClient.UploadMuslArtifacts(context, version);
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