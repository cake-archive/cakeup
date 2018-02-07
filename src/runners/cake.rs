// Licensed to the .NET Foundation under one or more agreements.
// The .NET Foundation licenses this file to you under the MIT license.
// See the LICENSE file in the project root for more information.

use std::io::{Error};
use config::*;
use utils::*;

pub fn install(config: &Config) -> Result<(), Error> {

    // Get the version we're going to use.
    let mut version = config.cake_version.clone();
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
    let cake_folder_path = config.tools.join(format!("{0}.{1}", flavor.to_lowercase(), version));
    if !cake_folder_path.exists() {

        let cake_nupkg_path = config.tools.join(get_cake_package_name(&config, &version));
        if !cake_nupkg_path.exists() {
            println!("Downloading {0} {1}...", flavor, version);
            let url = &format!("https://www.nuget.org/api/v2/package/{0}/{1}", flavor, version);
            http::download(&url, &cake_nupkg_path, 
                Some(&format!("Cakeup NuGet Client/{0}", ::utils::version::VERSION)[..]))?;
        }

        // Nupkg files are just zip files, so unzip it.
        println!("Unzipping binaries...");
        zip::unzip(&cake_nupkg_path, &cake_folder_path)?;
    }

    return Ok(());
}

fn get_cake_package_name(config: &Config, version: &String) -> String {
    if config.use_coreclr {
        return format!("cake.coreclr.{}.nupkg", version);
    }
    return format!("cake.{}.nupkg", version);
}