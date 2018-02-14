#addin "nuget:?package=WindowsAzure.Storage&version=9.0.0"

using Microsoft.WindowsAzure.Storage;
using Microsoft.WindowsAzure.Storage.Auth;
using Microsoft.WindowsAzure.Storage.Blob;

public class AzureFileClient
{
    public static async Task Upload(Stream stream, string connectionString, string containerName, string filename)
    {
        var storage = CloudStorageAccount.Parse(connectionString);
        var client = storage.CreateCloudBlobClient();

        var container = client.GetContainerReference(containerName);
        await container.CreateIfNotExistsAsync();

        var blob = container.GetBlockBlobReference($"{filename}");
        await blob.UploadFromStreamAsync(stream);
    }
}