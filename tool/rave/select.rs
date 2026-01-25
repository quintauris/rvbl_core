use itertools::Itertools;

use crate::element::{Element, Machine, Moniker, Toolchain};
use crate::configuration::Configuration;
use crate::interactive;

fn generic<'a, F>(filter : &str, elements : &'a Vec<Element>, configuration : &mut Configuration, f : F) -> Vec<&'a Element> 
where
    F : Fn(&&Element) -> bool
{
    let filters = filter.split(',');
    let result : Vec<&Element> = elements
        .iter().filter(f)
        .filter(|e| filters.clone().find(|&f| { f == e.moniker() }).is_some())
        .collect()
    ;

    if result.is_empty(){
        panic!("Can't find any suitable elements in \"{filter}\"");
    } else {
        configuration.last_machines.clear();
        result.iter().for_each(|m| configuration.last_machines.push(m.moniker()));
        result
    }
}

fn is_machine(element : &&Element) -> bool {
    matches!(element, Element::Machine(_))
}

fn is_toolchain(element : &&Element) -> bool {
    matches!(element, Element::Toolchain(_))
}

fn all<'a, F>(elements : &'a Vec<Element>, f : F) -> Vec<&'a Element> 
where
    F : Fn(&&Element) -> bool
{
    elements.iter().filter(f).collect()
}

pub fn machines<'a>(filter : &str, elements : &'a Vec<Element>, configuration : &mut Configuration) -> Vec<&'a Machine> {
    let all : Vec<String> = all(elements, is_machine).iter().map(|e| e.moniker()).collect();
    let input = match filter {
        "all" => all.iter().join(","),
        "last" => configuration.last_machines.join(","),
        "ask" => interactive::choose("machine", &all).expect("No option chosen"),
        _ => filter.to_string()
    };
    let machines = generic(&input, elements, configuration, |e| matches!(e, Element::Machine(_)));
    
    machines.iter().map(|e| match e { Element::Machine(m) => Some(m), _ => None } ).flatten().collect()
}

pub fn toolchains<'a>(filter : &str, elements : &'a Vec<Element>, configuration : &mut Configuration) -> Vec<&'a Toolchain> {
    let all : Vec<String> = all(elements, is_toolchain).iter().map(|e| e.moniker()).collect();
    let input = match filter {
        "all" => all.iter().join(","),
        "last" => configuration.last_toolchains.join(","),
        "ask" => interactive::choose("toolchain", &all).expect(""),
        _ => filter.to_string()
    };
    let toolchains = generic(&input, elements, configuration, |e| matches!(e, Element::Toolchain(_)));
    
    toolchains.iter().map(|e| match e { Element::Toolchain(m) => Some(m), _ => None } ).flatten().collect()
}
