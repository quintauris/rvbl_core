// Copyright 2026 Quintauris GmbH
// Licensed under the Apache License, Version 2.0 (the "License").
// https://www.apache.org/licenses/LICENSE-2.0

use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::fmt::Display;
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Configuration {
    pub last_toolchains: Vec<String>,
    pub last_toolchain_target: Option<String>,
    pub last_targets: Vec<String>,
    pub last_runners: Vec<String>,
    pub last_build_generator: Option<String>,
}

pub enum Error {
    JsonError(serde_json::Error),
    IoError(std::io::Error),
    StringError(std::string::FromUtf8Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::JsonError(error) => error.fmt(f),
            Error::IoError(error) => error.fmt(f),
            Error::StringError(error) => error.fmt(f),
        }
    }
}

type Result<T> = std::result::Result<T, Error>;

fn deserialize<T>(path: &PathBuf) -> Result<T>
where
    T: serde::de::DeserializeOwned,
{
    let data = fs::read(path).map_err(|e| Error::IoError(e))?;
    let text = String::from_utf8(data).map_err(|e| Error::StringError(e))?;

    serde_json::from_str::<T>(text.as_str()).map_err(|e| Error::JsonError(e))
}

fn serialize<T>(path: &PathBuf, object: &T) -> Result<()>
where
    T: serde::Serialize,
{
    let text = serde_json::to_string(object).map_err(|e| Error::JsonError(e))?;

    fs::write(path, text).map_err(|e| Error::IoError(e))
}

pub fn write(path: &PathBuf, configuration: &Configuration) -> Result<()> {
    serialize(path, configuration)
}

pub fn read(path: &PathBuf) -> Result<Configuration> {
    deserialize(path)
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Parameters {
    pub append: Option<bool>,
    pub default: Vec<String>,
    pub debug: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Docker {
    pub container: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Runner {
    pub programs: Vec<String>,
    pub parameters: Parameters,
    pub docker: Option<Docker>,
    pub prepend: Option<bool>,
}

pub fn runner(path: &PathBuf) -> Result<Runner> {
    deserialize(path)
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Target {
    pub runners: HashMap<String, Parameters>,
    pub default_toolchain_target: Option<String>,
}

pub fn target(path: &PathBuf) -> Result<Target> {
    deserialize(path)
}
