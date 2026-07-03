// Copyright 2026 Quintauris GmbH
// Licensed under the Apache License, Version 2.0 (the "License").
// https://www.apache.org/licenses/LICENSE-2.0

use itertools::Itertools;

use crate::configuration;
use crate::element::{Element, Moniker, Runner, Target, Toolchain};
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

fn is_target(element: &&Element) -> bool {
    matches!(element, Element::Target(_))
}

fn is_toolchain(element: &&Element) -> bool {
    matches!(element, Element::Toolchain(_))
}

fn is_runner(element: &&Element) -> bool {
    matches!(element, Element::Runner(_))
}

fn all<'a, F>(elements: &'a Vec<Element>, f: F) -> Vec<&'a Element>
where
    F: Fn(&&Element) -> bool,
{
    elements.iter().filter(f).collect()
}

pub fn targets<'a>(
    filter: &str,
    elements: &'a Vec<Element>,
    configuration: &mut configuration::Configuration,
) -> Vec<&'a Target> {
    let all: Vec<String> = all(elements, is_target)
        .iter()
        .map(|e| e.moniker())
        .collect();
    let input = match filter {
        "all" => all.iter().join(","),
        "last" => configuration.last_targets.join(","),
        "ask" => interactive::choose("target", &all).expect("No option chosen"),
        _ => filter.to_string(),
    };
    let targets = generic(&input, elements, |e| matches!(e, Element::Target(_)));
    let result: Vec<&Target> = targets
        .iter()
        .map(|e| match e {
            Element::Target(m) => Some(m),
            _ => None,
        })
        .flatten()
        .collect();

    configuration.last_targets = result.iter().map(|m| m.element.moniker()).collect();

    result
}

pub fn toolchains<'a>(
    filter: &str,
    elements: &'a Vec<Element>,
    configuration: &mut configuration::Configuration,
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

pub fn toolchain_target<'a>(
    filter: &str,
    toolchain: &Toolchain,
    target_configuration: &configuration::Target,
    configuration: &mut configuration::Configuration,
) -> String {
    let default = target_configuration
        .default_toolchain_target
        .as_ref()
        .unwrap_or(&String::from("rt_europa_m32"))
        .clone();
    let last = configuration
        .last_toolchain_target
        .as_ref()
        .unwrap_or(&default)
        .clone();
    let input = match filter {
        "last" => last,
        "default" => default,
        "ask" => interactive::choose("toolchain/target", &toolchain.targets()).expect(""),
        _ => filter.to_string(),
    };

    configuration.last_toolchain_target = Some(input.clone());

    input
}

pub fn runners<'a>(
    filter: &str,
    elements: &'a Vec<Element>,
    configuration: &mut configuration::Configuration,
) -> Vec<&'a Runner> {
    let all: Vec<String> = all(elements, is_runner)
        .iter()
        .map(|e| e.moniker())
        .collect();
    let input = match filter {
        "all" => all.iter().join(","),
        "last" => configuration.last_runners.join(","),
        "ask" => interactive::choose("runner", &all).expect(""),
        _ => filter.to_string(),
    };
    let runners = generic(&input, elements, |e| matches!(e, Element::Runner(_)));
    let result: Vec<&Runner> = runners
        .iter()
        .map(|e| match e {
            Element::Runner(m) => Some(m),
            _ => None,
        })
        .flatten()
        .collect();

    configuration.last_runners = result.iter().map(|m| m.element.moniker()).collect();

    result
}
