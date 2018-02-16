if(!$PSScriptRoot){
    $PSScriptRoot = Split-Path $MyInvocation.MyCommand.Path -Parent
}

# Make sure that cakeup is present.
$Cakeup = Join-Path $PSScriptRoot "cakeup-x86_64-latest.exe"
if (!(Test-Path $Cakeup)) {
    Write-Verbose -Message "Downloading cakeup.exe ($CakeupVersion)..."
    try {        
        $wc = (New-Object System.Net.WebClient);
        $wc.DownloadFile("https://cakeup.blob.core.windows.net/windows/cakeup-x86_64-latest.exe", $Cakeup) } catch {
            Throw "Could not download cakeup.exe."
    }
}

# Execute Cakeup
&$Cakeup "--cake=0.25.0" "--nuget=latest" `
         "--sdk=1.1.7" "--coreclr" `
         "--bootstrap" "--execute" "--" "$args"

# Return the exit code from Cakeup.
exit $LASTEXITCODE;