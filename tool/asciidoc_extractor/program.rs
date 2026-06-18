// Copyright 2026 Quintauris GmbH
// Licensed under the Apache License, Version 2.0 (the "License").
// https://www.apache.org/licenses/LICENSE-2.0

use log::{error, info, warn};
use std::{
    fs::{File, read_to_string},
    io::prelude::*,
    process::ExitCode,
};

fn main() -> ExitCode {
    let parameters: Vec<String> = std::env::args().collect();

    colog::init();

    if parameters.len() >= 3 {
        let output_path = parameters.last().unwrap();
        let mut errors = 0;

        match File::create(output_path) {
            Ok(mut target) => {
                for input_path in parameters[1..parameters.len() - 1].iter() {
                    match read_to_string(&input_path) {
                        Ok(source) => {
                            let mut block_length: usize = 0;

                            for line in source.lines() {
                                let trimmed = line.trim();
                                let status = if trimmed.starts_with("///") {
                                    block_length += 1;
                                    target
                                        .write_fmt(format_args!("{}\n", &trimmed[3..].trim_start()))
                                } else if block_length != 0 {
                                    block_length = 0;
                                    target.write_all("\n".as_bytes())
                                } else {
                                    Ok(())
                                };

                                if let Err(error) = status {
                                    warn!("Error writing to {output_path}: {error}");
                                    errors = errors + 1;
                                }
                            }
                        }
                        Err(error) => {
                            warn!("Unable to open input file {input_path}: {error}");
                            errors = errors + 1;
                        }
                    }
                }
            }
            Err(error) => {
                error!("Unable to open output file {output_path}: {error}");
                errors = errors + 1;
            }
        }

        ExitCode::from(errors)
    } else {
        info!("Usage: <input> ... <input> <output>");
        ExitCode::from(1)
    }
}
