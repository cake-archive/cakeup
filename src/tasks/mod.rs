// Licensed to the .NET Foundation under one or more agreements.
// The .NET Foundation licenses this file to you under the MIT license.
// See the LICENSE file in the project root for more information.

use std::io;
use super::config::Config;

mod prepare;
mod nuget;

pub trait Task {
    fn run(&self, config: &Config) -> Result<(), io::Error>;
}

pub fn get_tasks() -> Vec<Box<Task>> {
    let mut list : Vec<Box<Task>> = Vec::new();
    list.push(Box::new(prepare::PrepareTask { }));
    list.push(Box::new(nuget::NuGetTask { }));
    return list;
}