use std::path::PathBuf;
use walkdir::DirEntry;

pub fn layers_root() -> Option<PathBuf> {
    let mut path = std::env::current_exe()
        .expect("Can't determine executable path")
        .canonicalize()
        .expect("Can't canonicalize executable path");

    while path.pop() {
        path.push("base");

        if path.exists() {
            path.pop();

            return Some(path);
        } else {
            path.pop();
        }
    }

    return None;
}

pub fn configuration_file(root: &PathBuf) -> PathBuf {
    let mut result = root.clone();

    result.push(".config.json");
    result
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
