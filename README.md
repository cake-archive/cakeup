# cakeup

A binary bootstrapper for Cake.  
This application will do things like:  

* Install Cake
* Install NuGet.exe
* Install dotnet SDK
* Bootstrap modules

## Usage

```
Usage: cakeup [--cake=<latest|VERSION>] [--script=SCRIPT]
              [--nuget=<latest|VERSION>] [--sdk=VERSION]
              [--coreclr] [--bootstrap]

  --cake   <VERSION>  The version of Cake to install.
  --script <SCRIPT>   The script to execute.
  --nuget  <VERSION>  The version of NuGet to install.
  --sdk    <VERSION>  The version of the dotnet SDK to install.
  --coreclr           Use CoreCLR version of Cake.
  --bootstrap         Bootstrap Cake modules.
```

You can also set parameters to cakeup via the following
environment variables. Note that this does not override
parameters directly to cakeup.

```
CAKEUP_CAKE       = "0.24.0"
CAKEUP_SCRIPT     = "test.cake"
CAKEUP_NUGET      = "latest"
CAKEUP_SDK        = "1.1.7"
CAKEUP_CORECLR    = "true"
CAKEUP_BOOTSTRAP  = "true"
```

## Example

Install CoreCLR version of Cake 0.25.0.
Also install version 1.1.7 of the dotnet SDK.

```
> ./cakeup --cake=0.25.0 --coreclr --sdk=1.1.7
```