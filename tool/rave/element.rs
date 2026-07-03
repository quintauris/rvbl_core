// Copyright 2026 Quintauris GmbH
// Licensed under the Apache License, Version 2.0 (the "License").
// https://www.apache.org/licenses/LICENSE-2.0

use crate::path;
use std::path::PathBuf;
use std::{fmt, fs};

pub struct ElementPath {
    path: PathBuf,
}

impl ElementPath {
    pub fn new(path: PathBuf) -> Option<Self> {
        match path.to_str() {
            Some(_) => {
                if path.iter().count() >= 3 {
                    Some(Self { path })
                } else {
                    None
                }
            }
            None => None,
        }
    }

    pub fn vendor(&self) -> &str {
        self.path.iter().nth_back(2).unwrap().to_str().unwrap()
    }

    pub fn kind(&self) -> &str {
        self.path.iter().nth_back(1).unwrap().to_str().unwrap()
    }

    pub fn name(&self) -> &str {
        self.path.iter().nth_back(0).unwrap().to_str().unwrap()
    }

    pub fn slug(&self) -> String {
        format!("{}_{}", self.vendor(), self.name())
    }

    pub fn moniker(&self) -> String {
        format!("{}.{}", self.vendor(), self.name())
    }
}

impl fmt::Display for ElementPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.moniker().as_str())
    }
}

pub struct Target {
    pub element: ElementPath,
}

impl Target {
    pub fn build_path(
        &self,
        root: &PathBuf,
        target: &Target,
        toolchain: &Toolchain,
        docker: bool,
    ) -> PathBuf {
        root.join("build")
            .join(if docker { "docker" } else { "local" })
            .join(toolchain.element.slug())
            .join(target.element.slug())
    }

    pub fn configuration(&self) -> PathBuf {
        self.element.path.join("machine.json")
    }

    pub fn path(&self) -> PathBuf {
        self.element.path.clone()
    }
}

impl fmt::Display for Target {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.element.fmt(f)
    }
}

pub struct Toolchain {
    pub element: ElementPath,
}

impl Toolchain {
    pub fn container(&self) -> String {
        format!("rvbl.{}", self.element.slug())
    }

    pub fn image(&self) -> String {
        format!("rvbl:{}", self.element.moniker())
    }

    pub fn tools_build_path(&self, root: &PathBuf, docker: bool) -> PathBuf {
        root.join("build")
            .join(if docker {
                PathBuf::from("docker").join(self.element.slug())
            } else {
                PathBuf::from("local")
            })
            .join("tool")
    }

    pub fn tools_path(&self, root: &PathBuf, docker: bool) -> PathBuf {
        self.tools_build_path(root, docker).join("release")
    }

    pub fn path(&self) -> PathBuf {
        self.element.path.clone()
    }

    pub fn targets(&self) -> Vec<String> {
        let mut result: Vec<String> = vec![];

        if let Ok(paths) = fs::read_dir(self.path()) {
            for path in paths.flatten().filter(path::is_toolchain_target) {
                result.push(
                    path.path()
                        .file_stem()
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .to_string(),
                );
            }
        }

        result
    }
}

impl fmt::Display for Toolchain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.element.fmt(f)
    }
}

pub struct Runner {
    pub element: ElementPath,
}

impl Runner {
    pub fn configuration(&self) -> PathBuf {
        self.element.path.join("runner.json")
    }
}

impl fmt::Display for Runner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.element.fmt(f)
    }
}

pub enum Element {
    Toolchain(Toolchain),
    Target(Target),
    Runner(Runner),
    Unknown,
}

pub trait Moniker {
    fn moniker(&self) -> String;
}

impl Moniker for Element {
    fn moniker(&self) -> String {
        match self {
            Element::Toolchain(t) => t.element.moniker(),
            Element::Target(m) => m.element.moniker(),
            Element::Runner(r) => r.element.moniker(),
            _ => String::from("Unknown"),
        }
    }
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.moniker().as_str())
    }
}

impl Element {
    pub fn from(path: &PathBuf) -> Element {
        match ElementPath::new(path.clone()) {
            Some(element) => match element.kind() {
                "machine" | "project" => Element::Target(Target { element }),
                "toolchain" => Element::Toolchain(Toolchain { element }),
                "runner" => Element::Runner(Runner { element }),
                _ => Element::Unknown,
            },
            None => Element::Unknown,
        }
    }
}
