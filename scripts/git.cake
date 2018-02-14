public static class GitUtils
{
    public static string GetBranch(ICakeContext context)
    {
        var ci = context.BuildSystem();
        if(ci.TravisCI.IsRunningOnTravisCI) {
            return ci.TravisCI.Environment.Build.Branch;
        }
        if(ci.AppVeyor.IsRunningOnAppVeyor) {
            return ci.AppVeyor.Environment.Repository.Branch;
        }

        using(var process = context.StartAndReturnProcess("git", new ProcessSettings 
        {
            RedirectStandardOutput = true,
            Arguments = new ProcessArgumentBuilder()
                .Append("rev-parse")
                .Append("--abbrev-ref HEAD"),
        }))
        {
            process.WaitForExit();
            return string.Join("", process.GetStandardOutput());
        }
    }

    public static string GetTag(ICakeContext context)
    {
        using(var process = context.StartAndReturnProcess("git", new ProcessSettings 
        {
            RedirectStandardOutput = true,
            Arguments = new ProcessArgumentBuilder()
                .Append("tag")
                .Append("-l")
                .Append("--merged master")
                .Append("--sort=\"-*authordate\"")
        }))
        {
            process.WaitForExit();
            return string.Join("", process.GetStandardOutput());
        }
    }

    public static string GetCommitsSinceTag(ICakeContext context, string tag) 
    {
        using(var process = context.StartAndReturnProcess("git", new ProcessSettings 
        {
            RedirectStandardOutput = true,
            Arguments = new ProcessArgumentBuilder()
                .Append("rev-list")
                .Append("HEAD")
                .Append($"^{tag}")
                .Append("--ancestry-path")
                .Append($"{tag}")
                .Append("--count"),
        }))
        {
            process.WaitForExit();
            return string.Join("", process.GetStandardOutput());
        }
    }
}