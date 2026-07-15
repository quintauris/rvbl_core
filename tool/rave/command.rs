// Copyright 2026 Quintauris GmbH
// Licensed under the Apache License, Version 2.0 (the "License").
// https://www.apache.org/licenses/LICENSE-2.0

use core::time;
use itertools::Itertools;
use log::{info, warn};
use regex::Regex;
use std::fmt::Display;
use std::fs;
use std::io::{Write, stdout};
use std::path::PathBuf;
use std::process::{Command, ExitCode, Stdio};
use std::thread::sleep;
use which::which;

use crate::configuration::{self};
use crate::element::{Runner, Target, Toolchain};

pub enum Error {
    IoError(std::io::Error),
    ConfigurationFileError(configuration::Error),
    ProgramError(which::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::IoError(error) => error.fmt(f),
            Error::ConfigurationFileError(error) => error.fmt(f),
            Error::ProgramError(error) => error.fmt(f),
        }
    }
}

type Result<T> = std::result::Result<T, Error>;

fn execute<I, S>(
    root: &PathBuf,
    program: &str,
    parameters: I,
    toolchain: &Toolchain,
    docker: bool,
    docker_container: Option<String>,
    docker_user: Option<String>,
) -> Result<ExitCode>
where
    I: IntoIterator<Item = S>,
    S: AsRef<std::ffi::OsStr>,
{
    let mut command = if docker {
        let mut result = Command::new("docker");

        result.args([
            "exec",
            "--interactive",
            "--tty",
            "--env",
            &format!(
                "RVBL_TOOL={}",
                toolchain.tools_path(root, docker).to_str().unwrap()
            ),
            "--env",
            &format!("RVBL_ROOT={}", root.to_str().unwrap()),
        ]);

        match docker_user {
            Some(user) => {
                result.args(["--user", &user]);
            }
            None => {}
        }

        match docker_container {
            Some(container) => {
                result.arg(container);
            }
            None => {
                result.arg(toolchain.container());
            }
        }

        result.arg(program);

        result
    } else {
        Command::new(program)
    };

    command.args(parameters);
    command.stdout(Stdio::inherit());
    command.stdin(Stdio::inherit());
    command.stderr(Stdio::inherit());

    warn!(
        "{}\n{}",
        command.get_program().to_str().unwrap(),
        command.get_args().map(|a| a.to_str()).flatten().join("\n")
    );

    match command.output() {
        Ok(o) => {
            if o.status.success() {
                info!("Command completed successfully");
                Ok(ExitCode::from(0))
            } else {
                warn!(
                    "Command completed unsuccessfully with code: {:#?}",
                    o.status.code()
                );
                Ok(ExitCode::from(o.status.code().unwrap_or(255) as u8))
            }
        }
        Err(e) => Err(Error::IoError(e)),
    }
}

pub fn configure(
    root: &PathBuf,
    target: &Target,
    toolchain: &Toolchain,
    toolchain_target: &str,
    build_type: &String,
    build_generator: &String,
    docker: bool,
    docker_user: Option<String>,
) -> Result<ExitCode> {
    let build_path = target.build_path(root, target, toolchain, docker);
    let toolchain_path = toolchain.path().join(format!("{toolchain_target}.cmake"));

    std::fs::create_dir_all(&build_path).map_err(|e| Error::IoError(e))?;

    execute(
        root,
        "cmake",
        [
            "-S",
            target.path().to_str().unwrap(),
            "-B",
            build_path.to_str().unwrap(),
            "-G",
            build_generator,
            "-D",
            &format!("CMAKE_BUILD_TYPE={}", build_type),
            "--toolchain",
            toolchain_path.to_str().unwrap(),
        ],
        toolchain,
        docker,
        None,
        docker_user,
    )
}

pub fn build(
    root: &PathBuf,
    target: &Target,
    toolchain: &Toolchain,
    jobs: u32,
    docker: bool,
    docker_user: Option<String>,
) -> Result<ExitCode> {
    let build_path = target.build_path(root, target, toolchain, docker);

    execute(
        root,
        "cmake",
        [
            "--build",
            build_path.to_str().unwrap(),
            "--parallel",
            &jobs.to_string(),
        ],
        toolchain,
        docker,
        None,
        docker_user,
    )
}

