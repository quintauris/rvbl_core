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
pub struct ConfigureArgs {
    #[arg(short, long, default_value_t = String::from("gnu.gcc"))]
    pub toolchain: String,
    #[arg(short, long, default_value_t = String::from("qemu.virt"))]
    pub machine: String,
    #[arg(short, long, default_value_t = String::from("Release"))]
    pub build_type: String,
    #[arg(short, long)]
    pub docker: bool,
}

#[derive(Args)]
#[command(version, about, long_about = None)]
pub struct BuildArgs {
    #[arg(short, long, default_value_t = String::from("last"))]
    pub toolchain: String,
    #[arg(short, long, default_value_t = String::from("last"))]
    pub machine: String,
    #[arg(short, long, default_value_t = 1)]
    pub jobs: u32,
    #[arg(short, long)]
    pub docker: bool,
}

#[derive(Args)]
#[command(version, about, long_about = None)]
pub struct TestArgs {
    #[arg(short, long, default_value_t = String::from("last"))]
    pub toolchain: String,
    #[arg(short, long, default_value_t = String::from("last"))]
    pub machine: String,
    #[arg(short, long)]
    pub docker: bool,
}

#[derive(Args)]
#[command(version, about, long_about = None)]
pub struct ProvisionArgs {
    #[arg(short, long, default_value_t = String::from("gnu.gcc"))]
    pub toolchain: String,
}

pub type UpArgs = ProvisionArgs;
pub type DownArgs = ProvisionArgs;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub enum Arguments {
    List(ListArgs),
    Configure(ConfigureArgs),
    Build(BuildArgs),
    Test(TestArgs),
    Provision(ProvisionArgs),
    Up(UpArgs),
    Down(DownArgs),
}
