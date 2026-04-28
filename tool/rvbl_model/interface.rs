use std::{io::Result, path::PathBuf};

use crate::model;

#[allow(unused)]
pub trait Generator {
    fn generate(&self, machine: &model::Machine) -> Result<()>;
    fn byproducts(&self, machine: &model::Machine) -> Vec<PathBuf>;
}
