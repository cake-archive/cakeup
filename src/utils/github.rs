// Licensed to the .NET Foundation under one or more agreements.
// The .NET Foundation licenses this file to you under the MIT license.
// See the LICENSE file in the project root for more information.

extern crate serde_json;

use std::io::Error;
use super::http;

#[derive(Serialize, Deserialize, Clone)]
pub struct GitHubRelease {
    pub url: String,
    pub name: String
}

pub fn get_latest_release<'a>(owner: &str, repo: &str) -> Result<GitHubRelease, Error> {
    let url = format!("https://api.github.com/repos/{}/{}/releases", owner, repo);
    let json = http::get(&url, Some("cakeup"))?;
    let releases : Vec<GitHubRelease> = serde_json::from_str(&json)?;
    return Ok(releases.first().unwrap().clone());
}