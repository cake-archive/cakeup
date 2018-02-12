#!/usr/bin/env bash
cake_version="0.25.0";
nuget_version="none";
dotnet_version="1.1.7";
coreclr=true;
bootstrap=false;

# Fix up the script root.
script_dir=$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )
tools_dir="$script_dir/tools"

# Make sure the tools folder exist.
if [ ! -d "$tools_dir" ]; then
  mkdir "$tools_dir"
fi

# Install .NET CLI
echo "Installing .NET CLI..."
if [ ! -d "$script_dir/.dotnet" ]; then
  mkdir "$script_dir/.dotnet"
fi
curl -Lsfo "$script_dir/.dotnet/dotnet-install.sh" https://dot.net/v1/dotnet-install.sh
sudo bash "$script_dir/.dotnet/dotnet-install.sh" --version 1.1.7 --install-dir .dotnet --no-path
export PATH="$script_dir/.dotnet":$PATH
export DOTNET_SKIP_FIRST_TIME_EXPERIENCE=1
export DOTNET_CLI_TELEMETRY_OPTOUT=1
"$script_dir/.dotnet/dotnet" --info

# Download and install Cake
cake_exe=$tools_dir/Cake.$cake_version/Cake.dll
if [ ! -f "$cake_exe" ]; then
    echo "Installing Cake $cake_version..."
    curl -Lsfo Cake.zip "https://www.nuget.org/api/v2/package/Cake.CoreClr/$cake_version" && unzip -q Cake.zip -d "$tools_dir/Cake.$cake_version" && rm -f Cake.zip
    if [ $? -ne 0 ]; then
        echo "An error occured while installing Cake."
        exit 1
    fi
fi

# Start Cake
(exec ./.dotnet/dotnet "$cake_exe" --bootstrap) && (exec ./.dotnet/dotnet "$cake_exe" "$@")