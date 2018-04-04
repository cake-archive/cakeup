#addin "nuget:?package=WindowsAzure.Storage&version=9.0.0"
#load "./utils.cake"

using Microsoft.WindowsAzure.Storage;
using Microsoft.WindowsAzure.Storage.Auth;
using Microsoft.WindowsAzure.Storage.Blob;

public class AzureFileClient
{
    public static async Task UploadArtifacts(ICakeContext context, string version, string target = null)
    {
        var platform = GetPlatformName(context, target);
        var filename = GetTargetFilename(context);
        var path = GetTargetDirectory(context, target).CombineWithFilePath(filename);

        // Upload as current version.
        await AzureFileClient.Upload(context, path, platform,
            platform == "windows" 
                ? $"cakeup-x86_64-v{version}.exe"
                : $"cakeup-x86_64-v{version}");

        // Overwrite the latest version.
        await AzureFileClient.Upload(context, path, platform,
            platform == "windows" 
                ? $"cakeup-x86_64-latest.exe"
                : $"cakeup-x86_64-latest");
    }

    private static async Task Upload(ICakeContext context, FilePath path, string platform, string filename)
    {
        var connection = context.EnvironmentVariable("CAKEUP_AZURE_STORAGE");
        if(string.IsNullOrWhiteSpace(connection)) 
        {
            throw new InvalidOperationException("Could not resolve Azure connection string.");
        }

        context.Information("Uploading executable to Azure ({0}/{1})...", platform, filename);
        using(var stream = context.FileSystem.GetFile(path).OpenRead())
        {
            await AzureFileClient.Upload(stream, connection, platform, filename);
        }
    }

    private static async Task Upload(Stream stream, string connectionString, string containerName, string filename)
    {
        var storage = CloudStorageAccount.Parse(connectionString);
        var client = storage.CreateCloudBlobClient();

        var container = client.GetContainerReference(containerName);
        await container.CreateIfNotExistsAsync();

        var blob = container.GetBlockBlobReference($"{filename}");
        await blob.UploadFromStreamAsync(stream);
    }
}