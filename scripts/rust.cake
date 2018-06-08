#load "utils.cake"

public static class Rust
{
    public static void Build(ICakeContext context, string target = null)
    {
        if(!string.IsNullOrWhiteSpace(target)) 
        {
            // Ensure target is installed.
            context.StartProcess("rustup", new ProcessSettings {
                Arguments = new ProcessArgumentBuilder()
                    .Append("target")
                    .Append("add")
                    .Append(target)
            });
        }

        context.Information("Building ({0})...", target ?? "default");
        context.StartProcess("cargo", new ProcessSettings {
            Arguments = new ProcessArgumentBuilder()
                .Append("+nightly")
                .Append("build")
                .AppendIf(target != null, $"--target={target}")
                .Append("--release")
        });

        // Not running on Windows?
        if(context.Environment.Platform.Family != PlatformFamily.Windows)
        {
            // Remove inessential information from executable.
            // This way we make the binary size smaller.
            var path = GetTargetDirectory(context, target);
            context.StartProcess("strip", new ProcessSettings {
                Arguments = new ProcessArgumentBuilder()
                    .Append(path.CombineWithFilePath("cakeup").FullPath)
            });
        }
    }
}