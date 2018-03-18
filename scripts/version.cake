#load "git.cake"

public static class CakeVersion
{
    public static string Calculate(ICakeContext context)
    {
        var branch = GitUtils.GetBranch(context);
        if(string.IsNullOrWhiteSpace(branch) || branch == "HEAD") {
            throw new InvalidOperationException("Could not retrieve branch from Git.");
        }

        var tag = GitUtils.GetTag(context);
        if(string.IsNullOrWhiteSpace(tag)) {
            if(branch == "master") {
                throw new InvalidOperationException("Could not retrieve tag from Git.");
            }

            // This is not ideal, but let's go with it for now...
            var normalizedBranchName = branch.Split('/').LastOrDefault() ?? branch;
            return $"0.0.1-{normalizedBranchName}";
        }

        // Get the commit count since tag.
        var commits = GitUtils.GetCommitsSinceTag(context, tag);
        var version = Version.Parse(tag.Trim('v'));

        // Create the version depending on the branchg.
        if(branch.Equals("master", StringComparison.OrdinalIgnoreCase)) {
            return $"{version.Major}.{version.Minor}.{commits}";
        }
        return $"{version.Major}.{version.Minor}.{version.Build}-{branch}-build{commits}";
    }
}
