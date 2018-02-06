// Licensed to the .NET Foundation under one or more agreements.
// The .NET Foundation licenses this file to you under the MIT license.
// See the LICENSE file in the project root for more information.

use std::io;
use config::*;
use commands::Command;

pub struct HelpCommand { }
impl Command for HelpCommand {
    fn run(&self, _config: &Config) -> Result<(), io::Error> {
        println!("Usage: cakeup [--cake=<VERSION>] [--script=<SCRIPT>]");
        println!("              [--nuget=<VERSION>] [--sdk=<VERSION>]");
        println!("              [--coreclr] [--bootstrap] [-- ARGUMENTS]\n");
        println!("  --cake   <VERSION>  The version of Cake to install.");
        println!("  --script <SCRIPT>   The script to execute.");
        println!("  --nuget  <VERSION>  The version of NuGet to install.");
        println!("  --sdk    <VERSION>  The version of the dotnet SDK to install.");
        println!("  --coreclr           Use CoreCLR version of Cake.");
        println!("  --bootstrap         Bootstrap Cake modules.");
        println!("  --version           Prints version information.");
        println!("  --help              Prints help information.");
        return Ok(());
    }
}