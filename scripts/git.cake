public static class GitUtils
{
    public static string GetBranch(ICakeContext context)
    {
        context.Verbose("Git: Getting branch...");

        var ci = context.BuildSystem();
        if(ci.TravisCI.IsRunningOnTravisCI) {
            var branchName = ci.TravisCI.Environment.Build.Branch;
            context.Verbose("Returning AppVeyor branch: {0}", branchName);
            return branchName;
        }
        if(ci.AppVeyor.IsRunningOnAppVeyor) {
            var branchName = ci.AppVeyor.Environment.Repository.Branch;
            context.Verbose("Returning AppVeyor branch: {0}", branchName);
            return branchName;
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
        context.Verbose("Git: Getting latest tag...");

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
        context.Verbose("Git: Getting commits since tag...");

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