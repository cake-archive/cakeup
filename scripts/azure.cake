#addin "nuget:?package=WindowsAzure.Storage&version=9.0.0"
#load "./utils.cake"

using Microsoft.WindowsAzure.Storage;
using Microsoft.WindowsAzure.Storage.Auth;
using Microsoft.WindowsAzure.Storage.Blob;

public class AzureFileClient
{
    public static async Task Upload(ICakeContext context, FilePath path, string filename)
    {
        var platform = GetPlatformName(context);

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