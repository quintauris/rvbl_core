// Copyright 2026 Quintauris GmbH
// Licensed under the Apache License, Version 2.0 (the "License").
// https://www.apache.org/licenses/LICENSE-2.0

use std::path::PathBuf;
use walkdir::DirEntry;

pub fn layers_root() -> std::io::Result<PathBuf> {
    let is_layers_root = |p: &PathBuf| p.join("core").exists();
    let not_found = std::io::Error::from(std::io::ErrorKind::NotFound);

    if let Ok(rvbl_root) = std::env::var("RVBL_ROOT") {
        let path = PathBuf::from(rvbl_root);

        if is_layers_root(&path) {
            Ok(path)
        } else {
            Err(not_found)
        }
    } else {
        let mut path = std::env::current_exe()?.canonicalize()?;

        while path.pop() {
            if is_layers_root(&path) {
                return Ok(path);
            }
        }

        Err(not_found)
    }
}

pub fn configuration_file(root: &PathBuf) -> PathBuf {
    root.join(".config.json")
}

pub fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

pub fn is_directory(entry: &DirEntry) -> bool {
    entry.file_type().is_dir()
}

pub fn is_build(entry: &DirEntry) -> bool {
    entry
        .path()
        .iter()
        .rfind(|c| c.to_str().unwrap() == "build")
        .is_some()
}

pub fn is_toolchain_target(entry: &std::fs::DirEntry) -> bool {
    !entry.path().is_dir()
        && entry.path().extension().unwrap_or_default() == "cmake"
        && entry.file_name().to_str().unwrap_or_default().contains("_")
}
