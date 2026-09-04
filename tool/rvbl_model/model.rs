// Copyright 2026 Quintauris GmbH
// Licensed under the Apache License, Version 2.0 (the "License").
// https://www.apache.org/licenses/LICENSE-2.0

use serde::{Deserialize, Serialize};

use crate::evaluator;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Value {
    pub name: String,
    pub description: Option<String>,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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

#[derive(Serialize, Deserialize, Debug, Clone)]
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Register {
    pub name: String,
    pub description: Option<String>,
    pub class: Option<String>,
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

    #[allow(unused)]
    pub fn resolve_class(&self, addressable: &Addressable) -> String {
        match self.class.as_ref() {
            Some(class) => class.clone(),
            None => addressable.default_register_class.clone(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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
    pub default_register_class: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Parameter {
    pub name: String,
    pub description: Option<String>,
    pub stage: String,
    pub class: String,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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

#[cfg(test)]
mod tests {
    use super::*;

    static MACHINE: Machine = Machine {
        name: String::new(),
        description: None,
        word_size: 128,
        memory_map: vec![],
        peripherals: vec![],
        parameters: vec![],
    };
    static PERIPHERAL: Peripheral = Peripheral {
        name: String::new(),
        description: None,
        instance: String::new(),
        word_size: None,
        addressables: vec![],
        parameters: None,
    };
    static ADDRESSABLE: Addressable = Addressable {
        name: String::new(),
        description: None,
        base: String::new(),
        size: String::new(),
        registers: None,
        read: None,
        write: None,
        execute: None,
        word_size: None,
        default_register_class: String::new(),
    };
    static REGISTER: Register = Register {
        name: String::new(),
        description: None,
        class: None,
        offset: String::new(),
        width: None,
        values: None,
        fields: None,
        indexing: None,
    };
    static FIELD: Field = Field {
        name: String::new(),
        description: None,
        position: 0,
        length: None,
        values: None,
    };

    #[test]
    fn test_field_resolve_position() {
        let machine = &MACHINE;
        let peripheral = &PERIPHERAL;
        let addressable = &ADDRESSABLE;
        let register = &REGISTER;
        let mut field = FIELD.clone();

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
            126
        );
    }

    #[test]
    fn test_field_resolve_length() {
        let machine = &MACHINE;
        let peripheral = &PERIPHERAL;
        let addressable = &ADDRESSABLE;
        let register = &REGISTER;
        let mut field = FIELD.clone();

        assert_eq!(
            field.resolve_length(&register, &addressable, &peripheral, &machine),
            128
        );
        field.position = 16;
        assert_eq!(
            field.resolve_length(&register, &addressable, &peripheral, &machine),
            112
        );
        field.length = Some(8);
        assert_eq!(
            field.resolve_length(&register, &addressable, &peripheral, &machine),
            8
        );
    }

    #[test]
    fn test_indexing_resolve_stride() {
        let machine = &MACHINE;
        let peripheral = &PERIPHERAL;
        let addressable = &ADDRESSABLE;
        let mut register = REGISTER.clone();

        register.indexing = Some(Indexing {
            stride: Some(String::from("8")),
            lower_bound: None,
            upper_bound: String::from("8"),
        });

        assert_eq!(
            register.indexing.as_ref().unwrap().resolve_stride(
                &register,
                &addressable,
                &peripheral,
                &machine
            ),
            8
        );

        match &mut register.indexing {
            Some(indexing) => indexing.stride = None,
            None => {}
        }

        assert_eq!(
            register.indexing.as_ref().unwrap().resolve_stride(
                &register,
                &addressable,
                &peripheral,
                &machine
            ),
            16
        );
    }

    #[test]
    fn test_register_resolve_width() {
        let machine = &MACHINE;
        let mut peripheral = PERIPHERAL.clone();
        let mut addressable = ADDRESSABLE.clone();
        let mut register = REGISTER.clone();

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

    #[test]
    fn test_register_resolve_class() {
        let mut addressable = ADDRESSABLE.clone();
        let mut register = REGISTER.clone();

        addressable.default_register_class = String::from("control_status");

        register.class = Some(String::from("memory_mapped"));
        assert_eq!(register.resolve_class(&addressable), "memory_mapped");

        register.class = None;
        assert_eq!(register.resolve_class(&addressable), "control_status");
    }
}
