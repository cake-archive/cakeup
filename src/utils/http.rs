// Licensed to the .NET Foundation under one or more agreements.
// The .NET Foundation licenses this file to you under the MIT license.
// See the LICENSE file in the project root for more information.

extern crate curl;

use std::str;
use std::path::Path;
use self::curl::easy::Easy;
use std::io::*;
use std::fs;
use std::fs::File;

pub fn get(uri: &String, user_agent: Option<&str>) -> String {
    let mut handle = Easy::new();
    handle.follow_location(true).unwrap(); // Follow redirects.
    handle.url(uri).unwrap(); // Set the URL.

    // Add user agent?
    let user_agent = Option::from(user_agent).unwrap_or("");
    if !user_agent.is_empty() {
        let mut list = curl::easy::List::new();
        list.append(&format!("User-Agent: {}", user_agent)[..]).unwrap();
        handle.http_headers(list).unwrap();
    }

    let mut content: String = String::new();
    {
        let mut transfer = handle.transfer();
        transfer
            .write_function(|data| {
                content.push_str(str::from_utf8(data).unwrap());
                Ok(data.len())
            })
            .unwrap();

        transfer.perform().unwrap();
    }

    return content;
}

pub fn download(uri: &String, path: &Path, user_agent: Option<&str>) -> Result<()> {
    let mut handle = Easy::new();
    handle.follow_location(true)?; // Follow redirects.
    handle.url(uri)?; // Set the URL.

    // Add user agent?
    let user_agent = Option::from(user_agent).unwrap_or("");
    if !user_agent.is_empty() {
        let mut list = curl::easy::List::new();
        list.append(&format!("User-Agent: {}", user_agent)[..]).unwrap();
        handle.http_headers(list).unwrap();
    }

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