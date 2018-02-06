// Licensed to the .NET Foundation under one or more agreements.
// The .NET Foundation licenses this file to you under the MIT license.
// See the LICENSE file in the project root for more information.

extern crate unzip;

use std::path::Path;
use std::io::*;
use std::fs;

pub fn unzip(input: &Path, output: &Path) -> Result<()> {
    let file = fs::File::open(&input)?;
    let unzipper = unzip::Unzipper::new(&file, &output);
    unzipper.unzip()?;
    return Ok(());
}