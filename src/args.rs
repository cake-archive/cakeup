// Licensed to the .NET Foundation under one or more agreements.
// The .NET Foundation licenses this file to you under the MIT license.
// See the LICENSE file in the project root for more information.

extern crate getopts;
use self::getopts::{Options};

use std::env;
use std::io::{Error, ErrorKind};

#[derive(Debug)]
pub struct Arguments {
    pub cake_version: String,
    pub script: String,
    pub nuget_version: String,
    pub sdk_version: String,
    pub use_coreclr: bool,
    pub bootstrap: bool,
    pub show_help: bool,
    pub show_version: bool,
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
    let script = matches.opt_str("script").unwrap_or(String::from("latest"));
    let cake_version = matches.opt_str("cake").unwrap_or(String::from("latest"));
    let nuget_version = matches.opt_str("nuget").unwrap_or(String::from(""));
    let sdk_version = matches.opt_str("sdk").unwrap_or(String::from(""));
    let use_coreclr = matches.opt_present("coreclr");
    let show_help = matches.opt_present("help");
    let show_version = matches.opt_present("version");
    let bootstrap = matches.opt_present("bootstrap");

    // We currently have no way of knowing what is the latest version of the SDK.
    if sdk_version == "latest" {
        return Err(Error::new(ErrorKind::Other, "You must specify a specific SDK version or none at all."));
    }

    return Ok(Arguments {
        cake_version,
        script,
        nuget_version,
        sdk_version,
        use_coreclr,
        bootstrap,
        show_help,
        show_version,
        arguments: env::args().skip_while(|a| a != "--").skip(1).collect(),
    });
}