pub fn test(
    root: &PathBuf,
    target: &Target,
    toolchain: &Toolchain,
    filter: &Option<String>,
    filter_exclude: &Option<String>,
    docker: bool,
    docker_user: Option<String>,
) -> Result<ExitCode> {
    let build_path = target.build_path(root, target, toolchain, docker);
    let dir_parameters = vec!["--test-dir", build_path.to_str().unwrap()];
    let filter_parameters = match &filter {
        Some(r) => vec!["-R", r],
        None => vec![],
    };
    let filter_exclude_parameters = match &filter_exclude {
        Some(e) => vec!["-E", e],
        None => vec![],
    };
    let parameters = vec![dir_parameters, filter_parameters, filter_exclude_parameters];

    execute(
        root,
        "ctest",
        parameters.iter().flatten(),
        toolchain,
        false,
        None,
        docker_user,
    )
}

pub fn provision(
    root: &PathBuf,
    toolchain: &Toolchain,
    docker_user: Option<String>,
) -> Result<ExitCode> {
    execute(
        root,
        "docker",
        [
            "build",
            "-t",
            &toolchain.image(),
            toolchain.path().to_str().unwrap(),
        ],
        toolchain,
        false,
        None,
        docker_user,
    )
}

fn provision_tools(
    root: &PathBuf,
    toolchain: &Toolchain,
    docker_user: Option<String>,
) -> Result<ExitCode> {
    let tools_manifest_path = root.join("core/tool/Cargo.toml");

    execute(
        root,
        "cargo",
        [
            "build",
            "--release",
            "--manifest-path",
            &tools_manifest_path.to_str().unwrap(),
            "--target-dir",
            &toolchain.tools_build_path(root, true).to_str().unwrap(),
        ],
        toolchain,
        true,
        None,
        docker_user.clone(),
    )
}

pub fn up(root: &PathBuf, toolchain: &Toolchain, docker_user: Option<String>) -> Result<ExitCode> {
    let mut inspect = Command::new("docker");
    let root_string = root.to_str().unwrap();

    inspect.args(["inspect", "-f", "{{.State.Status}}", &toolchain.container()]);

    warn!(
        "{}\n{}",
        inspect.get_program().to_str().unwrap(),
        inspect.get_args().map(|a| a.to_str()).flatten().join("\n")
    );

    match inspect.output() {
        Ok(o) => {
            if o.status.success() && String::from_utf8(o.stdout).unwrap() == "running" {
                info!("Container {} already running", toolchain.container());
                provision_tools(root, toolchain, docker_user)
            } else {
                let container = toolchain.container();
                let volume = format!("{root_string}:{root_string}");
                let image = toolchain.image();
                let mut parameters: Vec<String> = vec![
                    "run".into(),
                    "--interactive".into(),
                    "--tty".into(),
                    "--detach".into(),
                    "--name".into(),
                    container,
                    "--volume".into(),
                    volume,
                ];

                warn!(
                    "Container {} not running, spinning it up...",
                    toolchain.container()
                );

                match docker_user.clone() {
                    Some(user) => {
                        parameters.push("--user".into());
                        parameters.push(user);
                    }
                    None => {}
                }

                parameters.push(image);

                execute(
                    root,
                    "docker",
                    parameters,
                    toolchain,
                    false,
                    None,
                    docker_user.clone(),
                )?;
                provision_tools(root, toolchain, docker_user)
            }
        }
        Err(e) => Err(Error::IoError(e)),
    }
}

pub fn down(
    root: &PathBuf,
    toolchain: &Toolchain,
    docker_user: Option<String>,
) -> Result<ExitCode> {
    execute(
        root,
        "docker",
        ["stop", &toolchain.container()],
        toolchain,
        false,
        None,
        docker_user.clone(),
    )
}

