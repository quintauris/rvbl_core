mod element;
mod path;
mod argument;
mod configuration;
mod command;
mod select;

use std::process::ExitCode;
use walkdir::WalkDir;
use itertools::Itertools;
use element::Element;
use argument::Arguments;
use clap::Parser;

fn main() -> ExitCode {
    let root = path::layers_root().expect("Layers root path not found.");
    let configuration_file = path::configuration_file(&root);
    let args = Arguments::parse();
    let mut elements : Vec<Element> = vec![];
    let mut scanned = 0;
    let mut result = ExitCode::from(0);
    let mut configuration = configuration::read(&configuration_file);

    println!("Layers root: {:?}", root.as_os_str());
    println!("Configuration: {:?}", configuration);

    let walker = WalkDir::new(&root).into_iter();
    
    for entry in walker.filter_entry(|e| !path::is_hidden(e) && path::is_directory(e) && !path::is_build(e)) {
        let entry = entry.unwrap();
        let element = Element::from(&entry.path().to_path_buf());
        
        match element {
            Element::Machine(_) | Element::Toolchain(_) => elements.push(element),
            Element::Unknown => {},
        }

        scanned = scanned + 1;
    }

    println!(
        "Scanned {scanned} filesystem entries, found {} machines(s) and toolchain(s).",
        elements.len()
    );

    match args {
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
        },
        Arguments::Configure(args) => {
            let machines = select::machines(&args.machine, &elements, &mut configuration);
            let toolchains = select::toolchains(&args.toolchain, &elements, &mut configuration);

            for machine in machines {
                for toolchain in toolchains.iter().into_iter() {
                    println!(
                        "Configuring machine {} with toolchain {} ...",
                        machine.element.moniker(),
                        toolchain.element.moniker(),
                    );
                    match command::configure(&root, &machine, &toolchain, &args.build_type, args.docker) {
                        Ok(code) => if code == 0.into() { continue } else { result = code },
                        Err(_) => result = ExitCode::from(254),
                    }
                }
            }
        
            configuration::write(&configuration_file, &configuration);
        },
        Arguments::Build(args) => {
            let machines = select::machines(&args.machine, &elements, &mut configuration);
            let toolchains = select::toolchains(&args.toolchain, &elements, &mut configuration);

            for machine in machines {
                for toolchain in toolchains.iter().into_iter() {
                    println!(
                        "Building machine {} with toolchain {} ...",
                        machine.element.moniker(),
                        toolchain.element.moniker(),
                    );
                    match command::build(&root, &machine, &toolchain, args.jobs, args.docker) {
                        Ok(code) => if code == 0.into() { continue } else { result = code },
                        Err(_) => result = ExitCode::from(254),
                    }
                }
            }
        
            configuration::write(&configuration_file, &configuration);
        }
        Arguments::Test(args) => {
            let machines = select::machines(&args.machine, &elements, &mut configuration);
            let toolchains = select::toolchains(&args.toolchain, &elements, &mut configuration);

            for machine in machines {
                for toolchain in toolchains.iter().into_iter() {
                    println!(
                        "Testing machine {} with toolchain {} ...",
                        machine.element.moniker(),
                        toolchain.element.moniker(),
                    );
                    match command::test(&root, &machine, &toolchain, args.docker) {
                        Ok(code) => if code == 0.into() { continue } else { result = code },
                        Err(_) => result = ExitCode::from(254),
                    }
                }
            }
        
            configuration::write(&configuration_file, &configuration);
        },
        Arguments::Provision(args) => {
            let toolchains = select::toolchains(&args.toolchain, &elements, &mut configuration);

            for toolchain in toolchains.iter().into_iter() {
                println!(
                    "Provisioning Docker toolchain {} ...",
                    toolchain.element.moniker(),
                );
                match command::provision(&toolchain) {
                    Ok(code) => if code == 0.into() { continue } else { result = code },
                    Err(_) => result = ExitCode::from(254),
                }
            }
        },
        Arguments::Up(args) => {
            let toolchains = select::toolchains(&args.toolchain, &elements, &mut configuration);

            for toolchain in toolchains.iter().into_iter() {
                println!(
                    "Spinning up Docker container for toolchain {} ...",
                    toolchain.element.moniker(),
                );
                match command::up(&root, &toolchain) {
                    Ok(code) => if code == 0.into() { continue } else { result = code },
                    Err(_) => result = ExitCode::from(254),
                }
            }
        },
        Arguments::Down(args) => {
            let toolchains = select::toolchains(&args.toolchain, &elements, &mut configuration);

            for toolchain in toolchains.iter().into_iter() {
                println!(
                    "Winding down Docker container for toolchain {} ...",
                    toolchain.element.moniker(),
                );
                match command::down(&toolchain) {
                    Ok(code) => if code == 0.into() { continue } else { result = code },
                    Err(_) => result = ExitCode::from(254),
                }
            }
        }
    };

    result
}