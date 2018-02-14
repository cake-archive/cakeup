#load "./scripts/version.cake"
#load "./scripts/azure.cake"
#load "./scripts/git.cake"
#load "./scripts/utils.cake"

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

Setup(context => {
    version = CakeVersion.Calculate(context);

    var branch = GitUtils.GetBranch(context);
    if(branch.Equals("master", StringComparison.OrdinalIgnoreCase)) {
        deploy = true;
    }

    Information("Version: {0}", version);
});

///////////////////////////////////////////////////////////////////////////////
// TASKS
///////////////////////////////////////////////////////////////////////////////

Task("Build")
    .Does(c => 
{
    // Build Cakeup.
    StartProcess("cargo", new ProcessSettings {
        Arguments = new ProcessArgumentBuilder()
            .Append("build")
            .Append("--release")
    });

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
    var connection = EnvironmentVariable("CAKEUP_AZURE_STORAGE");
    if(string.IsNullOrWhiteSpace(connection)) 
    {
        throw new InvalidOperationException("Could not resolve Azure connection string.");
    }

    var path = File("./target/release/cakeup.exe");

    var platform = GetPlatformName(context);
    var filename = platform == "windows" ? "cakeup.exe" : "cakeup";
    var remoteFilename = $"cakeup-x86_64-v{version}.exe";

    Information("Uploading executable to Azure ({0}/{1})...", platform, remoteFilename);
    using(var stream = context.FileSystem.GetFile(path).OpenRead())
    {
        await AzureFileClient.Upload(stream, connection, platform, remoteFilename);
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