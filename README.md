# cakeup

A prototype binary bootstrapper for Cake.  
This application will do things like:  

* Install Cake
* Install NuGet.exe
* Install .NET Core SDK
* Bootstrap modules
* Execute script

## Disclaimer

**This is a prototype, so please do not use cakeup yet since we might
simply remove this repository at any point in time until we've decided 
to keep this around or not.**

## Pull requests

**Pull requests are currently NOT accepted for this project.**

## Usage

```
USAGE:
    cakeup [FLAGS] [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -t, --trace      Show trace information.
    -V, --version    Prints version information

SUBCOMMANDS:
    help    Prints this message or the help of the given subcommand(s)
    run     Runs installation and execution of Cake and related tools
```

### Run command

```
USAGE:
    cakeup run [FLAGS] [OPTIONS] [-- <remaining>...]

FLAGS:
        --bootstrap    Bootstraps Cake modules.
        --coreclr      Use the CoreCLR version of Cake.
        --execute      Executes the Cake script.
    -h, --help         Prints help information
    -V, --version      Prints version information

OPTIONS:
        --cake <cake>      The version of Cake to install.
        --nuget <nuget>    The version of NuGet to install.
        --sdk <sdk>        The version of the .NET Core SDK to install.

ARGS:
    <remaining>...    Arguments that will be sent to Cake.
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
Also install version 1.1.7 of the .NET Core SDK and bootstrap Cake.

```
> ./cakeup.exe --cake=0.25.0 --sdk=1.1.7 --coreclr --bootstrap

Creating tools directory...
Downloading https://www.nuget.org/api/v2/package/Cake.CoreClr/0.25.0...
Unzipping binaries...
Creating .dotnet directory...
Downloading https://dot.net/v1/dotnet-install.ps1...
Installing .NET Core SDK...
Verifying installation...
Bootstrapping script (dotnet)...
```

### Example 2

Install CLR version of Cake 0.25.0.  
Also bootstrap Cake and execute the build.cake script.  
Use diagnostic verbosity when calling Cake and also pass the flag `lol`.

```
> ./cakeup.exe --cake=0.25.0 --bootstrap --execute -- --verbosity=diagnostic --lol

Creating tools directory...
Downloading https://www.nuget.org/api/v2/package/Cake.CoreClr/0.25.0...
Unzipping binaries...
Bootstrapping script (CLR)...
Module directory does not exist.
NuGet.config not found.
Analyzing D:/Source/github/cake-build/cakeup/target/release/build.cake...
Executing script (CLR)...
Module directory does not exist.
NuGet.config not found.
Analyzing build script...
Analyzing D:/Source/github/cake-build/cakeup/target/release/build.cake...
Processing build script...
Compiling build script...
Hello World!
```