mod argument;
mod command;
mod configuration;
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
    let mut configuration = configuration::read(&configuration_file);
    let mut scanned = 0;
    let mut result = ExitCode::from(0);

    println!("Layers root: {:?}", root.as_os_str());
    println!("Configuration: {:?}", configuration);

    let walker = WalkDir::new(&root).into_iter();

    for entry in
        walker.filter_entry(|e| !path::is_hidden(e) && path::is_directory(e) && !path::is_build(e))
    {
        let entry = entry.unwrap();
        let element = Element::from(&entry.path().to_path_buf());

        match element {
            Element::Machine(_) | Element::Toolchain(_) => elements.push(element),
            Element::Unknown => {}
        }

        scanned = scanned + 1;
    }

    println!(
        "Scanned {scanned} filesystem entries, found {} machines(s) and toolchain(s).",
        elements.len()
    );

    match Arguments::parse() {
        Arguments::List(args) => {
            if args.machines {
                let machines = select::machines("all", &elements, &mut configuration);

                println!("{}", machines.iter().format("\n"));
            } else if args.toolchains {
                let toolchains = select::toolchains("all", &elements, &mut configuration);

                println!("{}", toolchains.iter().format("\n"));
            } else {
                panic!("Please specify --machines or --toolchains");
            }
        }
        Arguments::Make(args) => {
            let machines = select::machines(&args.machine, &elements, &mut configuration);
            let toolchains = select::toolchains(&args.toolchain, &elements, &mut configuration);

            'machine_loop: for machine in machines {
                for toolchain in toolchains.iter().into_iter() {
                    if args.configure {
                        println!(
                            "Configuring machine {} with toolchain {} ...",
                            machine.element.moniker(),
                            toolchain.element.moniker(),
                        );
                        match command::configure(
                            &root,
                            &machine,
                            &toolchain,
                            &args.build_type,
                            args.docker,
                        ) {
                            Ok(code) => {
                                if code != 0.into() {
                                    result = code;
                                    break 'machine_loop;
                                }
                            }
                            Err(_) => result = ExitCode::from(254),
                        };
                    }

                    if args.build {
                        println!(
                            "Building machine {} with toolchain {} ...",
                            machine.element.moniker(),
                            toolchain.element.moniker(),
                        );
                        match command::build(&root, &machine, &toolchain, args.jobs, args.docker) {
                            Ok(code) => {
                                if code != 0.into() {
                                    result = code;
                                    break 'machine_loop;
                                }
                            }
                            Err(_) => result = ExitCode::from(254),
                        };
                    }

                    if args.test {
                        println!(
                            "Testing machine {} with toolchain {} ...",
                            machine.element.moniker(),
                            toolchain.element.moniker(),
                        );
                        match command::test(&root, &machine, &toolchain, args.docker) {
                            Ok(code) => {
                                if code != 0.into() {
                                    result = code;
                                    break 'machine_loop;
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
                    println!(
                        "Provisioning Docker toolchain {} ...",
                        toolchain.element.moniker(),
                    );

                    match command::provision(&toolchain) {
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
                    println!(
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
                    println!(
                        "Winding down Docker container for toolchain {} ...",
                        toolchain.element.moniker(),
                    );
                    match command::down(&toolchain) {
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
    };

    result
}
