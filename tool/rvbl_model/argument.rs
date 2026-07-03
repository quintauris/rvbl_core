// Copyright 2026 Quintauris GmbH
// Licensed under the Apache License, Version 2.0 (the "License").
// https://www.apache.org/licenses/LICENSE-2.0

use std::{collections::HashMap, path::PathBuf};

use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Arguments {
    /// Path to input model
    #[arg(short, long)]
    pub input: PathBuf,

    /// Path to meta model
    #[arg(short, long)]
    pub meta_model: PathBuf,

    /// Search path for preprocessor
    #[arg(short, long)]
    pub search_path: PathBuf,

    /// Don't generate any code, just output byproduct paths
    #[arg(short, long)]
    pub dry_run: bool,

    /// Model parameter in NAME=VALUE form, can be used multiple times
    #[arg(short, long)]
    pub parameter: Vec<String>,
}

pub fn parse_parameters(arguments: &Arguments) -> HashMap<String, String> {
    let parameter_parser = |s: &String| {
        let mut i = s.splitn(2, "=");

        (
            i.next().unwrap().to_string(),
            i.next().unwrap_or("").to_string(),
        )
    };

    arguments.parameter.iter().map(parameter_parser).collect()
}
