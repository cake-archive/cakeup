#!/usr/bin/env bash
script_dir=$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )

# Get the platform that we're running on.
uname_out="$(uname -s)"
case "${uname_out}" in
    Darwin*)    platform=osx;;
    *)          platform=linux
esac

# Make sure that cakeup exist.
cakeup="$script_dir/cakeup-x86_64-latest"
if [ ! -f "$cakeup" ]; then
    echo "Downloading Cakeup..."
    curl -Lsfo $cakeup "https://cakeup.blob.core.windows.net/$platform/cakeup-x86_64-latest"
    echo "Changing access permissions for Cakeup..."
    chmod +x $cakeup
    if [ $? -ne 0 ]; then
        echo "An error occured while downloading Cakeup."
        exit 1
    fi
fi

# Start Cake
exec $cakeup --cake="0.25.0" --nuget="latest" \
             --sdk="1.1.7" --coreclr \
             --bootstrap --execute -- $@