// Copyright 2026 Quintauris GmbH
// Licensed under the Apache License, Version 2.0 (the "License").
// https://www.apache.org/licenses/LICENSE-2.0

use std::{io::Result, path::PathBuf};

use crate::model;

#[allow(unused)]
pub trait Generator {
    fn generate(&self, machine: &model::Machine) -> Result<()>;
    fn byproducts(&self, machine: &model::Machine) -> Vec<PathBuf>;
}
