public static string GetPlatformName(ICakeContext context)
{
    switch(context.Environment.Platform.Family)
    {
        case PlatformFamily.Windows:
            return "windows";
        case PlatformFamily.Linux:
            return "linux";
        case PlatformFamily.OSX:
            return "osx";
    }
    throw new InvalidOperationException("Could not get platform name.");
}

public static DirectoryPath GetTargetDirectory(ICakeContext context)
{
    switch(context.Environment.Platform.Family)
    {
        case PlatformFamily.Linux:
            return new DirectoryPath("./target/x86_64-unknown-linux-musl/release");
        default:
            return new DirectoryPath("./target/release");
    }
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