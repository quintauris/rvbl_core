// Copyright 2026 Quintauris GmbH
// Licensed under the Apache License, Version 2.0 (the "License").
// https://www.apache.org/licenses/LICENSE-2.0

use std::{
    fs::File,
    io::{BufWriter, Result},
    path::PathBuf,
};

use super::{
    templates::MemoryLayoutTemplate,
    templates::{Gnu, Iar, Tasking},
};
use rvbl_model::{interface::Generator, model};

pub struct LinkerGenerator<'a> {
    toolchains: [(&'a str, &'a dyn MemoryLayoutTemplate); 3],
}

impl Default for LinkerGenerator<'_> {
    fn default() -> Self {
        Self {
            toolchains: [("GNU", &Gnu {}), ("Tasking", &Tasking {}), ("IAR", &Iar {})],
        }
    }
}

impl Generator for LinkerGenerator<'_> {
    fn generate(&self, machine: &model::Machine) -> Result<()> {
        for toolchain in self.toolchains {
            let file = File::create(format!("memory.{}.ld", toolchain.0)).unwrap();
            let mut writer = BufWriter::new(file);

            toolchain.1.open(&mut writer)?;
            for addressable in machine
                .memory_map
                .iter()
                .filter_map(|r| r.dereference().ok())
            {
                toolchain.1.entry(&mut writer, addressable)?;
            }
            toolchain.1.close(&mut writer)?;
        }

        Ok(())
    }

    fn byproducts(&self, _machine: &model::Machine) -> Vec<PathBuf> {
        return self
            .toolchains
            .map(|t| PathBuf::from(format!("memory.{}.ld", t.0)))
            .into_iter()
            .collect::<Vec<PathBuf>>();
    }
}
