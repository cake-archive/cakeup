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