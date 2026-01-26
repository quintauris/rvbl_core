use std::fmt;
use std::path::PathBuf;

pub struct ElementPath {
    pub path: PathBuf,
}

impl ElementPath {
    pub fn vendor(&self) -> String {
        self.path
            .iter()
            .nth_back(2)
            .unwrap()
            .to_str()
            .unwrap()
            .to_string()
    }

    pub fn name(&self) -> String {
        self.path
            .iter()
            .nth_back(0)
            .unwrap()
            .to_str()
            .unwrap()
            .to_string()
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

pub struct Machine {
    pub element: ElementPath,
}

impl Machine {
    pub fn build_path(
        &self,
        root: &PathBuf,
        machine: &Machine,
        toolchain: &Toolchain,
        docker: bool,
    ) -> PathBuf {
        let mut result = root.clone();

        result.push("build");
        result.push(if docker { "docker" } else { "local" });
        result.push(toolchain.element.slug());
        result.push(machine.element.slug());

        result
    }
}

impl fmt::Display for Machine {
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
        let mut result = root.clone();

        result.push("build");
        result.push(if docker { "docker" } else { "local" });

        if docker {
            result.push(self.element.slug());
        }

        result.push("tool");

        result
    }

    pub fn tools_path(&self, root: &PathBuf, docker: bool) -> PathBuf {
        let mut result = self.tools_build_path(root, docker);

        result.push("release");

        result
    }
}

impl fmt::Display for Toolchain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.element.fmt(f)
    }
}

pub enum Element {
    Toolchain(Toolchain),
    Machine(Machine),
    Unknown,
}

pub trait Moniker {
    fn moniker(&self) -> String;
}

impl Moniker for Element {
    fn moniker(&self) -> String {
        match self {
            Element::Toolchain(t) => t.element.moniker(),
            Element::Machine(m) => m.element.moniker(),
            _ => String::from(""),
        }
    }
}

impl Element {
    pub fn from(path: &PathBuf) -> Element {
        if path.iter().count() >= 3 {
            match path.iter().nth_back(1).unwrap().to_str().unwrap() {
                "machine" => Element::Machine(Machine {
                    element: ElementPath { path: path.clone() },
                }),
                "toolchain" => Element::Toolchain(Toolchain {
                    element: ElementPath { path: path.clone() },
                }),
                _ => Element::Unknown,
            }
        } else {
            Element::Unknown
        }
    }
}
