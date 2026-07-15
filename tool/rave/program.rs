// Copyright 2026 Quintauris GmbH
// Licensed under the Apache License, Version 2.0 (the "License").
// https://www.apache.org/licenses/LICENSE-2.0

mod argument;
mod command;
mod configuration;
mod element;
mod interactive;
mod path;
mod select;

use argument::Arguments;
use clap::Parser;
use configuration::Configuration;
use element::Element;
use itertools::Itertools;
use log::{error, info, warn};
use std::{path::PathBuf, process::ExitCode};
use walkdir::WalkDir;

fn main() -> ExitCode {
    let root = match initialize() {
        Ok(root) => root,
        Err(_) => return ExitCode::from(1),
    };
    let mut elements: Vec<Element> = vec![];
    let mut configuration =
        configuration::read(&path::configuration_file(&root)).unwrap_or_default();
    let mut scanned = 0;
    let mut result = ExitCode::from(0);

    for entry in WalkDir::new(&root)
        .into_iter()
        .filter_entry(|e| !path::is_hidden(e) && path::is_directory(e) && !path::is_build(e))
    {
        let entry = entry.unwrap();
        let element = Element::from(&entry.path().to_path_buf());

        match element {
            Element::Target(_) | Element::Toolchain(_) | Element::Runner(_) => {
                elements.push(element)
            }
            Element::Unknown => {}
        }

        scanned = scanned + 1;
    }

    match Arguments::parse_from(std::env::args()) {
        Arguments::List(args) => {
            if ![args.targets, args.toolchains, args.runners]
                .iter()
                .any(|x| *x)
            {
                println!("{}", elements.iter().format("\n"));
            } else {
                if args.targets {
                    println!(
                        "{}",
                        select::targets("all", &elements, &mut configuration)
                            .iter()
                            .format("\n")
                    );
                }

                if args.toolchains {
                    println!(
                        "{}",
                        select::toolchains("all", &elements, &mut configuration)
                            .iter()
                            .format("\n")
                    );
                }

                if args.runners {
                    println!(
                        "{}",
                        select::runners("all", &elements, &mut configuration)
                            .iter()
                            .format("\n")
                    );
                }
            }
        }
        Arguments::Make(args) => {
            let targets = select::targets(&args.target, &elements, &mut configuration);
            let toolchains = select::toolchains(&args.toolchain, &elements, &mut configuration);

            'target_loop: for target in targets {
                let target_configuration =
                    configuration::target(&target.configuration()).unwrap_or_default();

                for toolchain in toolchains.iter().into_iter() {
                    if args.configure {
                        let toolchain_target = select::toolchain_target(
                            &args.toolchain_target,
                            toolchain,
                            &target_configuration,
                            &mut configuration,
                        );

                        info!(
                            "Configuring machine {} with toolchain {} ...",
                            target.element.moniker(),
                            toolchain.element.moniker(),
                        );
                        match command::configure(
                            &root,
                            &target,
                            &toolchain,
                            &toolchain_target,
                            &args.build_type,
                            &args.build_generator,
                            args.docker,
                            args.docker_user.clone(),
                        ) {
                            Ok(code) => {
                                if code != 0.into() {
                                    result = code;
                                    break 'target_loop;
                                }
                            }
                            Err(error) => {
                                warn!("Configure failed: {error}");
                                result = ExitCode::from(254)
                            }
                        };
                    }

                    if args.build {
                        info!(
                            "Building machine {} with toolchain {} ...",
                            target.element.moniker(),
                            toolchain.element.moniker(),
                        );
                        match command::build(
                            &root,
                            &target,
                            &toolchain,
                            args.jobs,
                            args.docker,
                            args.docker_user.clone(),
                        ) {
                            Ok(code) => {
                                if code != 0.into() {
                                    result = code;
                                    break 'target_loop;
                                }
                            }
                            Err(error) => {
                                warn!("Build failed: {error}");
                                result = ExitCode::from(254)
                            }
                        };
                    }

                    if args.test {
                        info!(
                            "Testing machine {} with toolchain {} ...",
                            target.element.moniker(),
                            toolchain.element.moniker(),
                        );
                        match command::test(
                            &root,
                            &target,
                            &toolchain,
                            &args.test_filter,
                            &args.test_filter_exclude,
                            args.docker,
                            args.docker_user.clone(),
                        ) {
                            Ok(code) => {
                                if code != 0.into() {
                                    result = code;
                                    break 'target_loop;
                                }
                            }
                            Err(error) => {
                                warn!("Test failed: {error}");
                                result = ExitCode::from(254)
                            }
                        };
                    }
                }
            }

            update_configuration(&root, &configuration);
        }
        Arguments::Docker(args) => {
            let toolchains = select::toolchains(&args.toolchain, &elements, &mut configuration);

            for toolchain in toolchains.iter().into_iter() {
                if args.provision {
                    info!(
                        "Provisioning Docker toolchain {} ...",
                        toolchain.element.moniker(),
                    );

                    match command::provision(&root, &toolchain, args.docker_user.clone()) {
                        Ok(code) => {
                            if code != 0.into() {
                                result = code;
                                break;
                            }
                        }
                        Err(error) => {
                            warn!("Provision failed: {error}");
                            result = ExitCode::from(254)
                        }
                    }
                }

                if args.up {
                    info!(
                        "Spinning up Docker container for toolchain {} ...",
                        toolchain.element.moniker(),
                    );
                    match command::up(&root, &toolchain, args.docker_user.clone()) {
                        Ok(code) => {
                            if code != 0.into() {
                                result = code;
                                break;
                            }
                        }
                        Err(error) => {
                            warn!("Up failed: {error}");
                            result = ExitCode::from(254)
                        }
                    }
                }

                if args.down {
                    info!(
                        "Winding down Docker container for toolchain {} ...",
                        toolchain.element.moniker(),
                    );
                    match command::down(&root, &toolchain, args.docker_user.clone()) {
                        Ok(code) => {
                            if code != 0.into() {
                                result = code;
                                break;
                            }
                        }
                        Err(error) => {
                            warn!("Down failed: {error}");
                            result = ExitCode::from(254)
                        }
                    }
                }
            }

            update_configuration(&root, &configuration);
        }
        Arguments::Run(args) => {
            let targets = select::targets(&args.target, &elements, &mut configuration);
            let toolchains = select::toolchains(&args.toolchain, &elements, &mut configuration);
            let runners = select::runners(&args.runner, &elements, &mut configuration);

            'target_loop: for target in targets {
                let target_configuration =
                    configuration::target(&target.configuration()).unwrap_or_default();

                for toolchain in toolchains.iter().into_iter() {
                    for runner in runners.iter().into_iter() {
                        match command::run(
                            &root,
                            target,
                            toolchain,
                            runner,
                            args.docker,
                            &args.file,
                            args.debug,
                            args.delay,
                            &args.dump_file,
                            &target_configuration,
                            &None,
                        ) {
                            Ok(code) => {
                                if code != 0.into() {
                                    result = code;
                                    break 'target_loop;
                                }
                            }
                            Err(error) => {
                                warn!("Run failed: {error}");
                                result = ExitCode::from(254)
                            }
                        }
                    }
                }
            }

            update_configuration(&root, &configuration);
        }
        Arguments::Moniker(args) => match Element::from(&args.path) {
            Element::Unknown => {
                error!("Unable to extract moniker from {:?}", args.path);
                result = ExitCode::from(1)
            }
            x => println!("{}", x),
        },
        Arguments::Diagnostic => {
            diagnostic();
        }
    };

    result
}

