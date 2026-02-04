use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Configuration {
    pub last_toolchains: Vec<String>,
    pub last_targets: Vec<String>,
    pub last_runners: Vec<String>,
}

fn deserialize<T>(path: &PathBuf) -> Option<T>
where
    T: serde::de::DeserializeOwned,
{
    if let Ok(data) = fs::read(path) {
        if let Ok(text) = String::from_utf8(data) {
            if let Ok(result) = serde_json::from_str::<T>(text.as_str()) {
                return Some(result);
            }
        }
    }

    None
}

fn serialize<T>(path: &PathBuf, object: &T)
where
    T: serde::Serialize,
{
    if let Ok(text) = serde_json::to_string(object) {
        let _ = fs::write(path, text);
    }
}

pub fn write(path: &PathBuf, configuration: &Configuration) {
    serialize(path, configuration);
}

pub fn read(path: &PathBuf) -> Option<Configuration> {
    deserialize(path)
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Parameters {
    pub append: Option<bool>,
    pub default: Vec<String>,
    pub debug: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Runner {
    pub programs: Vec<String>,
    pub parameters: Parameters,
}

pub fn runner(path: &PathBuf) -> Option<Runner> {
    deserialize(path)
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Target {
    pub runners: HashMap<String, Parameters>,
}

pub fn target(path: &PathBuf) -> Option<Target> {
    deserialize(path)
}
