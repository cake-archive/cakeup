// Licensed to the .NET Foundation under one or more agreements.
// The .NET Foundation licenses this file to you under the MIT license.
// See the LICENSE file in the project root for more information.

use std::env;

use clap::{App, ArgMatches};
use cakeup::{self, CakeupResult, Config};

use commands::Command;

pub struct RunCommand {}
impl Command for RunCommand {
    fn run(&self, app: App) -> CakeupResult<i32> {
        let config = parse(app);
        return cakeup::run(config);
    }
}

pub fn parse(app: App) -> Config {
    let root = env::current_dir().unwrap();
    let tools = root.join("tools");

    let matches = app.get_matches();
    let args = matches.subcommand_matches("run").unwrap();

    let cake_version = parse_string_option(args, "cake", "CAKEUP_CAKE", "none", false);
    let nuget_version = parse_string_option(args, "nuget", "CAKEUP_NUGET", "none", false);
    let sdk_version = parse_string_option(args, "sdk", "CAKEUP_SDK", "none", false);
    let bootstrap = parse_bool(args, "bootstrap", "CAKEUP_BOOTSTRAP");
    let use_coreclr = parse_bool(args, "coreclr", "CAKEUP_CORECLR");
    let execute_script = parse_bool(args, "execute", "CAKEUP_EXECUTE");

    let mut remaining: Vec<String> = vec![];
    let raw_remaining = args.values_of("remaining")
        .map(|vals| vals.collect::<Vec<_>>());
        
    if raw_remaining.is_some() {
        for arg in raw_remaining.unwrap() {
            remaining.push(String::from(arg));
        }
    }

    return Config {
        root,
        tools,
        bootstrap,
        cake_version,
        nuget_version,
        sdk_version,
        use_coreclr,
        execute_script,
        remaining
    };
}

fn parse_string_option(
    matches: &ArgMatches,
    arg_name: &str,
    env_name: &str,
    default: &str,
    prefix: bool,
) -> Option<String> {
    let value = parse_string(matches, arg_name, env_name, default);
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

fn parse_string(matches: &ArgMatches, arg_name: &str, env_name: &str, default: &str) -> String {
    return match matches.value_of(arg_name) {
        None => String::from(env::var(env_name).unwrap_or(String::from(default))),
        Some(v) => match &v[..] {
            "" => String::from(default),
            _ => String::from(v),
        },
    };
}

fn parse_bool(matches: &ArgMatches, arg_name: &str, env_name: &str) -> bool {
    if matches.is_present(arg_name) {
        return true;
    }
    let value = env::var(env_name);
    if value.is_ok() {
        let value = value.unwrap();
        if value != "" {
            return value.to_lowercase() == "true";
        }
    }
    return false;
}