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
    pub cake_version: Option<String>,
    pub script: Option<PathBuf>,
    pub nuget_version: Option<String>,
    pub sdk_version: Option<String>,
    pub use_coreclr: bool,
    pub bootstrap: bool,
    pub remaining: Vec<String>,
}

impl Config {
    pub fn new(args: &args::Arguments) -> Self {

        // Get the absolute script path.
        let script = match args.script.as_ref() {
            "" => Some(env::current_dir().unwrap().join("build.cake")),
            _ => Some(PathBuf::from(args.script.clone()))
        };

        // Get other paths.
        let root = get_script_root(&script);
        let tools = root.join("tools");

        return Config {
            root,
            tools,
            cake_version: create_option(&args.cake, None),
            script,
            nuget_version: create_option(&args.nuget, Some("v")),
            sdk_version: create_option(&args.sdk, None),
            use_coreclr: args.coreclr,
            bootstrap: args.bootstrap,
            remaining: args.arguments.clone()
        };
    }

    pub fn should_install_nuget(&self) -> bool {
        return match self.nuget_version {
            None => false, 
            _ => true
        }
    }
}

fn create_option(value: &String, prefix: Option<&str>) -> Option<String> {
    return match value.as_ref() {
        "" => None,
        _ => {
            return match prefix {
                None => Some(value.clone()),
                Some(p) => Some(format!("{}{}", p, value.clone()))
            };
        }
    };
}

fn get_script_root(script: &Option<PathBuf>) -> PathBuf {
    match script {
        &None => return env::current_dir().unwrap(),
        &Some(ref path) => {
            if path.is_relative() {
                return env::current_dir().unwrap();
            }
            return path.parent().unwrap().to_path_buf();
        }
    };
}
