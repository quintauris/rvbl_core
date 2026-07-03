// Copyright 2026 Quintauris GmbH
// Licensed under the Apache License, Version 2.0 (the "License").
// https://www.apache.org/licenses/LICENSE-2.0

use clap::Parser;
use log::error;
use rvbl_model::{argument, interface::Generator, loader, model::Machine};
use std::{collections::HashSet, path::PathBuf, process::ExitCode};

mod generator;
mod templates;

fn main() -> ExitCode {
    let arguments = argument::Arguments::parse();
    let mut dependencies: HashSet<PathBuf> = [].into();
    let mut parameters = argument::parse_parameters(&arguments);

    colog::init();

    match loader::load_model::<Machine>(
        &arguments.input,
        &arguments.meta_model,
        &arguments.search_path,
        &mut parameters,
        &mut dependencies,
    ) {
        Ok(machine) => {
            let generator = generator::CGenerator {};

            if arguments.dry_run {
                println!(
                    "{}",
                    generator
                        .byproducts(&machine)
                        .iter()
                        .map(|x| x.to_str().unwrap())
                        .collect::<Vec<&str>>()
                        .join(";")
                );

                ExitCode::from(0)
            } else {
                match generator.generate(&machine) {
                    Ok(_) => ExitCode::from(0),
                    Err(error) => {
                        error!("Generation failed: {error}");
                        ExitCode::from(1)
                    }
                }
            }
        }
        Err(error) => {
            error!("Unable to load {:?}: {error}", arguments.input);
            ExitCode::from(1)
        }
    }
}
