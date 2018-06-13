// Licensed to the .NET Foundation under one or more agreements.
// The .NET Foundation licenses this file to you under the MIT license.
// See the LICENSE file in the project root for more information.

extern crate curl;
extern crate serde_json;

use std::str;
use std::path::Path;
use self::curl::easy::Easy;
use std::io::{Write};
use std::fs;
use std::fs::File;
use super::CakeupResult;

#[derive(Serialize, Deserialize, Clone)]
pub struct GitHubRelease {
    pub url: String,
    pub name: String
}

pub fn get_latest_github_release<'a>(owner: &str, repo: &str) -> CakeupResult<GitHubRelease> {
    let url = format!("https://api.github.com/repos/{}/{}/releases", owner, repo);
    let json = get(&url, Some("cakeup"))?;
    let releases : Vec<GitHubRelease> = serde_json::from_str(&json)?;
    return Ok(releases.first().unwrap().clone());
}

pub fn get(uri: &String, user_agent: Option<&str>) -> CakeupResult<String> {
    let mut handle = Easy::new();
    handle.follow_location(true)?; // Follow redirects.
    handle.url(uri)?; // Set the URL.

    // Add user agent?
    if let Some(agent_name) = user_agent {
        let mut list = curl::easy::List::new();
        list.append(&format!("User-Agent: {}", agent_name)[..])?;
        handle.http_headers(list)?;
    }

    let mut content: String = String::new();
    {
        let mut transfer = handle.transfer();
        transfer.write_function(|data| {
            content.push_str(str::from_utf8(data).unwrap());
            Ok(data.len())
        })?;

        transfer.perform()?;
    }

    return Ok(content);
}

pub fn download(uri: &String, path: &Path, user_agent: Option<&str>) -> CakeupResult<()> {
    let mut handle = Easy::new();
    handle.follow_location(true)?; // Follow redirects.
    handle.url(uri)?; // Set the URL.

    // Add user agent?
    match user_agent {
        None => { },
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
        return Err(format_err!("Expected status code 200 but received {}.", response));
    }

    return Ok(());
}