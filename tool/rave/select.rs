use itertools::Itertools;

use crate::configuration::Configuration;
use crate::element::{Element, Machine, Moniker, Toolchain};
use crate::interactive;

fn generic<'a, F>(filter: &str, elements: &'a Vec<Element>, f: F) -> Vec<&'a Element>
where
    F: Fn(&&Element) -> bool,
{
    let filters = filter.split(',');
    let result: Vec<&Element> = elements
        .iter()
        .filter(f)
        .filter(|e| filters.clone().find(|&f| f == e.moniker()).is_some())
        .collect();

    if result.is_empty() {
        panic!("Can't find any suitable elements in \"{filter}\"");
    } else {
        result
    }
}

fn is_machine(element: &&Element) -> bool {
    matches!(element, Element::Machine(_))
}

fn is_toolchain(element: &&Element) -> bool {
    matches!(element, Element::Toolchain(_))
}

fn all<'a, F>(elements: &'a Vec<Element>, f: F) -> Vec<&'a Element>
where
    F: Fn(&&Element) -> bool,
{
    elements.iter().filter(f).collect()
}

pub fn machines<'a>(
    filter: &str,
    elements: &'a Vec<Element>,
    configuration: &mut Configuration,
) -> Vec<&'a Machine> {
    let all: Vec<String> = all(elements, is_machine)
        .iter()
        .map(|e| e.moniker())
        .collect();
    let input = match filter {
        "all" => all.iter().join(","),
        "last" => configuration.last_machines.join(","),
        "ask" => interactive::choose("machine", &all).expect("No option chosen"),
        _ => filter.to_string(),
    };
    let machines = generic(&input, elements, |e| matches!(e, Element::Machine(_)));
    let result: Vec<&Machine> = machines
        .iter()
        .map(|e| match e {
            Element::Machine(m) => Some(m),
            _ => None,
        })
        .flatten()
        .collect();

    configuration.last_machines = result.iter().map(|m| m.element.moniker()).collect();

    result
}

pub fn toolchains<'a>(
    filter: &str,
    elements: &'a Vec<Element>,
    configuration: &mut Configuration,
) -> Vec<&'a Toolchain> {
    let all: Vec<String> = all(elements, is_toolchain)
        .iter()
        .map(|e| e.moniker())
        .collect();
    let input = match filter {
        "all" => all.iter().join(","),
        "last" => configuration.last_toolchains.join(","),
        "ask" => interactive::choose("toolchain", &all).expect(""),
        _ => filter.to_string(),
    };
    let toolchains = generic(&input, elements, |e| matches!(e, Element::Toolchain(_)));
    let result: Vec<&Toolchain> = toolchains
        .iter()
        .map(|e| match e {
            Element::Toolchain(m) => Some(m),
            _ => None,
        })
        .flatten()
        .collect();

    configuration.last_toolchains = result.iter().map(|m| m.element.moniker()).collect();

    result
}
