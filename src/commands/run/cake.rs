// Licensed to the .NET Foundation under one or more agreements.
// The .NET Foundation licenses this file to you under the MIT license.
// See the LICENSE file in the project root for more information.

use super::host::Host;
use super::Config;
use semver::Version;
use std::path::PathBuf;
use utils::CakeupResult;
use utils::*;

pub struct Cake {
    pub path: PathBuf,
    pub version: Version,
    pub host: Host,
}

impl Cake {
    pub fn bootstrap(&self, config: &Config) -> CakeupResult<i32> {
        // Is bootstrapping supported?
        if self.version < Version::parse("0.24.0").unwrap() {
            config.log.warning("Bootstrapping requires at lest version 0.24.0 of Cake.")?;
            return Ok(-1);
        }

        let mut args = vec![String::from("--bootstrap")];
        let remaining = &config.remaining;
        args.extend(remaining.iter().cloned());

        config.log.info(&format!(
            "Bootstrapping script ({})...",
            self.host.get_name()
        ))?;

        return self.execute_script(&args);
    }

    pub fn execute(&self, config: &Config) -> CakeupResult<i32> {
        config.log.info(&format!("Executing script ({})...", self.host.get_name()))?;
        return self.execute_script(&config.remaining);
    }

    fn execute_script(&self, args: &Vec<String>) -> CakeupResult<i32> {
        &self.host.verify()?; // Verify the host.
        let result = &self.host.execute(&self.path, args)?;
        return match result.code() {
            Some(n) => Ok(n),
            None => Err(format_err!(
                "An unknown error occured when executing script."
            )),
        };
    }
}

pub fn install(config: &Config) -> CakeupResult<Option<Cake>> {
    if !should_install(&config) {
        return Ok(Option::None);
    }

    // Get the version we're going to use.
    let mut version = String::from(&config.cake_version.as_ref().unwrap()[..]);
    if version == "latest" {
        config.log.info(&format!(
            "Figuring out what the latest release of Cake is..."
        ))?;
        let release = http::get_latest_github_release("cake-build", "cake")?;
        version = String::from(&release.name[1..]); // Github releases are prefixed with "v".
    }

    // Get the "flavor" of Cake to use.
    let flavor = get_cake_flavor(config);

    // Get the folder to where Cake should be installed.
    let cake_folder_path = config
        .tools
        .join(format!("{0}.{1}", flavor.to_lowercase(), version));

    // Do we need to download Cake?
    if !cake_folder_path.exists() {
        let cake_nupkg_path = config.tools.join(get_cake_package_name(&config, &version));
        if !cake_nupkg_path.exists() {
            let url = &format!(
                "https://www.nuget.org/api/v2/package/{0}/{1}",
                flavor, version
            );
            config.log.info(&format!("Downloading {}...", url))?;
            http::download(
                &url,
                &cake_nupkg_path,
                Some(&format!("Cakeup NuGet Client/{0}", ::utils::version::VERSION)[..]),
            )?;
        }

        // Nupkg files are just zip files, so unzip it.
        config.log.info("Unzipping binaries...")?;
        zip::unzip(&cake_nupkg_path, &cake_folder_path)?;
        config.log.info(&format!("Installed {} ({}).", flavor, &version))?;
    } else {
        config.log.info(&format!("{} ({}) is already installed.", flavor, &version))?;
    }

    return Ok(Option::Some(Cake {
        path: cake_folder_path.join(get_cake_filename(config)),
        version: Version::parse(&version).unwrap(),
        host: get_host(config),
    }));
}

pub fn should_install(config: &Config) -> bool {
    return config.cake_version != None;
}

fn get_cake_package_name(config: &Config, version: &String) -> String {
    let flavor = get_cake_flavor(config);
    return format!("{}.{}.nupkg", flavor.to_lowercase(), version);
}

fn get_cake_flavor(config: &Config) -> &str {
    if config.use_coreclr {
        return "Cake.CoreClr";
    } else {
        return "Cake";
    };
}

fn get_cake_filename(config: &Config) -> &str {
    if config.use_coreclr {
        return "Cake.dll";
    } else {
        return "Cake.exe";
    };
}

fn get_host(config: &Config) -> Host {
    if config.use_coreclr {
        return Host::CoreClr;
    } else if cfg!(unix) {
        return Host::Mono;
    } else {
        return Host::Clr;
    };
}
