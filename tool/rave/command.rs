use core::time;
use itertools::Itertools;
use regex::Regex;
use std::fs;
use std::io::{Write, stdout};
use std::path::PathBuf;
use std::process::{Command, ExitCode, Stdio};
use std::thread::sleep;
use which::which;

use crate::configuration::{self};
use crate::{
    console::{info, line, spawning, warning},
    element::{Machine, Runner, Toolchain},
};

fn execute<I, S>(
    program: &str,
    parameters: I,
    toolchain: &Toolchain,
    docker: bool,
) -> std::io::Result<ExitCode>
where
    I: IntoIterator<Item = S>,
    S: AsRef<std::ffi::OsStr>,
{
    let mut command = if docker {
        let mut result = Command::new("docker");
        result.args(["exec", "--interactive", "--tty", &toolchain.container()]);
        result.arg(program);
        result
    } else {
        Command::new(program)
    };

    command.args(parameters);
    command.stdout(Stdio::inherit());
    command.stdin(Stdio::inherit());
    command.stderr(Stdio::inherit());

    spawning!(
        "> {} {}",
        command.get_program().to_str().unwrap(),
        command.get_args().map(|a| a.to_str()).flatten().join(" ")
    );

    match command.output() {
        Ok(o) => {
            if o.status.success() {
                info!("Command completed successfully");
                Ok(ExitCode::from(0))
            } else {
                warning!("Command completed unsuccessfully");
                Ok(ExitCode::from(o.status.code().unwrap_or(255) as u8))
            }
        }
        Err(e) => Err(e),
    }
}

pub fn configure(
    root: &PathBuf,
    machine: &Machine,
    toolchain: &Toolchain,
    build_type: &String,
    docker: bool,
) -> std::io::Result<ExitCode> {
    let build_path = machine.build_path(root, machine, toolchain, docker);
    let mut toolchain_path = toolchain.element.path.clone();

    std::fs::create_dir_all(&build_path).expect("Can't create build directory");
    toolchain_path.push("riscv32_ilp32.cmake");

    execute(
        "cmake",
        [
            "-S",
            machine.element.path.as_os_str().to_str().unwrap(),
            "-B",
            build_path.as_os_str().to_str().unwrap(),
            "-D",
            &format!(
                "host_tools={}",
                toolchain
                    .tools_path(root, docker)
                    .as_os_str()
                    .to_str()
                    .unwrap()
            ),
            "-D",
            &format!("CMAKE_BUILD_TYPE={}", build_type),
            "--toolchain",
            toolchain_path.as_os_str().to_str().unwrap(),
        ],
        toolchain,
        docker,
    )
}

pub fn build(
    root: &PathBuf,
    machine: &Machine,
    toolchain: &Toolchain,
    jobs: u32,
    docker: bool,
) -> std::io::Result<ExitCode> {
    let build_path = machine.build_path(root, machine, toolchain, docker);

    execute(
        "cmake",
        [
            "--build",
            build_path.as_os_str().to_str().unwrap(),
            "--parallel",
            &jobs.to_string(),
        ],
        toolchain,
        docker,
    )
}

pub fn test(
    root: &PathBuf,
    machine: &Machine,
    toolchain: &Toolchain,
    docker: bool,
) -> std::io::Result<ExitCode> {
    let build_path = machine.build_path(root, machine, toolchain, docker);

    execute(
        "ctest",
        ["--test-dir", build_path.as_os_str().to_str().unwrap()],
        toolchain,
        false,
    )
}

pub fn provision(toolchain: &Toolchain) -> std::io::Result<ExitCode> {
    execute(
        "docker",
        [
            "build",
            "-t",
            &toolchain.image(),
            toolchain.element.path.as_os_str().to_str().unwrap(),
        ],
        toolchain,
        false,
    )
}

fn provision_tools(root: &PathBuf, toolchain: &Toolchain) -> std::io::Result<ExitCode> {
    let mut tools_manifest_path = root.clone();
    let mut generators_manifest_path = root.clone();

    tools_manifest_path.push("base/tool/Cargo.toml");
    generators_manifest_path.push("quintauris/tool/Cargo.toml");

    execute(
        "cargo",
        [
            "build",
            "--release",
            "--manifest-path",
            &tools_manifest_path.as_os_str().to_str().unwrap(),
            "--target-dir",
            &toolchain
                .tools_build_path(root, true)
                .as_os_str()
                .to_str()
                .unwrap(),
        ],
        toolchain,
        true,
    )?;
    execute(
        "cargo",
        [
            "build",
            "--release",
            "--manifest-path",
            &generators_manifest_path.as_os_str().to_str().unwrap(),
            "--target-dir",
            &toolchain
                .tools_build_path(root, true)
                .as_os_str()
                .to_str()
                .unwrap(),
        ],
        toolchain,
        true,
    )
}

