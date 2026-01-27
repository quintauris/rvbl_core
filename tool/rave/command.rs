use std::path::PathBuf;
use std::process::{Command, ExitCode, Stdio};

use itertools::Itertools;

use crate::{
    console::{info, line, spawning, warning},
    element::{Machine, Toolchain},
};

fn run<I, S>(
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

    run(
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

    run(
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

    run(
        "ctest",
        ["--test-dir", build_path.as_os_str().to_str().unwrap()],
        toolchain,
        false,
    )
}

pub fn provision(toolchain: &Toolchain) -> std::io::Result<ExitCode> {
    run(
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

    run(
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
    run(
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
                run(
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
    run("docker", ["stop", &toolchain.container()], toolchain, false)
}
