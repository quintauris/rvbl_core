// Copyright 2026 Quintauris GmbH
// Licensed under the Apache License, Version 2.0 (the "License").
// https://www.apache.org/licenses/LICENSE-2.0

mod argument;
mod evaluator;
mod interface;
mod loader;
mod model;

use argument::Arguments;
use clap::Parser;
use log::error;
use model::Machine;
use std::{collections::HashSet, path::PathBuf, process::ExitCode};

fn main() -> ExitCode {
    let arguments = Arguments::parse();
    let mut dependencies: HashSet<PathBuf> = [].into();
    let mut parameters = argument::parse_parameters(&arguments);
    let canonical_meta_model_path = arguments.meta_model.canonicalize().unwrap_or_default();
    let canonical_search_path = arguments.search_path.canonicalize().unwrap_or_default();

    colog::init();

    match loader::load_model::<Machine>(
        &arguments.input,
        &canonical_meta_model_path,
        &canonical_search_path,
        &mut parameters,
        &mut dependencies,
    ) {
        Ok(machine) => {
            if arguments.dry_run {
                println!(
                    "{}",
                    dependencies
                        .iter()
                        .map(|x| x.to_string_lossy().to_string())
                        .collect::<Vec<String>>()
                        .join(";")
                );
            } else {
                println!("{:#?}", machine);
            }

            ExitCode::from(0)
        }
        Err(error) => {
            error!("Unable to load {:?}: {error}", arguments.input);
            ExitCode::from(1)
        }
    }
}