pub fn up(root: &PathBuf, toolchain: &Toolchain) -> std::io::Result<ExitCode> {
    let mut inspect = Command::new("docker");
    let root_string = root.as_os_str().to_str().unwrap();

    inspect.args(["inspect", "-f", "{{.State.Status}}", &toolchain.container()]);

    spawning!(
        "> {} {:?}",
        inspect.get_program().to_str().unwrap(),
        inspect.get_args().map(|a| a.to_str()).flatten().join(" ")
    );

    match inspect.output() {
        Ok(o) => {
            if o.status.success() && String::from_utf8(o.stdout).unwrap() == "running" {
                info!("Container {} already running", toolchain.container());
                provision_tools(root, toolchain)
            } else {
                warning!(
                    "Container {} not running, spinning it up...",
                    toolchain.container()
                );
                execute(
                    "docker",
                    [
                        "run",
                        "--interactive",
                        "--tty",
                        "--detach",
                        "--name",
                        &toolchain.container(),
                        "--volume",
                        &format!("{root_string}:{root_string}"),
                        &toolchain.image(),
                    ],
                    toolchain,
                    false,
                )?;
                provision_tools(root, toolchain)
            }
        }
        Err(e) => Err(e),
    }
}

pub fn down(toolchain: &Toolchain) -> std::io::Result<ExitCode> {
    execute("docker", ["stop", &toolchain.container()], toolchain, false)
}

fn extract_parameters(
    runner: &Runner,
    machine_configuration: &crate::configuration::Machine,
    runner_configuration: &crate::configuration::Runner,
    debug: bool,
) -> Vec<String> {
    let mut result: Vec<String> = vec![];
    let mut machine_runner_parameters: &Vec<String> = &vec![];
    let mut runner_parameters: &Vec<String> = &vec![];
    let mut append_machine_runner_parameters = false;

    for (key, machine_runner) in &machine_configuration.runners {
        if let Ok(regex) = Regex::new(key) {
            if regex.is_match(&runner.element.moniker()) {
                if debug {
                    if let Some(parameters) = machine_runner.debug.as_ref() {
                        machine_runner_parameters = &parameters;
                    }
                } else {
                    machine_runner_parameters = &machine_runner.default;
                }

                append_machine_runner_parameters = machine_runner.append.unwrap_or(false);

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

    if append_machine_runner_parameters {
        result.extend_from_slice(runner_parameters);
        result.extend_from_slice(machine_runner_parameters);
    } else {
        result.extend_from_slice(machine_runner_parameters);
        result.extend_from_slice(runner_parameters);
    }

    result
}

pub fn run(
    root: &PathBuf,
    machine: &Machine,
    toolchain: &Toolchain,
    runner: &Runner,
    docker: bool,
    file: &PathBuf,
    debug: bool,
    delay: u32,
    dump_file: &Option<PathBuf>,
    machine_configuration: &crate::configuration::Machine,
) -> std::io::Result<ExitCode> {
    let mut result = Ok(ExitCode::from(0));

    if let Some(runner_configuration) = configuration::runner(&runner.configuration()) {
        for program in runner_configuration.programs.iter() {
            let program_replaced = program.replace("$machine", &machine.element.slug());

            if which(&program_replaced).is_ok() {
                let parameters = extract_parameters(
                    runner,
                    &machine_configuration,
                    &runner_configuration,
                    debug,
                );
                let mut file_path = root.clone();

                file_path.push(machine.build_path(root, machine, toolchain, docker));
                file_path.push(file);

                let parameters_replaced = parameters
                    .iter()
                    .map(|p| p.replace("$file_path", file_path.parent().unwrap().to_str().unwrap()))
                    .map(|p| p.replace("$file", file_path.to_str().unwrap()))
                    .map(|p| p.replace("$machine_path", &machine.element.path.to_str().unwrap()))
                    .map(|p| p.replace("$machine", &machine.element.slug()));

                result = execute(&program_replaced, parameters_replaced, toolchain, docker);

                if delay != 0 {
                    sleep(time::Duration::from_secs(delay.into()));
                }

                if let Some(dump) = dump_file {
                    let mut dump_path = root.clone();

                    dump_path.push(machine.build_path(root, machine, toolchain, docker));
                    dump_path.push(dump);

                    if let Ok(data) = fs::read(&dump_path) {
                        let _ = stdout().write(&data);

                        println!();
                    } else {
                        warning!("File \"{}\" not found.", dump.to_str().unwrap());
                        result = Err(std::io::Error::from(std::io::ErrorKind::NotFound))
                    }
                }

                break;
            } else {
                warning!("Program \"{}\" not found.", program_replaced);
                result = Err(std::io::Error::from(std::io::ErrorKind::NotFound))
            }
        }
    } else {
        result = Err(std::io::Error::from(std::io::ErrorKind::InvalidData))
    }

    result
}
