// Licensed to the .NET Foundation under one or more agreements.
// The .NET Foundation licenses this file to you under the MIT license.
// See the LICENSE file in the project root for more information.

use std::io::{Error, ErrorKind};
use std::path::PathBuf;
use std::process::{Command, ExitStatus};

#[derive(PartialEq)]
pub enum Host {
    Clr,
    CoreClr,
    Mono,
}

impl Host {
    pub fn verify(&self) -> Result<(), Error> {
        match self {
            Host::Clr => { },
            Host::CoreClr => {
                let output = Command::new("dotnet").arg("--version").output()?;
                if !output.status.success() {
                    return Err(Error::new(ErrorKind::Other, "Could not locate the .NET Core SDK."))
                }
            }
            Host::Mono => {
                let output = Command::new("mono").arg("--version").output()?;
                if !output.status.success() {
                    return Err(Error::new(ErrorKind::Other, "Could not locate the Mono runtime."))
                }
            }
        }
        return Ok(());
    }

    pub fn execute(&self, path: &PathBuf, args: &Vec<String>) -> Result<ExitStatus, Error> {
        let result: ExitStatus;
        match self {
            Host::Clr => {
                result = Command::new(path).args(args).status()?;
            }
            Host::CoreClr | Host::Mono => {
                let mut host = "dotnet";
                if *self == Host::Mono {
                    host = "mono";
                }
                result = Command::new(host).arg(path).args(args).status()?;
            }
        };
        return Ok(result);
    }

    pub fn get_name(&self) -> &str {
        match &self {
            &&Host::Clr => return "CLR",
            &&Host::CoreClr => return "dotnet",
            &&Host::Mono => return "mono",
        }
    }
}