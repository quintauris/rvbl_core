use serde::{Deserialize, Serialize};
use serde_json;
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct Configuration {
    pub last_toolchains: Vec<String>,
    pub last_machines: Vec<String>,
    pub last_runners: Vec<String>,
}

pub fn write(path: &PathBuf, configuration: &Configuration) {
    if let Ok(text) = serde_json::to_string(configuration) {
        let _ = fs::write(path, text);
    }
}

pub fn read(path: &PathBuf) -> Configuration {
    if let Ok(data) = fs::read(path) {
        if let Ok(text) = String::from_utf8(data) {
            if let Ok(result) = serde_json::from_str::<Configuration>(text.as_str()) {
                return result;
            }
        }
    }

    Configuration {
        last_toolchains: vec![],
        last_machines: vec![],
        last_runners: vec![],
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Parameters {
    pub default: Vec<String>,
    pub debug: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Runner {
    pub programs: Vec<String>,
    pub parameters: Parameters,
}

pub fn runner(path: &PathBuf) -> Option<Runner> {
    if let Ok(data) = fs::read(path) {
        if let Ok(text) = String::from_utf8(data) {
            if let Ok(result) = serde_json::from_str::<Runner>(text.as_str()) {
                return Some(result);
            }
        }
    }

    None
}
