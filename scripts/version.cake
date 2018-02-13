public static class CakeVersion
{
    public static string Calculate(ICakeContext context)
    {
        var branch = GetBranch(context);
        if(string.IsNullOrWhiteSpace(branch) || branch == "HEAD") {
            throw new InvalidOperationException("Could not retrieve branch from Git.");
        }

        var tag = GetTag(context);
        if(string.IsNullOrWhiteSpace(tag)) {
            throw new InvalidOperationException("Could not retrieve tag from Git.");
        }

        // Get the commit count since tag.
        var commits = GetCommitsSinceTag(context, tag);
        var version = Version.Parse(tag.Trim('v'));

        // Create the version depending on the branchg.
        if(branch.Equals("master", StringComparison.OrdinalIgnoreCase)) {
            return $"{version.Major}.{version.Minor}.{commits}";
        }
        if(branch.Equals("develop", StringComparison.OrdinalIgnoreCase)) {
            return $"{version.Major}.{version.Minor}.{version.Build}-alpha{commits}";
        }
        return $"{version.Major}.{version.Minor}.{version.Build}-{branch}-build{commits}";
    }

    private static string GetBranch(ICakeContext context)
    {
        var ci = context.BuildSystem().TravisCI;
        if(ci.IsRunningOnTravisCI) {
            return ci.Environment.Build.Branch;
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

    private static string GetTag(ICakeContext context)
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

    private static string GetCommitsSinceTag(ICakeContext context, string tag) 
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
