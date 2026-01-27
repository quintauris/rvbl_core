use clap::{Args, Parser};

#[derive(Args)]
#[command(version, about, long_about = None)]
pub struct ListArgs {
    /// List all machines
    #[arg(short, long, alias = "machine")]
    pub machines: bool,

    /// List all toolchains
    #[arg(short, long, alias = "toolchain")]
    pub toolchains: bool,
}

#[derive(Args)]
#[command(version, about, long_about = None)]
pub struct MakeArgs {
    /// Toolchain(s) in <layer>.<name> form (e.g. gnu.gcc), comma-sepparated
    #[arg(short, long, default_value_t = String::from("last"))]
    pub toolchain: String,

    /// Machine(s) in <layer>.<name> form (e.g. qemu.virt), comma-separated
    #[arg(short, long, default_value_t = String::from("last"))]
    pub machine: String,

    /// Configure build for selected machine(s)
    #[arg(short, long)]
    pub configure: bool,

    /// Build type to configure (e.g. Release)
    #[arg(long, default_value_t = String::from("Release"), requires = "configure")]
    pub build_type: String,

    /// Perform build for selected machine(s)
    #[arg(short, long)]
    pub build: bool,

    /// Number of parallel jobs to use during build
    #[arg(short, long, default_value_t = 1, requires = "build")]
    pub jobs: u32,

    /// Run tests for selected machine(s)
    #[arg(short, long)]
    pub test: bool,

    /// Run selected actions in Docker container(s)
    #[arg(short, long)]
    pub docker: bool,
}

#[derive(Args)]
#[command(version, about, long_about = None)]
pub struct DockerArgs {
    /// Toolchain(s) in <layer>.<name> form (e.g. gnu.gcc), comma-sepparated
    #[arg(short, long, default_value_t = String::from("last"))]
    pub toolchain: String,

    /// Build Docker image for selected toolchain(s)
    #[arg(short, long)]
    pub provision: bool,

    /// Spin-up Docker image for selected toolchain(s)
    #[arg(short, long)]
    pub up: bool,

    /// Wind-down Docker image for selected toolchain(s)
    #[arg(short, long)]
    pub down: bool,
}

/// RISC-V Base Layer build tool
#[derive(Parser)]
#[command(version, about, long_about = None)]
pub enum Arguments {
    /// List available elements of different kinds
    List(ListArgs),
    /// Configure, build and test machines
    Make(MakeArgs),
    /// Manage Docker images and containers for toolchains
    Docker(DockerArgs),
}
