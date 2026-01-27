use clap::{Args, Parser};

#[derive(Args)]
#[command(version, about, long_about = None)]
pub struct ListArgs {
    #[arg(short, long)]
    pub machines: bool,
    #[arg(short, long)]
    pub toolchains: bool,
}

#[derive(Args)]
#[command(version, about, long_about = None)]
pub struct MakeArgs {
    #[arg(short, long, default_value_t = String::from("last"))]
    pub toolchain: String,
    #[arg(short, long, default_value_t = String::from("last"))]
    pub machine: String,
    #[arg(short, long, default_value_t = 1)]
    pub jobs: u32,
    #[arg(short, long)]
    pub configure: bool,
    #[arg(short, long)]
    pub build: bool,
    #[arg(short, long)]
    pub test: bool,
    #[arg(short, long)]
    pub docker: bool,
    #[arg(long, default_value_t = String::from("Release"))]
    pub build_type: String,
}

#[derive(Args)]
#[command(version, about, long_about = None)]
pub struct DockerArgs {
    #[arg(short, long, default_value_t = String::from("gnu.gcc"))]
    pub toolchain: String,
    #[arg(short, long)]
    pub provision: bool,
    #[arg(short, long)]
    pub up: bool,
    #[arg(short, long)]
    pub down: bool,
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub enum Arguments {
    List(ListArgs),
    Make(MakeArgs),
    Docker(DockerArgs),
}
