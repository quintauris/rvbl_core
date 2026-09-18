// Copyright 2026 Quintauris GmbH
// Licensed under the Apache License, Version 2.0 (the "License").
// https://www.apache.org/licenses/LICENSE-2.0

use log::error;
use serde::de::DeserializeOwned;
use serde_json::Value as JsonValue;
use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    format, mem,
    path::PathBuf,
};

use crate::model;

#[derive(Debug)]
pub enum Error {
    IoError(std::io::Error),
    EncodingError(std::string::FromUtf8Error),
    JsonError(serde_json::Error),
    ValidationError(String),
    PathError(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::IoError(error) => error.fmt(f),
            Error::EncodingError(error) => error.fmt(f),
            Error::JsonError(error) => error.fmt(f),
            Error::ValidationError(error) => error.fmt(f),
            Error::PathError(error) => error.fmt(f),
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;

pub fn load(path: &PathBuf) -> Result<String> {
    let data = std::fs::read(path).map_err(|e| Error::IoError(e))?;

    Ok(String::from_utf8(data).map_err(|e| Error::EncodingError(e))?)
}

#[cfg(test)]
#[test]
fn test_load() -> Result<()> {
    let valid = load(&PathBuf::from("test/valid.json"))?;
    let not_found = load(&PathBuf::from("test/not_found.json"));

    assert_eq!(
        valid,
        "{\n    \"entry\": [\n        \"there's JSON here\"\n    ]\n}\n"
    );

    assert!(not_found.is_err());
    assert!(matches!(not_found.err().unwrap(), Error::IoError(_)));

    Ok(())
}

pub fn parse(text: &str) -> Result<JsonValue> {
    serde_json::from_str(text).map_err(|e| Error::JsonError(e))
}

#[cfg(test)]
#[test]
fn test_parse() {
    assert!(
        parse(r#"{ "foo": [] }"#).is_ok_and(|j| j.is_object() && j.as_object().unwrap().len() == 1)
    );
    assert!(parse(r#"{ "foo": [] "#).is_err_and(|e| match e {
        Error::JsonError(j) => j.is_eof(),
        _ => false,
    }));
    assert!(parse(r#"{ foo": [] }"#).is_err_and(|e| match e {
        Error::JsonError(j) => j.is_syntax(),
        _ => false,
    }));
}

pub fn preprocess<'a>(
    value: &'a mut JsonValue,
    search_path: &'a PathBuf,
    parameters: &'a HashMap<String, String>,
    dependencies: &'a mut HashSet<PathBuf>,
) -> Result<&'a mut JsonValue> {
    scan(value, parameters, search_path, dependencies)?;

    Ok(value)
}

pub fn validate<'a>(model: &'a JsonValue, meta_model_path: &std::path::PathBuf) -> Result<()> {
    let format_error = |e: jsonschema::ValidationError| {
        Error::ValidationError(format!("{}: {}", e.instance_path, e))
    };
    let meta_model_search_uri = format!(
        "file://{}/",
        meta_model_path
            .parent()
            .ok_or_else(|| Error::PathError(format!(
                "Meta-model path has no parent: {meta_model_path:?}"
            )))?
            .to_str()
            .ok_or_else(|| Error::PathError(format!(
                "Only UTF-8 paths are supported: {meta_model_path:?}"
            )))?
    );
    let meta_model = parse(&load(&meta_model_path)?)?;
    let validator = jsonschema::options()
        .with_base_uri(meta_model_search_uri)
        .build(&meta_model)
        .map_err(format_error)?;

    if validator.is_valid(&model) {
        Ok(())
    } else {
        Err(format_error(validator.iter_errors(&model).nth(0).unwrap()))
    }
}

fn deserialize<T>(model: JsonValue) -> Result<T>
where
    T: DeserializeOwned,
{
    serde_json::from_value(model).map_err(|e| Error::JsonError(e))
}

#[cfg(test)]
#[test]
fn test_preprocess() -> Result<()> {
    let source = load(&PathBuf::from("test/include_source.json"))?;
    let mut parsed = parse(&source.as_str())?;
    let mut dependencies: HashSet<PathBuf> = [].into();
    let mut parameters: HashMap<String, String> = [].into();
    let search_path = PathBuf::from("./test");
    let preprocessed = preprocess(
        &mut parsed,
        &search_path,
        &mut parameters,
        &mut dependencies,
    );

    assert!(
        preprocessed
            .as_ref()
            .is_ok_and(|j| j["data"]["value"].as_str().is_some_and(|s| s == "foo"))
    );
    assert!(preprocessed.as_ref().is_ok_and(|j| {
        j["data"]["array"]
            .as_array()
            .is_some_and(|a| a[0].as_str().is_some_and(|s| s == "bar"))
    }));

    Ok(())
}

