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
    pub fn resolve_position(
        &self,
        register: &Register,
        addressable: &Addressable,
        peripheral: &Peripheral,
        machine: &Machine,
    ) -> u32 {
        match self.position < 0 {
            true => {
                (register.resolve_width(&addressable, &peripheral, &machine) as i32 + self.position)
                    as u32
            }
            false => self.position as u32,
        }
    }

    #[allow(unused)]
    pub fn resolve_length(
        &self,
        register: &Register,
        addressable: &Addressable,
        peripheral: &Peripheral,
        machine: &Machine,
    ) -> u32 {
        let width = register.resolve_width(&addressable, &peripheral, &machine);
        let position = self.resolve_position(register, &addressable, &peripheral, &machine);

        self.length.unwrap_or(width - position)
    }
}

#[cfg(test)]
#[test]
fn test_resolve_position() {
    let machine = Machine {
        name: String::new(),
        description: None,
        word_size: 128,
        memory_map: vec![],
        peripherals: vec![],
        parameters: vec![],
    };
    let peripheral = Peripheral {
        name: String::new(),
        description: None,
        instance: String::new(),
        word_size: None,
        addressables: vec![],
        parameters: None,
    };
    let addressable = Addressable {
        name: String::new(),
        description: None,
        base: String::from("0"),
        size: String::from("64K"),
        registers: None,
        read: None,
        write: None,
        execute: None,
        word_size: None,
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

    assert_eq!(
        field.resolve_position(&register, &addressable, &peripheral, &machine),
        0
    );
    field.position = 5;
    assert_eq!(
        field.resolve_position(&register, &addressable, &peripheral, &machine),
        5
    );
    field.position = -2;
    assert_eq!(
        field.resolve_position(&register, &addressable, &peripheral, &machine),
        30
    );
}

#[cfg(test)]
#[test]
fn test_resolve_length() {
    let machine = Machine {
        name: String::new(),
        description: None,
        word_size: 128,
        memory_map: vec![],
        peripherals: vec![],
        parameters: vec![],
    };
    let peripheral = Peripheral {
        name: String::new(),
        description: None,
        instance: String::new(),
        word_size: None,
        addressables: vec![],
        parameters: None,
    };
    let addressable = Addressable {
        name: String::new(),
        description: None,
        base: String::from("0"),
        size: String::from("64K"),
        registers: None,
        read: None,
        write: None,
        execute: None,
        word_size: None,
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

    assert_eq!(
        field.resolve_length(&register, &addressable, &peripheral, &machine),
        32
    );
    field.position = 16;
    assert_eq!(
        field.resolve_length(&register, &addressable, &peripheral, &machine),
        16
    );
    field.length = Some(8);
    assert_eq!(
        field.resolve_length(&register, &addressable, &peripheral, &machine),
        8
    );
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Indexing {
    pub stride: Option<String>,
    pub lower_bound: Option<String>,
    pub upper_bound: String,
}

impl Indexing {
    #[allow(unused)]
    pub fn resolve_stride(
        &self,
        register: &Register,
        addressable: &Addressable,
        peripheral: &Peripheral,
        machine: &Machine,
    ) -> u32 {
        match &self.stride {
            None => register.resolve_width(&addressable, &peripheral, &machine) / 8,
            Some(x) => evaluator::parse_expression(x.as_str()).try_into().unwrap(),
        }
    }
}

#[cfg(test)]
#[test]
fn test_resolve_stride() {
    let machine = Machine {
        name: String::new(),
        description: None,
        word_size: 128,
        memory_map: vec![],
        peripherals: vec![],
        parameters: vec![],
    };
    let peripheral = Peripheral {
        name: String::new(),
        description: None,
        instance: String::new(),
        word_size: None,
        addressables: vec![],
        parameters: None,
    };
    let addressable = Addressable {
        name: String::new(),
        description: None,
        base: String::from("0"),
        size: String::from("64K"),
        registers: None,
        read: None,
        write: None,
        execute: None,
        word_size: None,
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
        register.indexing.as_ref().unwrap().resolve_stride(
            &register,
            &addressable,
            &peripheral,
            &machine
        ),
        64
    );
    register.indexing.as_mut().unwrap().stride = None;
    assert_eq!(
        register.indexing.as_ref().unwrap().resolve_stride(
            &register,
            &addressable,
            &peripheral,
            &machine
        ),
        128
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
    pub fn resolve_width(
        &self,
        addressable: &Addressable,
        peripheral: &Peripheral,
        machine: &Machine,
    ) -> u32 {
        self.width.unwrap_or(
            addressable
                .word_size
                .unwrap_or(peripheral.word_size.unwrap_or(machine.word_size)),
        )
    }
}

#[cfg(test)]
#[test]
fn test_resolve_width() {
    use std::vec;

    let machine = Machine {
        name: String::new(),
        description: None,
        word_size: 128,
        memory_map: vec![],
        peripherals: vec![],
        parameters: vec![],
    };
    let mut peripheral = Peripheral {
        name: String::new(),
        description: None,
        instance: String::new(),
        word_size: None,
        addressables: vec![],
        parameters: None,
    };
    let mut addressable = Addressable {
        name: String::new(),
        description: None,
        base: String::from("0"),
        size: String::from("64K"),
        registers: None,
        read: None,
        write: None,
        execute: None,
        word_size: None,
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

    assert_eq!(
        register.resolve_width(&addressable, &peripheral, &machine),
        128
    );
    peripheral.word_size = Some(64);
    assert_eq!(
        register.resolve_width(&addressable, &peripheral, &machine),
        64
    );
    addressable.word_size = Some(32);
    assert_eq!(
        register.resolve_width(&addressable, &peripheral, &machine),
        32
    );
    register.width = Some(16);
    assert_eq!(
        register.resolve_width(&addressable, &peripheral, &machine),
        16
    );
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Addressable {
    pub name: String,
    pub description: Option<String>,
    pub base: String,
    pub size: String,
    pub registers: Option<Vec<Register>>,
    pub read: Option<bool>,
    pub write: Option<bool>,
    pub execute: Option<bool>,
    pub word_size: Option<u32>,
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
pub struct Peripheral {
    pub name: String,
    pub description: Option<String>,
    pub instance: String,
    pub word_size: Option<u32>,
    pub addressables: Vec<Addressable>,
    pub parameters: Option<Vec<Parameter>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Machine {
    pub name: String,
    pub description: Option<String>,
    pub word_size: u32,
    pub memory_map: Vec<Addressable>,
    pub peripherals: Vec<Peripheral>,
    pub parameters: Vec<Parameter>,
}
