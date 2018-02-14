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