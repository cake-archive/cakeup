extern crate getopts;

use std::env;
use std::process;
use args::getopts::{Matches, Options};

#[derive(Debug)]
pub enum Version {
    None,
    Latest,
    Specific(String),
}

#[derive(Debug)]
pub enum Script {
    Default(String),
    Specific(String),
}

#[derive(Debug)]
pub struct Arguments {
    pub cake_version: Version,
    pub script: Script,
    pub nuget_version: Version,
    pub sdk_version: Version,
    pub use_coreclr: bool,
    pub bootstrap: bool,
    pub remaining: Vec<String>,
}

pub fn parse() -> Arguments {
    let mut options = Options::new();
    options.optopt("", "cake", "", "VERSION");
    options.optopt("", "script", "", "SCRIPT");
    options.optopt("", "nuget", "", "VERSION");
    options.optopt("", "sdk", "", "VERSION");
    options.optflag("", "coreclr", "");
    options.optflag("", "bootstrap", "");
    options.optflag("h", "help", "");

    let args: Vec<String> = env::args().collect();
    let matches = match options.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            print_error_and_exit(&f.to_string()[..]);
        }
    };

    // Should we show help?
    if matches.opt_present("help") {
        print_help();
        process::exit(0);
    }

    // Parse versions.
    let cake_version = parse_version(&matches, "cake");
    let nuget_version = parse_version(&matches, "nuget");
    let sdk_version = parse_version(&matches, "sdk");

    // Parse the script.
    let script: Script = match matches.opt_str("script") {
        None => Script::Default(String::from("build.cake")),
        Some(s) => Script::Specific(s),
    };

    // Parse flags.
    let use_coreclr = matches.opt_present("coreclr");
    let bootstrap = matches.opt_present("bootstrap");

    // Make sure that SDK isn't set to latest version
    // since we currently have no way of knowing what
    // is the latest version of the SDK.
    match sdk_version {
        Version::Latest => {
            print_error_and_exit("You must specify a specific SDK version or none at all.")
        }
        _ => {}
    }

    return Arguments {
        cake_version,
        script,
        nuget_version,
        sdk_version,
        use_coreclr,
        bootstrap,
        remaining: env::args().skip_while(|a| a != "--").skip(1).collect(),
    };
}

fn print_help() {
    println!("Usage: cakeup [--cake=<VERSION>] [--script=<SCRIPT>]");
    println!("              [--nuget=<VERSION>] [--sdk=<VERSION>]");
    println!("              [--coreclr] [--bootstrap] [-- ARGUMENTS]\n");
    println!("  --cake   <VERSION>  The version of Cake to install.");
    println!("  --script <SCRIPT>   The script to execute.");
    println!("  --nuget  <VERSION>  The version of NuGet to install.");
    println!("  --sdk    <VERSION>  The version of the dotnet SDK to install.");
    println!("  --coreclr           Use CoreCLR version of Cake.");
    println!("  --bootstrap         Bootstrap Cake modules.");
    println!("  --help              Prints help information.");
}

fn print_error_and_exit(text: &str) -> ! {
    println!("Error: {}\n", text);
    print_help();
    process::exit(1);
}

fn parse_version(matches: &Matches, name: &str) -> Version {
    return match matches.opt_str(name) {
        None => Version::None,
        Some(n) => {
            if n == "latest" {
                return Version::Latest;
            }
            return Version::Specific(n);
        }
    };
}
