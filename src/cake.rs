// Licensed to the .NET Foundation under one or more agreements.
// The .NET Foundation licenses this file to you under the MIT license.
// See the LICENSE file in the project root for more information.

use std::env;
use std::fs;
use std::path::PathBuf;

use failure;
use semver::Version;

use host::Host;
use utils::CakeupResult;
use utils::{http, zip};
use Config;

pub struct Package {
    pub name: String,
    pub version: Version,
    pub filename: String,
    pub directory: PathBuf,
    pub core_clr: bool,
}

impl Package {
    pub fn new(config: &Config, version: &Version) -> Self {
        let name = if config.use_coreclr {
            "Cake.CoreClr"
        } else {
            "Cake"
        };
        let directory = config
            .tools
            .join(format!("{0}.{1}", name.to_lowercase(), version));
        let filename = format!("{}.{}.nupkg", name.to_lowercase(), version);
        return Package {
            name: name.to_string(),
            version: version.clone(),
            core_clr: config.use_coreclr,
            directory,
            filename,
        };
    }

    pub fn get_path(&self) -> PathBuf {
        let extension = if self.core_clr {
            "dll"
        } else {
            "exe"
        };
        return self.directory.join(format!("{0}.{1}", &self.name, extension));
    }

    pub fn get_url(&self) -> String {
        return format!(
            "https://www.nuget.org/api/v2/package/{0}/{1}",
            self.name, self.version
        );
    }
}

pub struct Cake {
    pub path: PathBuf,
    pub version: Version,
    pub host: Host,
}

impl Cake {
    pub fn bootstrap(&self, config: &Config) -> CakeupResult<i32> {
        // Is bootstrapping supported?
        if self.version < Version::parse("0.24.0").unwrap() {
            warn!("Bootstrapping requires at lest version 0.24.0 of Cake.");
            return Ok(-1);
        }

        let mut args = vec![String::from("--bootstrap")];
        let remaining = &config.remaining;
        args.extend(remaining.iter().cloned());

        info!("Bootstrapping script ({})...", self.host.get_name());
        return self.execute_script(&args);
    }

    pub fn execute(&self, config: &Config) -> CakeupResult<i32> {
        info!("Executing script ({})...", self.host.get_name());
        return self.execute_script(&config.remaining);
    }

    fn execute_script(&self, args: &Vec<String>) -> CakeupResult<i32> {
        &self.host.verify()?; // Verify the host.
        trace!("Executing cake from: {}", &self.path.to_string_lossy());
        let result = &self.host.execute(&self.path, args)?;
        return match result.code() {
            Some(n) => Ok(n),
            None => Err(format_err!(
                "An unknown error occured when executing script."
            )),
        };
    }
}

pub fn should_install(config: &Config) -> bool {
    return config.cake_version != None;
}

pub fn install(config: &Config) -> CakeupResult<Option<Cake>> {
    if !should_install(&config) {
        return Ok(Option::None);
    }

    // Get the version we're going to use.
    let version = match Version::parse(&config.cake_version.as_ref().unwrap()[..]) {
        Ok(v) => v,
        Err(_) => return Err(failure::err_msg("Provided Cake version is not valid.")),
    };

    let package = Package::new(config, &version);
    install_package(&package)?;

    return Ok(Option::Some(Cake {
        path: package.get_path(),
        version: package.version.clone(),
        host: Host::from_config(config),
    }));
}

fn install_package(package: &Package) -> CakeupResult<()> {
    if !package.directory.exists() {
        trace!("Creating package directory...");
        fs::create_dir(package.directory.to_str().unwrap())?;

        let cake_nupkg_path = package.get_path();
        if !cake_nupkg_path.exists() {
            fetch_package(&package)?;
        }
        trace!("Unzipping {} binaries...", package.name);
        zip::unzip(&cake_nupkg_path, &package.directory)?;
        info!("Installed {} ({}).", package.name, package.version);
    } else {
        info!(
            "{} ({}) is already installed.",
            package.name, &package.version
        );
    }
    return Ok(());
}

fn fetch_package(package: &Package) -> CakeupResult<()> {
    let path = package.get_path();
    let home = env::home_dir();
    if home.is_some() {
        let packages_path = home
            .unwrap()
            .join(".nuget")
            .join("packages")
            .join(&package.name.to_lowercase())
            .join(format!("{}", &package.version))
            .join(&package.filename);

        if packages_path.exists() {
            trace!(
                "Copying {} package from global package cache...",
                package.name
            );
            let bytes_copied = fs::copy(packages_path, &path)?;
            if bytes_copied > 0 {
                return Ok(());
            }
        }
    }

    let url = package.get_url();
    trace!("Downloading {}...", url);
    let user_agent = &format!("Cakeup NuGet Client/{}", ::utils::version::VERSION)[..];
    http::download(&url, &path, Some(user_agent))?;

    return Ok(());
}
