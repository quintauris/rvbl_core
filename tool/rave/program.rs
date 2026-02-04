mod argument;
mod command;
mod configuration;
mod console;
mod element;
mod interactive;
mod path;
mod select;

use argument::Arguments;
use clap::Parser;
use element::Element;
use itertools::Itertools;
use std::process::ExitCode;
use walkdir::WalkDir;

fn main() -> ExitCode {
    let root = path::layers_root().expect("Layers root path not found.");
    let configuration_file = path::configuration_file(&root);
    let mut elements: Vec<Element> = vec![];
    let mut configuration = configuration::read(&configuration_file).unwrap_or_default();
    let mut scanned = 0;
    let mut result = ExitCode::from(0);
    let extra_arguments_delimiter = std::env::args().find_position(|a| a == "--");
    let arguments = match extra_arguments_delimiter {
        Some((position, _)) => std::env::args().take(position).into_iter(),
        None => std::env::args().take(std::env::args().len()),
    };
    let walker = WalkDir::new(&root).into_iter();

    for entry in
        walker.filter_entry(|e| !path::is_hidden(e) && path::is_directory(e) && !path::is_build(e))
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

    match Arguments::parse_from(arguments) {
        Arguments::List(args) => {
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

            if !args.targets && !args.toolchains && !args.runners {
                println!("{}", elements.iter().format("\n"));
            }
        }
        Arguments::Make(args) => {
            let targets = select::targets(&args.target, &elements, &mut configuration);
            let toolchains = select::toolchains(&args.toolchain, &elements, &mut configuration);

            'target_loop: for target in targets {
                for toolchain in toolchains.iter().into_iter() {
                    if args.configure {
                        console::info!(
                            "Configuring machine {} with toolchain {} ...",
                            target.element.moniker(),
                            toolchain.element.moniker(),
                        );
                        match command::configure(
                            &root,
                            &target,
                            &toolchain,
                            &args.build_type,
                            args.docker,
                        ) {
                            Ok(code) => {
                                if code != 0.into() {
                                    result = code;
                                    break 'target_loop;
                                }
                            }
                            Err(_) => result = ExitCode::from(254),
                        };
                    }

                    if args.build {
                        console::info!(
                            "Building machine {} with toolchain {} ...",
                            target.element.moniker(),
                            toolchain.element.moniker(),
                        );
                        match command::build(&root, &target, &toolchain, args.jobs, args.docker) {
                            Ok(code) => {
                                if code != 0.into() {
                                    result = code;
                                    break 'target_loop;
                                }
                            }
                            Err(_) => result = ExitCode::from(254),
                        };
                    }

                    if args.test {
                        console::info!(
                            "Testing machine {} with toolchain {} ...",
                            target.element.moniker(),
                            toolchain.element.moniker(),
                        );
                        match command::test(&root, &target, &toolchain, args.docker) {
                            Ok(code) => {
                                if code != 0.into() {
                                    result = code;
                                    break 'target_loop;
                                }
                            }
                            Err(_) => result = ExitCode::from(254),
                        };
                    }
                }
            }

            configuration::write(&configuration_file, &configuration);
        }
        Arguments::Docker(args) => {
            let toolchains = select::toolchains(&args.toolchain, &elements, &mut configuration);

            for toolchain in toolchains.iter().into_iter() {
                if args.provision {
                    console::info!(
                        "Provisioning Docker toolchain {} ...",
                        toolchain.element.moniker(),
                    );

                    match command::provision(&root, &toolchain) {
                        Ok(code) => {
                            if code != 0.into() {
                                result = code;
                                break;
                            }
                        }
                        Err(_) => result = ExitCode::from(254),
                    }
                }

                if args.up {
                    console::info!(
                        "Spinning up Docker container for toolchain {} ...",
                        toolchain.element.moniker(),
                    );
                    match command::up(&root, &toolchain) {
                        Ok(code) => {
                            if code != 0.into() {
                                result = code;
                                break;
                            }
                        }
                        Err(_) => result = ExitCode::from(254),
                    }
                }

                if args.down {
                    console::info!(
                        "Winding down Docker container for toolchain {} ...",
                        toolchain.element.moniker(),
                    );
                    match command::down(&root, &toolchain) {
                        Ok(code) => {
                            if code != 0.into() {
                                result = code;
                                break;
                            }
                        }
                        Err(_) => result = ExitCode::from(254),
                    }
                }
            }

            configuration::write(&configuration_file, &configuration);
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
                        ) {
                            Ok(code) => {
                                if code != 0.into() {
                                    result = code;
                                    break 'target_loop;
                                }
                            }
                            Err(_) => result = ExitCode::from(254),
                        }
                    }
                }
            }

            configuration::write(&configuration_file, &configuration);
        }
        Arguments::Moniker(args) => {
            let path = match args.path {
                Some(p) => p,
                None => root,
            };

            match Element::from(&path) {
                Element::Unknown => result = ExitCode::from(1),
                x => {
                    println!("{}", x);
                }
            }
        }
        Arguments::Diagnostic => {
            console::info!("Layers root: {}", root.as_os_str().to_str().unwrap());
            console::info!("Configuration: {:?}", configuration);
            console::info!(
                "Scanned {scanned} filesystem entries, found {} element(s).",
                elements.len()
            );
        }
    };

    result
}
