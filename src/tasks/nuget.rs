// Licensed to the .NET Foundation under one or more agreements.
// The .NET Foundation licenses this file to you under the MIT license.
// See the LICENSE file in the project root for more information.

use std::io;
use config::Config;
use tasks::Task;

pub struct NuGetTask { }
impl Task for NuGetTask {
    fn run(&self, config: &Config) -> Result<(), io::Error> {
        if config.nuget_version.has_version() {

            let nuget_path = config.root.join("nuget.exe");
            let version = config.nuget_version.get_version();

            if !nuget_path.exists() {
                println!("Downloading NuGet v{}...", version);
                // TODO: Download from https://dist.nuget.org/win-x86-commandline/{}/nuget.exe
            }
        }

        return Ok(());
    }
}