fn resolve_path(path: &PathBuf, search_path: &PathBuf) -> Result<PathBuf> {
    if path.exists() {
        Ok(path.clone())
    } else if search_path.join(path).exists() {
        Ok(search_path.join(path))
    } else {
        Err(Error::IoError(std::io::Error::from(
            std::io::ErrorKind::NotFound,
        )))
    }
}

fn scan(
    value: &mut JsonValue,
    parameters: &HashMap<String, String>,
    search_path: &PathBuf,
    dependencies: &mut HashSet<PathBuf>,
) -> Result<()> {
    if value.is_object() {
        if value.as_object().unwrap().contains_key("include") && value["include"].is_string() {
            let path = PathBuf::from(value["include"].as_str().unwrap());
            let resolved_path = resolve_path(&path, search_path)?;
            let text = load(&resolved_path)?;
            let mut parameters = parameters.clone();

            dependencies.insert(path.clone());

            match parse(&text) {
                Ok(mut content) => {
                    for (k, v) in value.as_object().unwrap().iter() {
                        if v.is_string() && (k != "include") {
                            parameters.insert(k.to_string(), v.as_str().unwrap().to_string());
                        }
                    }

                    preprocess(&mut content, search_path, &parameters, dependencies)?;
                    mem::swap(&mut content, value);
                }
                Err(_) => error!(
                    "Unable to parse {}.",
                    resolved_path.as_os_str().to_str().unwrap()
                ),
            };
        } else {
            value
                .as_object_mut()
                .unwrap()
                .iter_mut()
                .for_each(|(_, value)| {
                    let _ = scan(value, parameters, search_path, dependencies);
                });
        }
    } else if value.is_array() {
        value.as_array_mut().unwrap().iter_mut().for_each(|item| {
            let _ = scan(item, parameters, search_path, dependencies);
        });
    } else if value.is_string() {
        let pattern = regex_static::static_regex!(r"\$[A-Z][A-Z0-9_]*");
        let text = value.as_str().unwrap();
        let values: Vec<&str> = pattern.find_iter(text).map(|m| m.as_str()).collect();

        if !values.is_empty() {
            let mut updated = String::from(text);

            for v in values {
                let key = v[1..].to_string();

                if parameters.contains_key(&key) {
                    updated = updated.replace(v, &parameters[&key]);
                }
            }

            mem::swap(value, &mut serde_json::json!(updated));
        }
    }

    Ok(())
}

fn locate_file(path: &PathBuf, search_path: &PathBuf) -> Result<PathBuf> {
    if path.is_relative() {
        let absolute = search_path.join(path);

        if absolute.exists() {
            Ok(absolute)
        } else {
            Err(Error::IoError(std::io::ErrorKind::NotFound.into()))
        }
    } else if path.exists() {
        Ok(path.clone())
    } else {
        Err(Error::IoError(std::io::ErrorKind::NotFound.into()))
    }
}

pub fn load_model<T>(
    input: &PathBuf,
    meta_model_search_path: &PathBuf,
    include_search_path: &PathBuf,
    parameters: &HashMap<String, String>,
    dependencies: &mut HashSet<PathBuf>,
) -> Result<T>
where
    T: DeserializeOwned + model::Resolve,
{
    let input_path = locate_file(input, include_search_path)?;
    let model_data = load(&input_path)?;
    let mut model = parse(&model_data)?;

    preprocess(&mut model, include_search_path, parameters, dependencies)?;

    let type_name = std::any::type_name::<T>().split("::").last().unwrap();
    let schema_file_name = format!("{}.schema.json", type_name.to_lowercase());
    let meta_model_path = meta_model_search_path.join(schema_file_name);

    validate(&model, &meta_model_path)?;

    match deserialize::<T>(model) {
        Ok(mut result) => match result.resolve(meta_model_search_path, include_search_path) {
            Ok(_) => Ok(result),
            Err(e) => Err(e),
        },
        Err(e) => Err(e),
    }
}
