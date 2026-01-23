use crate::element::{Element, Machine, Toolchain};
use crate::configuration::Configuration;

pub fn machines<'a>(filter : &str, elements : &'a Vec<Element>, configuration : &mut Configuration) -> Vec<&'a Machine> {
    let input = if filter == "last" { configuration.last_machines.join(",") } else { filter.to_string() };
    let filters = input.split(',');
    let result : Vec<&'a Machine> = elements.iter().filter_map(
        |e| match e { Element::Machine(t) => Some(t), _ => None }
    ).filter(
        |m| filters.clone().find(|&f| { f == m.element.moniker() || filter == "all"}).is_some()
    ).collect();

    if result.is_empty(){
        panic!("Machine not available or not specified");
    } else {
        configuration.last_machines.clear();
        result.iter().for_each(|m| configuration.last_machines.push(m.element.moniker()));
        result
    }
}

pub fn toolchains<'a>(filter : &str, elements : &'a Vec<Element>, configuration : &mut Configuration) -> Vec<&'a Toolchain> {
    let input = if filter == "last" { configuration.last_toolchains.join(",") } else { filter.to_string() };
    let filters = input.split(',');
    let result : Vec<&'a Toolchain> = elements.iter().filter_map(
        |e| match e { Element::Toolchain(t) => Some(t), _ => None }
    ).filter(
        |m| filters.clone().find(|&f| f == m.element.moniker() || filter == "all").is_some() 
    ).collect();

    if result.is_empty(){
        panic!("Toolchain not available or not specified");
    } else {
        configuration.last_toolchains.clear();
        result.iter().for_each(|t| configuration.last_toolchains.push(t.element.moniker()));
        result
    }
}
