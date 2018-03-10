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

Task("Build")
    .IsDependentOn("Patch-Version")
    .Does(c => 
{
    // Are we building on Linux?
    // If so, we want to build for MUSL.
    if(c.Environment.Platform.Family != PlatformFamily.Windows) {
        // Ensure MUSL target is installed.
        StartProcess("rustup", new ProcessSettings {
            Arguments = new ProcessArgumentBuilder()
                .Append("target")
                .Append("add")
                .Append("x86_64-unknown-linux-musl")
        });

        // Build Cakeup for Linux.
        StartProcess("cargo", new ProcessSettings {
            Arguments = new ProcessArgumentBuilder()
                .Append("build")
                .Append("--target=x86_64-unknown-linux-musl")
                .Append("--release")
        });    
    } else {
        // Build Cakeup for Windows or MacOS.
        StartProcess("cargo", new ProcessSettings {
            Arguments = new ProcessArgumentBuilder()
                .Append("build")
                .Append("--release")
        });
    }

    // Not running on Windows?
    if(c.Environment.Platform.Family != PlatformFamily.Windows) {
        // Remove inessential information from executable.
        // This way we make the binary size smaller.
        StartProcess("strip", new ProcessSettings {
            Arguments = new ProcessArgumentBuilder()
                .Append("target/release/cakeup")
        });
    }
});

Task("Deploy")
    .WithCriteria(() => deploy)
    .IsDependentOn("Build")
    .Does(async context => 
{
    var platform = GetPlatformName(context);
    var filename = platform == "windows" ? "cakeup.exe" : "cakeup";
    var path = File($"./target/release/{filename}");

    // Upload as current version.
    await AzureFileClient.Upload(context, path, platform == "windows" 
        ? $"cakeup-x86_64-v{version}.exe"
        : $"cakeup-x86_64-v{version}");

    // Overwrite the latest version.
    await AzureFileClient.Upload(context, path, platform == "windows" 
        ? $"cakeup-x86_64-latest.exe"
        : $"cakeup-x86_64-latest");
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