fn extract_parameters(
    runner: &Runner,
    target: &Target,
    target_configuration: &crate::configuration::Target,
    runner_configuration: &crate::configuration::Runner,
    debug: bool,
) -> Vec<String> {
    let mut result: Vec<String> = vec![];
    let mut target_runner_parameters: &Vec<String> = &vec![];
    let mut runner_parameters: &Vec<String> = &vec![];
    let mut runner_parameters_first = false;

    for (key, target_runner) in &target_configuration.runners {
        if let Ok(regex) = Regex::new(key) {
            if regex.is_match(&runner.element.moniker()) {
                if debug {
                    if let Some(parameters) = target_runner.debug.as_ref() {
                        target_runner_parameters = &parameters;
                    } else {
                        warn!(
                            "No debug parameters defined for target {}, using defaults.",
                            target
                        );
                        target_runner_parameters = &target_runner.default;
                    }
                } else {
                    target_runner_parameters = &target_runner.default;
                }

                runner_parameters_first = target_runner.append.unwrap_or(false)
                    || runner_configuration.prepend.unwrap_or(false);

                break;
            }
        }
    }

    if debug {
        if let Some(parameters) = runner_configuration.parameters.debug.as_ref() {
            runner_parameters = &parameters;
        }
    } else {
        runner_parameters = &runner_configuration.parameters.default;
    }

    if runner_parameters_first {
        result.extend_from_slice(runner_parameters);
        result.extend_from_slice(target_runner_parameters);
    } else {
        result.extend_from_slice(target_runner_parameters);
        result.extend_from_slice(runner_parameters);
    }

    result
}

fn replace_parameters(
    root: &PathBuf,
    file: &PathBuf,
    target: &Target,
    parameters: &Vec<String>,
) -> Vec<String> {
    parameters
        .iter()
        .map(|p| {
            p.replace(
                "$file_canonical",
                file.canonicalize().unwrap().to_str().unwrap(),
            )
        })
        .map(|p| p.replace("$file_path", file.parent().unwrap().to_str().unwrap()))
        .map(|p| {
            p.replace(
                "$filename_we",
                &PathBuf::from(file.file_stem().unwrap())
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap(),
            )
        })
        .map(|p| p.replace("$file", file.to_str().unwrap()))
        .map(|p| p.replace("$machine_path", &target.path().to_str().unwrap()))
        .map(|p| p.replace("$machine", &target.element.slug()))
        .map(|p| p.replace("$root", &root.to_str().unwrap()))
        .collect()
}

pub fn run(
    root: &PathBuf,
    target: &Target,
    toolchain: &Toolchain,
    runner: &Runner,
    docker: bool,
    file: &PathBuf,
    debug: bool,
    delay: u32,
    dump_file: &Option<PathBuf>,
    target_configuration: &crate::configuration::Target,
    docker_user: &Option<String>,
) -> Result<ExitCode> {
    let mut result = Ok(ExitCode::from(0));
    let runner_configuration = configuration::runner(&runner.configuration())
        .map_err(|e| Error::ConfigurationFileError(e))?;

    if file.exists() {
        let parameters = replace_parameters(
            &root,
            &file,
            &target,
            &extract_parameters(
                runner,
                target,
                &target_configuration,
                &runner_configuration,
                debug,
            ),
        );

        for program in runner_configuration.programs.iter() {
            let program_replaced = program.replace("$machine", &target.element.slug());

            if docker && let Some(docker_configuration) = &runner_configuration.docker {
                result = execute(
                    root,
                    &program_replaced,
                    &parameters,
                    toolchain,
                    docker,
                    Some(docker_configuration.container.clone()),
                    docker_user.clone(),
                );
            } else {
                match which(&program_replaced) {
                    Ok(_) => {
                        result = execute(
                            root,
                            &program_replaced,
                            &parameters,
                            toolchain,
                            docker,
                            None,
                            docker_user.clone(),
                        );

                        break;
                    }
                    Err(error) => {
                        result = Err(Error::ProgramError(error));
                        continue;
                    }
                }
            }

            if delay != 0 {
                sleep(time::Duration::from_secs(delay.into()));
            }

            if let Some(dump) = dump_file {
                let dump_path = root
                    .join(target.build_path(root, target, toolchain, docker))
                    .join(dump);

                stdout()
                    .write_all(&fs::read(&dump_path).map_err(|e| Error::IoError(e))?)
                    .map_err(|e| Error::IoError(e))?;
                println!();
            }
        }
    } else {
        result = Err(Error::IoError(std::io::Error::from(
            std::io::ErrorKind::NotFound,
        )));
    }

    result
}
