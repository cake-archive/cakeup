// Licensed to the .NET Foundation under one or more agreements.
// The .NET Foundation licenses this file to you under the MIT license.
// See the LICENSE file in the project root for more information.

extern crate getopts;
use self::getopts::{Options, Matches};

use std::env;
use std::io::{Error, ErrorKind};

#[derive(Debug)]
pub struct Arguments {
    pub cake: String,
    pub nuget: String,
    pub sdk: String,
    pub coreclr: bool,
    pub bootstrap: bool,
    pub execute: bool,
    pub help: bool,
    pub version: bool,
    pub arguments: Vec<String>,
}

pub fn parse() -> Result<Arguments, Error> {
    let mut options = Options::new();
    options.optopt("", "cake", "", "VERSION");
    options.optopt("", "nuget", "", "VERSION");
    options.optopt("", "sdk", "", "VERSION");
    options.optflagopt("", "coreclr", "", "");
    options.optflagopt("", "bootstrap", "", "");
    options.optflagopt("", "execute", "", "");
    options.optflag("h", "help", "");
    options.optflag("", "version", "");

    let args: Vec<String> = env::args().collect();
    let matches = match options.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            return Err(Error::new(ErrorKind::Other, f.to_string()));
        }
    };

    // Parse versions.
    let cake = parse_string(&matches, "cake", "CAKEUP_CAKE", "latest");
    let nuget = parse_string(&matches, "nuget", "CAKEUP_NUGET", "none");
    let sdk = parse_string(&matches, "sdk", "CAKEUP_SDK", "none");
    let coreclr = parse_bool(&matches, "coreclr", "CAKE_CORECLR");
    let bootstrap = parse_bool(&matches, "bootstrap", "CAKE_BOOTSTRAP");
    let execute = parse_bool(&matches, "execute", "CAKE_EXECUTE");
    let help = matches.opt_present("help");
    let version = matches.opt_present("version");

    // We currently have no way of knowing what is the latest version of the SDK.
    if sdk == "latest" {
        return Err(Error::new(ErrorKind::Other, "You must specify a specific SDK version or none at all."));
    }

    return Ok(Arguments {
        cake,
        nuget,
        sdk,
        coreclr,
        bootstrap,
        execute,
        help,
        version,
        arguments: env::args().skip_while(|a| a != "--").skip(1).collect(),
    });
}

fn parse_string(matches: &Matches, arg_name: &str, env_name: &str, default: &str) -> String {
    return match matches.opt_str(arg_name) {
        None => String::from(env::var(env_name).unwrap_or(String::from(default))),
        Some(v) => {
            match &v[..] {
                "" => String::from(default),
                _ => String::from(v)
            }
        }
    };
}

fn parse_bool(matches: &Matches, arg_name: &str, env_name: &str) -> bool {
    if matches.opt_present(arg_name) {
        let value = parse_string(matches, arg_name, env_name, "true");
        return value.to_lowercase() == "true";
    }
    return String::from(env::var(env_name).unwrap_or(String::from("false"))) == "true";;
}
