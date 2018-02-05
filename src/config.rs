// Licensed to the .NET Foundation under one or more agreements.
// The .NET Foundation licenses this file to you under the MIT license.
// See the LICENSE file in the project root for more information.

use std::path::PathBuf;

#[derive(Debug)]
pub struct Config {
    pub root: PathBuf,
    pub cake_version: Version,
    pub script: Script,
    pub nuget_version: Version,
    pub sdk_version: Version,
    pub use_coreclr: bool,
    pub bootstrap: bool,
    pub remaining: Vec<String>,
}

#[derive(Debug)]
pub enum Version {
    None,
    Latest,
    Specific(String),
}

#[allow(dead_code)]
impl Version {
    pub fn has_version(&self) -> bool {
        match self {
            &Version::None => return false,
            _ => return true
        }
    }

    pub fn want_latest(&self) -> bool {
        match self {
            &Version::Latest => return true,
            _ => return false
        }
    }

    pub fn get_version(&self) -> &str {
        match self {
            &Version::None => return "",
            &Version::Latest => return "latest",
            &Version::Specific(ref v) => return v
        }
    }
}

#[derive(Debug)]
pub enum Script {
    Default,
    Specific(PathBuf),
}