$CakeVersion = "0.25.0";
$NuGetVersion = "none";
$DotnetVersion = "1.1.7";
$CoreClr = $true;
$Bootstrap = $false;

# Fix up the script root.
if(!$PSScriptRoot){
    $PSScriptRoot = Split-Path $MyInvocation.MyCommand.Path -Parent
}

# Make sure that cakeup is present.
$CakeUp = Join-Path $PSScriptRoot "cakeup.exe"
if (!(Test-Path $CakeUp)) {
    Write-Verbose -Message "Downloading cakeup.exe..."
    try {        
        $wc = (New-Object System.Net.WebClient);
        $wc.DownloadFile("https://github.com/patriksvensson/example/releases/download/v0.4.0/cakeup.exe", $CakeUp) } catch {
            Throw "Could not download cakeup.exe."
    }
}

# Execute Cakeup
&$CakeUp "--cake=$CakeVersion" "--nuget=$NuGetVersion" `
         "--sdk=$DotnetVersion" "--coreclr=$CoreClr" `
         "--bootstrap=$Bootstrap" "--execute" "--" "$args"

# Return the exit code from Cakeup.
exit $LASTEXITCODE;