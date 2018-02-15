// Licensed to the .NET Foundation under one or more agreements.
// The .NET Foundation licenses this file to you under the MIT license.
// See the LICENSE file in the project root for more information.

use std::io::{Error, ErrorKind};
use std::path::PathBuf;
use std::process::{Command, ExitStatus};
use semver::Version;
use config::*;
use utils::*;

pub struct Cake {
    pub path: PathBuf,
    pub version: Version,
    pub host: Host,
}

#[derive(PartialEq)]
pub enum Host {
    Clr,
    CoreClr,
    Mono,
}

impl Cake {
    pub fn bootstrap(&self, config: &Config) -> Result<(), Error> {
        // Is bootstrapping supported?
        if self.version < Version::parse("0.24.0").unwrap() {
            println!("Warning: Bootstrapping requires at lest verison 0.24.0 of Cake.");
            return Ok(());
        }

        let result: ExitStatus;
        match self.host {
            Host::Clr => {
                println!("Bootstrapping (CLR)...");
                result = Command::new(&self.path)
                    .arg("--bootstrap")
                    .args(&config.remaining)
                    .status()?;
            }
            Host::CoreClr | Host::Mono => {
                let mut host = "dotnet";
                if self.host == Host::Mono {
                    host = "mono";
                    println!("Bootstrapping (mono)...");
                } else {
                    println!("Bootstrapping (dotnet)...");
                }
                result = Command::new(host)
                    .arg(&self.path)
                    .arg("--bootstrap")
                    .args(&config.remaining)
                    .status()?;
            }
        };

        if result.code() != Some(0) {
            return Err(Error::new(ErrorKind::Other,
                format!("Exit code was {}.", result.code().unwrap())))
        }

        return Ok(());
    }

    pub fn execute(&self, config: &Config) -> Result<(), Error> {
        let result: ExitStatus;
        match self.host {
            Host::Clr => {
                println!("Executing (CLR)...");
                result = Command::new(&self.path)
                    .args(&config.remaining)
                    .status()?;
            }
            Host::CoreClr | Host::Mono => {
                let mut host = "dotnet";
                if self.host == Host::Mono {
                    host = "mono";
                    println!("Executing (mono)...");
                } else {
                    println!("Executing (dotnet)...");
                }
                result = Command::new(host)
                    .arg(&self.path)
                    .args(&config.remaining)
                    .status()?;
            }
        };

        if result.code() != Some(0) {
            return Err(Error::new(ErrorKind::Other,
                format!("Exit code was {}.", result.code().unwrap())))
        }

        return Ok(());
    }
}

pub fn install(config: &Config) -> Result<Cake, Error> {

    // Get the version we're going to use.
    let mut version = String::from(&config.cake_version.as_ref().unwrap()[..]);
    if version == "latest" {
        println!("Figuring out what the latest release of Cake is...");
        let release = github::get_latest_release("cake-build", "cake")?;
        version = String::from(&release.name[1..]); // Github releases are prefixed with "v".
    }

    // What flavor of Cake do we want to download?
    let mut flavor = "Cake";
    if config.use_coreclr {
        flavor = "Cake.CoreClr";
    }

    // Install Cake.
    let cake_folder_path = config
        .tools
        .join(format!("{0}.{1}", flavor.to_lowercase(), version));
    if !cake_folder_path.exists() {
        let cake_nupkg_path = config.tools.join(get_cake_package_name(&config, &version));
        if !cake_nupkg_path.exists() {
            println!("Downloading {0} {1}...", flavor, version);
            let url = &format!(
                "https://www.nuget.org/api/v2/package/{0}/{1}",
                flavor, version
            );
            http::download(
                &url,
                &cake_nupkg_path,
                Some(&format!("Cakeup NuGet Client/{0}", ::utils::version::VERSION)[..]),
            )?;
        }

        // Nupkg files are just zip files, so unzip it.
        println!("Unzipping binaries...");
        zip::unzip(&cake_nupkg_path, &cake_folder_path)?;
    }

    // What host should we use?
    let mut host = Host::Clr;
    if config.use_coreclr {
        host = Host::CoreClr;
    } else {
        // Running on OSX/Linux/BSD?
        if cfg!(unix) {
            host = Host::Mono;
        }
    }

    // Get the filename
    let cake_filename = if config.use_coreclr {
        "Cake.dll"
    } else {
        "Cake.exe"
    };

    return Ok(Cake {
        path: cake_folder_path.join(&cake_filename),
        version: Version::parse(&version).unwrap(),
        host: host,
    });
}

pub fn should_install_cake(config: &Config) -> bool {
    return match config.cake_version {
        None => false,
        _ => true
    }
}

fn get_cake_package_name(config: &Config, version: &String) -> String {
    if config.use_coreclr {
        return format!("cake.coreclr.{}.nupkg", version);
    }
    return format!("cake.{}.nupkg", version);
}
