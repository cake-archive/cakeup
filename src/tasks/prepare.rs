// Licensed to the .NET Foundation under one or more agreements.
// The .NET Foundation licenses this file to you under the MIT license.
// See the LICENSE file in the project root for more information.

use std::io;
use std::fs;
use config::Config;
use tasks::Task;

pub struct PrepareTask { }
impl Task for PrepareTask {
    fn run(&self, config: &Config) -> Result<(), io::Error> {
        
        let tool_path = config.root.join("tools");

        if !tool_path.exists() {
            println!("Creating tools directory...");
            fs::create_dir(&tool_path.to_str().unwrap())?;
        }

        return Ok(());
    }
}