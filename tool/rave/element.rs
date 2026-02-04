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
        let mut result = root.clone();

        result.push("build");
        result.push(if docker { "docker" } else { "local" });
        result.push(toolchain.element.slug());
        result.push(target.element.slug());

        result
    }

    pub fn configuration(&self) -> PathBuf {
        let mut path = self.element.path.clone();

        path.push("machine.json");

        path
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

pub struct Runner {
    pub element: ElementPath,
}

impl Runner {
    pub fn configuration(&self) -> PathBuf {
        let mut path = self.element.path.clone();

        path.push("runner.json");

        path
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
            _ => String::from(""),
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
        if path.iter().count() >= 3 {
            match path.iter().nth_back(1).unwrap().to_str().unwrap() {
                "machine" | "project" => Element::Target(Target {
                    element: ElementPath { path: path.clone() },
                }),
                "toolchain" => Element::Toolchain(Toolchain {
                    element: ElementPath { path: path.clone() },
                }),
                "runner" => Element::Runner(Runner {
                    element: ElementPath { path: path.clone() },
                }),
                _ => Element::Unknown,
            }
        } else {
            Element::Unknown
        }
    }
}