fn initialize() -> std::io::Result<PathBuf> {
    colog::init();

    match path::layers_root() {
        Ok(path) => Ok(path),
        Err(error) => {
            error!("Unable to locate distribution root path: {}", error);
            diagnostic();
            Err(error)
        }
    }
}

fn diagnostic() {
    let check = |v: &str| -> bool {
        match std::env::var(v) {
            Ok(value) => {
                info!("{v}: \"{value}\"");
                true
            }
            Err(error) => {
                error!("{v} is NOT set: \"{error}\"");
                true
            }
        }
    };

    if check("RVBL_ROOT") && check("RVBL_BUILD") && check("RVBL_TOOL") {
        match path::layers_root() {
            Ok(path) => {
                info!("Distribution root is: {path:#?}");

                match configuration::read(&path::configuration_file(&path)) {
                    Ok(configuration) => info!("{:#?}", configuration),
                    Err(error) => error!("Unable to read configuration: {error}"),
                }
            }
            Err(error) => error!("Unable to identify distribution root: {error}"),
        }
    }
}

fn update_configuration(root: &PathBuf, configuration: &Configuration) {
    match configuration::write(&path::configuration_file(root), &configuration) {
        Ok(_) => (),
        Err(error) => error!("Unable to store configuration: {error}"),
    }
}
