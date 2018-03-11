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
            // TODO: temporary hack to get things to compile...
            tag = "v0.2";
        }

        // Get the commit count since tag.
        var commits = GitUtils.GetCommitsSinceTag(context, tag);
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
}
