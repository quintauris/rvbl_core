// Copyright 2026 Quintauris GmbH
// Licensed under the Apache License, Version 2.0 (the "License").
// https://www.apache.org/licenses/LICENSE-2.0

use serde::{Deserialize, Serialize};

use crate::evaluator;

#[derive(Serialize, Deserialize, Debug)]
pub struct Value {
    pub name: String,
    pub description: Option<String>,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Field {
    pub name: String,
    pub description: Option<String>,
    pub position: i32,
    pub length: Option<u32>,
    pub values: Option<Vec<Value>>,
}

impl Field {
    #[allow(unused)]
    pub fn resolve_position(&self, register: &Register, cpu: &Cpu) -> u32 {
        match self.position < 0 {
            true => (register.resolve_width(cpu) as i32 + self.position) as u32,
            false => self.position as u32,
        }
    }

    #[allow(unused)]
    pub fn resolve_length(&self, register: &Register, cpu: &Cpu) -> u32 {
        self.length
            .unwrap_or(register.resolve_width(cpu) - self.resolve_position(register, cpu))
    }
}

#[cfg(test)]
#[test]
fn test_resolve_position() {
    let cpu = Cpu {
        name: String::new(),
        description: None,
        xlen: 32,
        register_sets: vec![],
    };
    let register = Register {
        name: String::new(),
        description: None,
        class: String::new(),
        offset: String::new(),
        width: None,
        values: None,
        fields: None,
        indexing: None,
    };
    let mut field = Field {
        name: String::new(),
        description: None,
        position: 0,
        length: None,
        values: None,
    };

    assert_eq!(field.resolve_position(&register, &cpu), 0);
    field.position = 5;
    assert_eq!(field.resolve_position(&register, &cpu), 5);
    field.position = -2;
    assert_eq!(field.resolve_position(&register, &cpu), 30);
}

#[cfg(test)]
#[test]
fn test_resolve_length() {
    let cpu = Cpu {
        name: String::new(),
        description: None,
        xlen: 32,
        register_sets: vec![],
    };
    let register = Register {
        name: String::new(),
        description: None,
        class: String::new(),
        offset: String::new(),
        width: None,
        values: None,
        fields: None,
        indexing: None,
    };
    let mut field = Field {
        name: String::new(),
        description: None,
        position: 0,
        length: None,
        values: None,
    };

    assert_eq!(field.resolve_length(&register, &cpu), 32);
    field.position = 16;
    assert_eq!(field.resolve_length(&register, &cpu), 16);
    field.length = Some(8);
    assert_eq!(field.resolve_length(&register, &cpu), 8);
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Indexing {
    pub stride: Option<String>,
    pub lower_bound: Option<String>,
    pub upper_bound: String,
}

impl Indexing {
    #[allow(unused)]
    pub fn resolve_stride(&self, register: &Register, cpu: &Cpu) -> u32 {
        match &self.stride {
            None => register.resolve_width(cpu) / 8,
            Some(x) => evaluator::parse_expression(x.as_str()).try_into().unwrap(),
        }
    }
}

#[cfg(test)]
#[test]
fn test_resolve_stride() {
    let cpu = Cpu {
        name: String::new(),
        description: None,
        xlen: 32,
        register_sets: vec![],
    };
    let mut register = Register {
        name: String::new(),
        description: None,
        class: String::new(),
        offset: String::new(),
        width: None,
        values: None,
        fields: None,
        indexing: Some(Indexing {
            stride: Some(String::from("64")),
            lower_bound: None,
            upper_bound: String::from("8"),
        }),
    };

    assert_eq!(
        register
            .indexing
            .as_ref()
            .unwrap()
            .resolve_stride(&register, &cpu),
        64
    );
    register.indexing.as_mut().unwrap().stride = None;
    assert_eq!(
        register
            .indexing
            .as_ref()
            .unwrap()
            .resolve_stride(&register, &cpu),
        4
    );
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Register {
    pub name: String,
    pub description: Option<String>,
    pub class: String,
    pub offset: String,
    pub width: Option<u32>,
    pub values: Option<Vec<Value>>,
    pub fields: Option<Vec<Field>>,
    pub indexing: Option<Indexing>,
}

impl Register {
    pub fn resolve_width(&self, cpu: &Cpu) -> u32 {
        self.width.unwrap_or(cpu.xlen)
    }
}

#[cfg(test)]
#[test]
fn test_resolve_width() {
    let cpu = Cpu {
        name: String::new(),
        description: None,
        xlen: 32,
        register_sets: vec![],
    };
    let mut register = Register {
        name: String::new(),
        description: None,
        class: String::new(),
        offset: String::new(),
        width: None,
        values: None,
        fields: None,
        indexing: None,
    };

    assert_eq!(register.resolve_width(&cpu), 32);
    register.width = Some(64);
    assert_eq!(register.resolve_width(&cpu), 64);
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Cpu {
    pub name: String,
    pub description: Option<String>,
    pub xlen: u32,
    pub register_sets: Vec<Vec<Register>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MemoryRegion {
    pub name: String,
    pub description: Option<String>,
    pub base: String,
    pub size: String,
    pub registers: Option<Vec<Register>>,
    pub read: Option<bool>,
    pub write: Option<bool>,
    pub execute: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Parameter {
    pub name: String,
    pub description: Option<String>,
    pub stage: String,
    pub class: String,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Device {
    pub name: String,
    pub description: Option<String>,
    pub instance: String,
    pub memory_regions: Vec<MemoryRegion>,
    pub parameters: Option<Vec<Parameter>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Machine {
    pub name: String,
    pub description: Option<String>,
    pub cpus: Vec<Cpu>,
    pub memory_regions: Vec<MemoryRegion>,
    pub devices: Vec<Device>,
    pub parameters: Vec<Parameter>,
}
