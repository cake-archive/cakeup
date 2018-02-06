extern crate curl;

use std::path::Path;
use self::curl::easy::Easy;
use std::io::*;
use std::fs;
use std::fs::File;

pub fn download(uri: &String, path: &Path) -> Result<()> {
    let mut handle = Easy::new();
    handle.follow_location(true)?; // Follow redirects.
    handle.url(uri)?; // Set the URL.

    // Download the file.
    let mut file = File::create(path)?;
    handle.write_function(move |data| {
        return Ok(file.write(data).unwrap());
    })?;
    handle.perform()?;

    // Check the response code.
    let response = handle.response_code()?;
    if response != 200 {
        fs::remove_file(path)?; // Delete the file.
        return Err(Error::new(
            ErrorKind::Other,
            format!("Expected status code 200 but received {}.", response),
        ));
    }

    return Ok(());
}