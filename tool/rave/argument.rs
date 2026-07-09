// Copyright 2026 Quintauris GmbH
// Licensed under the Apache License, Version 2.0 (the "License").
// https://www.apache.org/licenses/LICENSE-2.0

use std::path::PathBuf;

use clap::{Args, Parser};

#[derive(Args)]
#[command(version, about, long_about = None)]
pub struct ListArgs {
    /// List all toolchains
    #[arg(long, alias = "toolchain")]
    pub toolchains: bool,

    /// List all targets
    #[arg(short, long, aliases = ["target", "machine", "machines"])]
    pub targets: bool,

    /// List all runners
    #[arg(short, long, alias = "runner")]
    pub runners: bool,
}

#[derive(Args)]
#[command(version, about, long_about = None)]
pub struct MakeArgs {
    /// Toolchain(s) in <layer>.<name> form (e.g. gnu.gcc), comma-sepparated
    #[arg(long, default_value_t = String::from("last"))]
    pub toolchain: String,

    #[arg(long, default_value_t = String::from("last"))]
    pub toolchain_target: String,

    /// Target(s) in <layer>.<name> form (e.g. qemu.virt_rvi20u32), comma-separated
    #[arg(short, long, default_value_t = String::from("last"), aliases=["targets", "machine", "machines"])]
    pub target: String,

    /// Configure build for selected target(s)
    #[arg(short, long)]
    pub configure: bool,

    /// Build type to configure (e.g. Release)
    #[arg(long, default_value_t = String::from("Release"), requires = "configure")]
    pub build_type: String,

    /// Build generator (e.g. Unix Makefiles)
    #[arg(long, default_value_t = String::from("Unix Makefiles"), requires = "configure")]
    pub build_generator: String,

    /// Perform build for selected target(s)
    #[arg(short, long)]
    pub build: bool,

    /// Number of parallel jobs to use during build
    #[arg(short, long, default_value_t = 1, requires = "build")]
    pub jobs: u32,

    /// Run tests for selected target(s)
    #[arg(short, long)]
    pub test: bool,

    /// Run selected actions in Docker container(s)
    #[arg(short, long)]
    pub docker: bool,

    /// Set --user to desired value when invoking Docker
    #[arg(long)]
    pub docker_user: Option<String>,
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

    /// Set --user to desired value when invoking Docker
    #[arg(long)]
    pub docker_user: Option<String>,
}

#[derive(Args)]
#[command(version, about, long_about = None)]
pub struct RunArgs {
    /// Toolchain(s) in <layer>.<name> form (e.g. gnu.gcc), comma-sepparated
    #[arg(long, default_value_t = String::from("last"))]
    pub toolchain: String,

    /// Target(s) in <layer>.<name> form (e.g. qemu.virt_rvi20u32), comma-separated
    #[arg(short, long, default_value_t = String::from("last"), alias="machine")]
    pub target: String,

    /// Runners(s) in <layer>.<name> form (e.g. qemu.system32-elf), comma-separated
    #[arg(short, long, default_value_t = String::from("last"))]
    pub runner: String,

    /// Run selected actions in Docker container(s)
    #[arg(short, long)]
    pub docker: bool,

    /// Executable image file to run, absolute path
    #[arg(short, long)]
    pub file: PathBuf,

    /// Whether to run in debug configuration
    #[arg(short, long)]
    pub debug: bool,

    /// Delay after each run
    #[arg(short, long, default_value_t = 0)]
    pub delay: u32,

    /// File to dump after each run, absolute or relative to build directory
    #[arg(short, long)]
    pub dump_file: Option<PathBuf>,
}

#[derive(Args)]
#[command(version, about, long_about = None)]
pub struct MonikerArgs {
    /// Path to output moniker for
    pub path: PathBuf,
}

/// RISC-V Base Layer build tool
#[derive(Parser)]
#[command(version, about, long_about = None)]
pub enum Arguments {
    /// List available elements of different kinds
    List(ListArgs),
    /// Configure, build and test targets
    Make(MakeArgs),
    /// Manage Docker images and containers for toolchains
    Docker(DockerArgs),
    /// Run executable files on targets
    Run(RunArgs),
    /// Output moniker (<layer>.<name>) for a path
    Moniker(MonikerArgs),
    /// Output diagnostic information
    Diagnostic,
}
