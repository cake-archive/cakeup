# cakeup

A binary bootstrapper for Cake.  
This application will do things like:  

* Install Cake
* Install NuGet.exe
* Install dotnet SDK
* Bootstrap modules
* Execute script

## Usage

```
Usage: cakeup [--cake=<VERSION>] [--script=<SCRIPT>]
              [--nuget=<VERSION>] [--sdk=<VERSION>]
              [--coreclr] [--bootstrap] [-- ARGUMENTS]

  --cake   <VERSION>  The version of Cake to install.
  --script <SCRIPT>   The script to execute.
  --nuget  <VERSION>  The version of NuGet to install.
  --sdk    <VERSION>  The version of the dotnet SDK to install.
  --coreclr           Use CoreCLR version of Cake.
  --bootstrap         Bootstraps Cake modules.
  --execute           Executes the Cake script.
  --version           Prints version information.
  --help              Prints help information.
```

You can also set parameters to cakeup via the following
environment variables. Note that this does not override
parameters directly to cakeup.

```
CAKEUP_CAKE       = "0.24.0"
CAKEUP_SCRIPT     = "test.cake"
CAKEUP_NUGET      = "latest"
CAKEUP_SDK        = "1.1.7"
CAKEUP_EXECUTE    = "true"
CAKEUP_CORECLR    = "true"
CAKEUP_BOOTSTRAP  = "true"
```

## Useage examples

### Example 1

Install CoreCLR version of Cake 0.25.0.  
Also install version 1.1.7 of the dotnet SDK and bootstrap Cake.

```
> ./cakeup.exe --cake=0.25.0 --sdk=1.1.7 --coreclr --bootstrap

Creating tools directory...
Downloading Cake.CoreClr 0.24.0...
Unzipping binaries...
Creating .dotnet directory...
Downloading .NET Core SDK installation script...
Installing .NET Core SDK...
Bootstrapping build.cake...
Done!
```

### Example 2

Install CLR version of Cake 0.25.0.  
Also bootstrap Cake and execute the build.cake script.  
Use diagnostic verbosity when calling Cake and also pass the flag `lol`.

```
> ./cakeup.exe --cake=0.25.0 --bootstrap --execute -- --verbosity=diagnostic --lol

Creating tools directory...
Downloading Cake 0.25.0...
Unzipping binaries...
Bootstrapping build.cake...
Module directory does not exist.
NuGet.config not found.
Analyzing D:/Source/github/cake-build/cakeup/target/release/build.cake...
Executing build.cake...
Module directory does not exist.
NuGet.config not found.
Analyzing build script...
Analyzing D:/Source/github/cake-build/cakeup/target/release/build.cake...
Processing build script...
Compiling build script...
Hello World!
```