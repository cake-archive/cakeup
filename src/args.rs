// Licensed to the .NET Foundation under one or more agreements.
// The .NET Foundation licenses this file to you under the MIT license.
// See the LICENSE file in the project root for more information.

extern crate getopts;
use self::getopts::{Options};

use std::env;
use std::io::{Error, ErrorKind};

#[derive(Debug)]
pub struct Arguments {
    pub cake: String,
    pub script: String,
    pub nuget: String,
    pub sdk: String,
    pub coreclr: bool,
    pub bootstrap: bool,
    pub help: bool,
    pub version: bool,
    pub arguments: Vec<String>,
}

pub fn parse() -> Result<Arguments, Error> {
    let mut options = Options::new();
    options.optopt("", "cake", "", "VERSION");
    options.optopt("", "script", "", "SCRIPT");
    options.optopt("", "nuget", "", "VERSION");
    options.optopt("", "sdk", "", "VERSION");
    options.optflag("", "coreclr", "");
    options.optflag("", "bootstrap", "");
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
    let script = matches.opt_str("script").unwrap_or(String::from(""));
    let cake = matches.opt_str("cake").unwrap_or(String::from("latest"));
    let nuget = matches.opt_str("nuget").unwrap_or(String::from(""));
    let sdk = matches.opt_str("sdk").unwrap_or(String::from(""));
    let coreclr = matches.opt_present("coreclr");
    let help = matches.opt_present("help");
    let version = matches.opt_present("version");
    let bootstrap = matches.opt_present("bootstrap");

    // We currently have no way of knowing what is the latest version of the SDK.
    if sdk == "latest" {
        return Err(Error::new(ErrorKind::Other, "You must specify a specific SDK version or none at all."));
    }

    return Ok(Arguments {
        cake,
        script,
        nuget,
        sdk,
        coreclr,
        bootstrap,
        help,
        version,
        arguments: env::args().skip_while(|a| a != "--").skip(1).collect(),
    });
}
