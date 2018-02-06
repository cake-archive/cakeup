// Licensed to the .NET Foundation under one or more agreements.
// The .NET Foundation licenses this file to you under the MIT license.
// See the LICENSE file in the project root for more information.

use std::path::PathBuf;
use args;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Config {
    pub root: PathBuf,
    pub cake_version: Option<String>,
    pub script: Option<String>,
    pub nuget_version: Option<String>,
    pub sdk_version: Option<String>,
    pub use_coreclr: bool,
    pub bootstrap: bool,
    pub remaining: Vec<String>,
}

impl Config {
    pub fn new(args: &args::Arguments) -> Self {
        return Config {
            root: PathBuf::from(String::from("")),
            cake_version: None,
            script: None,
            nuget_version: None,
            sdk_version: None,
            use_coreclr: false,
            bootstrap: false,
            remaining: args.arguments.clone()
        };
    }
}