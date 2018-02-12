// Licensed to the .NET Foundation under one or more agreements.
// The .NET Foundation licenses this file to you under the MIT license.
// See the LICENSE file in the project root for more information.

use std::env;
use std::path::{PathBuf};
use args;

#[derive(Debug)]
pub struct Config {
    pub root: PathBuf,
    pub tools: PathBuf,
    pub cake_version: String,
    pub nuget_version: Option<String>,
    pub sdk_version: Option<String>,
    pub use_coreclr: bool,
    pub bootstrap: bool,
    pub execute_script: bool,
    pub remaining: Vec<String>,
}

impl Config {
    pub fn new(args: &args::Arguments) -> Self {

        // Get other paths.
        let root = env::current_dir().unwrap();
        let tools = root.join("tools");

        return Config {
            root,
            tools,
            cake_version: create_option(&args.cake, false).unwrap_or("latest".to_string()),
            nuget_version: create_option(&args.nuget, true),
            sdk_version: create_option(&args.sdk, false),
            use_coreclr: args.coreclr,
            bootstrap: args.bootstrap,
            execute_script: args.execute,
            remaining: args.arguments.clone()
        };
    }

    pub fn should_install_nuget(&self) -> bool {
        return match self.nuget_version {
            None => false,
            _ => true
        }
    }

    pub fn should_install_dotnet(&self) -> bool {
        return match self.sdk_version {
            None => false,
            _ => true
        }
    }
}

fn create_option(value: &String, prefix: bool) -> Option<String> {
    return match value.as_ref() {
        "" => None,
        "none" => None,
        "latest" => Some("latest".to_string()),
        _ => {
            if prefix {
                return Some(format!("v{}", value.clone()));
            }
            return Some(value.clone());
        }
    };
}
