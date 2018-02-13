#load "./scripts/version.cake"

///////////////////////////////////////////////////////////////////////////////
// ARGUMENTS
///////////////////////////////////////////////////////////////////////////////

var target = Argument("target", "Default");
var configuration = Argument("configuration", "Release");

Setup(context => {
    Information("Version: {0}", CakeVersion.Calculate(context));
});

///////////////////////////////////////////////////////////////////////////////
// TASKS
///////////////////////////////////////////////////////////////////////////////

Task("Dummy");

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

///////////////////////////////////////////////////////////////////////////////
// TARGETS
///////////////////////////////////////////////////////////////////////////////

Task("Default")
    .IsDependentOn("Build");

///////////////////////////////////////////////////////////////////////////////
// EXECUTION
///////////////////////////////////////////////////////////////////////////////

RunTarget(target);