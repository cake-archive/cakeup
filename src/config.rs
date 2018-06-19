// Licensed to the .NET Foundation under one or more agreements.
// The .NET Foundation licenses this file to you under the MIT license.
// See the LICENSE file in the project root for more information.

use std::path::PathBuf;

pub struct Config {
    pub root: PathBuf,
    pub tools: PathBuf,
    pub cake_version: Option<String>,
    pub nuget_version: Option<String>,
    pub sdk_version: Option<String>,
    pub use_coreclr: bool,
    pub bootstrap: bool,
    pub execute_script: bool,
    pub remaining: Vec<String>
}

impl Config {
    pub fn should_create_tools_directory(&self) -> bool {
        return self.cake_version != None || self.nuget_version != None || self.sdk_version != None;
    }
}