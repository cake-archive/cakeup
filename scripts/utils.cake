public static ProcessArgumentBuilder AppendIf(this ProcessArgumentBuilder builder, bool condition, string text)
{
    if(condition)
    {
        builder.Append(text);
    }
    return builder;
}

public static string GetPlatformName(ICakeContext context, string target = null)
{
    switch(context.Environment.Platform.Family)
    {
        case PlatformFamily.Windows:
            return "windows";
        case PlatformFamily.Linux:
        {
            if(target == "x86_64-unknown-linux-musl") {
                return "linux-musl";
            }
            return "linux";
        }
        case PlatformFamily.OSX:
            return "osx";
    }
    throw new InvalidOperationException("Could not get platform name.");
}

public static DirectoryPath GetTargetDirectory(ICakeContext context, string target = null)
{
    if(string.IsNullOrWhiteSpace(target))
    {
        return new DirectoryPath("./target/release");
    }
    return new DirectoryPath($"./target/{target}/release");
}

public static string GetTargetFilename(ICakeContext context)
{
    switch(context.Environment.Platform.Family)
    {
        case PlatformFamily.Windows:
            return "cakeup.exe";
        default:
            return "cakeup";
    }
}

public static void EnsureEnvironmentVariable(ICakeContext context, string key, string expected = null)
{
    if(!context.HasEnvironmentVariable(key))
    {
        throw new InvalidOperationException($"Environment variable '{key}' has not been set.");
    }
    if(expected != null)
    {
        var value = context.EnvironmentVariable(key);
        if(!string.Equals(value, expected))
        {
            throw new InvalidOperationException($"Expected environment variable '{key}' to be '{expected}', but as '{value}'.");
        }
    }
}