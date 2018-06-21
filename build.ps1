[CmdletBinding()]
Param(
    [Parameter(Position=0,Mandatory=$false,ValueFromRemainingArguments=$true)]
    [string[]]$ScriptArgs
)

# Pin versions
$CakeupVersion = "v0.2.93"
$CakeVersion = "0.28.1"

# Get the script root folder.
if(!$PSScriptRoot) {
    $PSScriptRoot = Split-Path $MyInvocation.MyCommand.Path -Parent
}

# Create the tools folder.
$Tools = Join-Path $PSScriptRoot "tools"
if (!(Test-Path $Tools)) {
    New-Item -Path $Tools -ItemType Directory | Out-Null
}

# Make sure that cakeup is present.
$Cakeup = Join-Path $Tools "cakeup-x86_64-$CakeupVersion.exe"
if (!(Test-Path $Cakeup)) {
    Write-Host "Downloading cakeup.exe ($CakeupVersion)..."
    try {
        $wc = (New-Object System.Net.WebClient);
        $wc.DownloadFile("https://cakeup.blob.core.windows.net/windows/cakeup-x86_64-$CakeupVersion.exe", $Cakeup) } catch {
            Throw "Could not download cakeup.exe."
    }
}

# Execute Cakeup
&$Cakeup "run" "--cake=$CakeVersion" "--sdk=2.1.4" "--execute" "--" $ScriptArgs

# Return the exit code from Cakeup.
exit $LASTEXITCODE;
