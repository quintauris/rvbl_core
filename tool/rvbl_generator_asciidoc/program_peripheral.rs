// Copyright 2026 Quintauris GmbH
// Licensed under the Apache License, Version 2.0 (the "License").
// https://www.apache.org/licenses/LICENSE-2.0

mod generator;
mod templates;

use clap::Parser;
use log::error;
use rvbl_model::{
    argument, loader,
    model::{Machine, Peripheral},
};
use std::{collections::HashSet, path::PathBuf, process::ExitCode, vec};

fn main() -> ExitCode {
    let arguments = argument::Arguments::parse();
    let mut dependencies: HashSet<PathBuf> = [].into();
    let mut parameters = argument::parse_parameters(&arguments);

    colog::init();

    match loader::load_model::<Peripheral>(
        &arguments.input,
        &arguments.meta_model,
        &arguments.search_path,
        &mut parameters,
        &mut dependencies,
    ) {
        Ok(peripheral) => {
            let generator = generator::AsciiDocGenerator::default();
            let machine = Machine {
                name: String::new(),
                description: None,
                word_size: 32,
                memory_map: vec![],
                peripherals: vec![],
                parameters: vec![],
            };

            match generator.generate_peripheral(&peripheral, &machine) {
                Ok(_) => ExitCode::from(0),
                Err(error) => {
                    error!("Generation failed: {error}");
                    ExitCode::from(1)
                }
            }
        }
        Err(error) => {
            error!("Unable to load {:?}: {error}", arguments.input);
            ExitCode::from(1)
        }
    }
}
