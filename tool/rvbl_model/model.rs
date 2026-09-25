// Copyright 2026 Quintauris GmbH
// Licensed under the Apache License, Version 2.0 (the "License").
// https://www.apache.org/licenses/LICENSE-2.0

use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
};

use serde::{Deserialize, Serialize, de::DeserializeOwned};

use crate::{evaluator, loader};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Description {
    pub content_type: Option<String>,
    pub content: String,
}

impl Description {
    #[allow(unused)]
    pub fn resolve_content_type(&self) -> String {
        match &self.content_type {
            Some(t) => t.clone(),
            None => String::from("text/plain"),
        }
    }
}

#[allow(unused)]
pub trait Described {
    fn resolve_description(&self) -> Option<String>;
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Value {
    pub name: String,
    pub description: Option<Description>,
    pub value: String,
}

impl Described for Value {
    fn resolve_description(&self) -> Option<String> {
        match &self.description {
            Some(d) => Some(d.content.clone()),
            None => None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Field {
    pub name: String,
    pub description: Option<Description>,
    pub position: i32,
    pub length: Option<u32>,
    pub values: Option<Vec<Value>>,
    pub volatile: Option<bool>,
}

impl Described for Field {
    fn resolve_description(&self) -> Option<String> {
        match &self.description {
            Some(d) => Some(d.content.clone()),
            None => None,
        }
    }
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

    #[allow(unused)]
    pub fn resolve_volatility(&self) -> bool {
        return self.volatile.unwrap_or(false);
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
    pub description: Option<Description>,
    pub class: Option<String>,
    pub offset: String,
    pub width: Option<u32>,
    pub volatile: Option<bool>,
    pub values: Option<Vec<Value>>,
    pub fields: Option<Vec<Field>>,
    pub indexing: Option<Indexing>,
}

impl Described for Register {
    fn resolve_description(&self) -> Option<String> {
        match &self.description {
            Some(d) => Some(d.content.clone()),
            None => None,
        }
    }
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

    #[allow(unused)]
    pub fn resolve_volatility(&self) -> bool {
        let field_volatilities: Vec<bool> = self
            .fields
            .iter()
            .flatten()
            .filter_map(|f| f.volatile)
            .collect();

        if field_volatilities.is_empty() {
            self.volatile.unwrap_or(false)
        } else {
            field_volatilities.iter().any(|x| *x)
        }
    }
}

pub trait Resolve {
    fn resolve(&mut self, _: &PathBuf, _: &PathBuf) -> loader::Result<()> {
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Addressable {
    pub name: String,
    pub description: Option<Description>,
    pub base: String,
    pub size: String,
    pub registers: Option<Vec<Register>>,
    pub read: Option<bool>,
    pub write: Option<bool>,
    pub execute: Option<bool>,
    pub word_size: Option<u32>,
    pub default_register_class: String,
}

impl Described for Addressable {
    fn resolve_description(&self) -> Option<String> {
        match &self.description {
            Some(d) => Some(d.content.clone()),
            None => None,
        }
    }
}

impl Resolve for Addressable {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Parameter {
    pub name: String,
    pub description: Option<Description>,
    pub stage: String,
    pub class: String,
    pub value: Option<String>,
}

impl Described for Parameter {
    fn resolve_description(&self) -> Option<String> {
        match &self.description {
            Some(d) => Some(d.content.clone()),
            None => None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Import {
    pub file: String,
    pub name: String,
    pub description: Option<Description>,
    pub substitutions: Option<HashMap<String, String>>,
}

impl Described for Import {
    fn resolve_description(&self) -> Option<String> {
        match &self.description {
            Some(d) => Some(d.content.clone()),
            None => None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Reference {
    pub reference: String,
    pub substitutions: Option<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Referenceable<T> {
    Reference(Reference),
    Inplace(T),
}

impl<T: Clone + DeserializeOwned + Resolve> Referenceable<T> {
    fn lookup(
        reference: &Reference,
        imports: &Option<Vec<Import>>,
        meta_model_search_path: &PathBuf,
        search_path: &PathBuf,
    ) -> loader::Result<T> {
        match imports
            .iter()
            .flatten()
            .find(|i| i.name == reference.reference)
        {
            Some(import) => {
                let mut dependencies: HashSet<PathBuf> = [].into();
                let mut parameters = match &import.substitutions {
                    Some(s) => s.clone(),
                    None => HashMap::new(),
                };

                match &reference.substitutions {
                    Some(s) => {
                        parameters.extend(s.into_iter().map(|(k, v)| (k.clone(), v.clone())));
                    }
                    None => {}
                }

                loader::load_model::<T>(
                    &PathBuf::from(&import.file),
                    meta_model_search_path,
                    search_path,
                    &parameters,
                    &mut dependencies,
                )
            }
            None => Err(loader::Error::ValidationError(format!(
                "Import \"{}\" not found.",
                reference.reference
            ))),
        }
    }

    pub fn resolve(
        &mut self,
        imports: &Option<Vec<Import>>,
        meta_model_search_path: &PathBuf,
        include_search_path: &PathBuf,
    ) -> loader::Result<()> {
        match self {
            Referenceable::Reference(reference) => match Referenceable::<T>::lookup(
                reference,
                imports,
                meta_model_search_path,
                include_search_path,
            ) {
                Ok(t) => {
                    *self = Referenceable::<T>::Inplace(t);
                    Ok(())
                }
                Err(e) => Err(e),
            },
            Referenceable::Inplace(_) => Ok(()),
        }
    }

    #[allow(unused)]
    pub fn dereference(&self) -> loader::Result<&T> {
        match self {
            Referenceable::Reference(reference) => Err(loader::Error::ValidationError(
                String::from(format!("Broken reference: {}", reference.reference)),
            )),
            Referenceable::Inplace(value) => Ok(value),
        }
    }

    #[allow(unused)]
    pub fn dereference_mut(&mut self) -> loader::Result<&mut T> {
        match self {
            Referenceable::Reference(reference) => Err(loader::Error::ValidationError(
                String::from(format!("Broken reference: {}", reference.reference)),
            )),
            Referenceable::Inplace(value) => Ok(value),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Peripheral {
    pub name: String,
    pub description: Option<Description>,
    pub instance: String,
    pub word_size: Option<u32>,
    pub imports: Option<Vec<Import>>,
    pub addressables: Vec<Referenceable<Addressable>>,
    pub parameters: Option<Vec<Parameter>>,
}

impl Described for Peripheral {
    fn resolve_description(&self) -> Option<String> {
        match &self.description {
            Some(d) => Some(d.content.clone()),
            None => None,
        }
    }
}

impl Resolve for Peripheral {
    fn resolve(
        &mut self,
        meta_model_search_path: &PathBuf,
        include_search_path: &PathBuf,
    ) -> loader::Result<()> {
        for reference in &mut self.addressables {
            reference.resolve(&self.imports, meta_model_search_path, include_search_path)?;
        }

        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Machine {
    pub name: String,
    pub description: Option<Description>,
    pub word_size: u32,
    pub imports: Option<Vec<Import>>,
    pub memory_map: Vec<Referenceable<Addressable>>,
    pub peripherals: Vec<Referenceable<Peripheral>>,
    pub parameters: Vec<Parameter>,
}

impl Described for Machine {
    fn resolve_description(&self) -> Option<String> {
        match &self.description {
            Some(d) => Some(d.content.clone()),
            None => None,
        }
    }
}

impl Resolve for Machine {
    fn resolve(
        &mut self,
        meta_model_search_path: &PathBuf,
        include_search_path: &PathBuf,
    ) -> loader::Result<()> {
        for reference in &mut self.memory_map {
            reference.resolve(&self.imports, meta_model_search_path, include_search_path)?;
        }

        for reference in &mut self.peripherals {
            reference.resolve(&self.imports, meta_model_search_path, include_search_path)?;

            if let Ok(peripheral) = reference.dereference_mut() {
                peripheral.resolve(meta_model_search_path, include_search_path)?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static DESCRIPTION: Description = Description {
        content_type: None,
        content: String::new(),
    };
    static MACHINE: Machine = Machine {
        name: String::new(),
        description: None,
        word_size: 128,
        imports: None,
        memory_map: vec![],
        peripherals: vec![],
        parameters: vec![],
    };
    static PERIPHERAL: Peripheral = Peripheral {
        name: String::new(),
        description: None,
        instance: String::new(),
        word_size: None,
        imports: None,
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
        volatile: None,
        values: None,
        fields: None,
        indexing: None,
    };
    static FIELD: Field = Field {
        name: String::new(),
        description: None,
        position: 0,
        length: None,
        volatile: None,
        values: None,
    };

    #[test]
    fn test_description_resolve_content_type() {
        let mut description = DESCRIPTION.clone();

        assert_eq!(description.resolve_content_type(), "text/plain");

        description.content_type = Some(String::from("text/markdown"));
        assert_eq!(description.resolve_content_type(), "text/markdown");
    }

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
    fn test_field_resolve_volatility() {
        let mut field = FIELD.clone();

        assert_eq!(field.resolve_volatility(), false);

        field.volatile = Some(false);
        assert_eq!(field.resolve_volatility(), false);

        field.volatile = Some(true);
        assert_eq!(field.resolve_volatility(), true);
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

    #[test]
    fn test_register_resolve_volatility() {
        let mut register = REGISTER.clone();

        // No fields, no explicit volatility: defaults to non-volatile.
        assert_eq!(register.resolve_volatility(), false);

        // No fields: falls back to the register's own volatility.
        register.volatile = Some(true);
        assert_eq!(register.resolve_volatility(), true);

        // Fields present but none set their own volatility: still falls
        // back to the register's own volatility.
        let mut field_a = FIELD.clone();
        field_a.name = String::from("a");
        let mut field_b = FIELD.clone();
        field_b.name = String::from("b");
        register.fields = Some(vec![field_a.clone(), field_b.clone()]);
        assert_eq!(register.resolve_volatility(), true);

        register.volatile = Some(false);
        assert_eq!(register.resolve_volatility(), false);

        // At least one field is explicitly non-volatile, but none is
        // volatile: register-level volatility is ignored, result is false.
        field_a.volatile = Some(false);
        register.volatile = Some(true);
        register.fields = Some(vec![field_a.clone(), field_b.clone()]);
        assert_eq!(register.resolve_volatility(), false);

        // At least one field is explicitly volatile: register is volatile,
        // regardless of the register's own volatility.
        field_b.volatile = Some(true);
        register.volatile = Some(false);
        register.fields = Some(vec![field_a.clone(), field_b.clone()]);
        assert_eq!(register.resolve_volatility(), true);
    }
}
