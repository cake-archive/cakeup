// Licensed to the .NET Foundation under one or more agreements.
// The .NET Foundation licenses this file to you under the MIT license.
// See the LICENSE file in the project root for more information.

extern crate curl;

use self::curl::easy::Easy;
use super::CakeupResult;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::str;

pub fn download(uri: &String, path: &Path, user_agent: Option<&str>) -> CakeupResult<()> {
    let mut handle = Easy::new();
    handle.follow_location(true)?; // Follow redirects.
    handle.url(uri)?; // Set the URL.

    // Add user agent?
    match user_agent {
        None => {}
        Some(ref agent_name) => {
            let mut list = curl::easy::List::new();
            list.append(&format!("User-Agent: {}", agent_name)[..])?;
            handle.http_headers(list)?;
        }
    };

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
        return Err(format_err!(
            "Expected status code 200 but received {}.",
            response
        ));
    }

    return Ok(());
}
