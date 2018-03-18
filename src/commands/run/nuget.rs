// Licensed to the .NET Foundation under one or more agreements.
// The .NET Foundation licenses this file to you under the MIT license.
// See the LICENSE file in the project root for more information.

use std::io::{Error};
use super::Config;
use utils::*;

pub fn install(config: &Config) -> Result<(), Error> {
    if should_install(config) {
        let file = config.tools.join("nuget.exe");
        if !file.exists() {
            let version = config.nuget_version.as_ref().unwrap();
            let url = format!("https://dist.nuget.org/win-x86-commandline/{}/nuget.exe", version);
            println!("Downloading {}...", url);
            http::download(&url, &file, Some("Cakeup"))?;
            println!("Installed Nuget.");
        } else {
            println!("Nuget is already installed.");
        }
    }
    return Ok(());
}

pub fn should_install(config: &Config) -> bool {
    return match config.nuget_version {
        None => false,
        _ => true
    }
}