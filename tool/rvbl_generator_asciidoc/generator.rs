// Copyright 2026 Quintauris GmbH
// Licensed under the Apache License, Version 2.0 (the "License").
// https://www.apache.org/licenses/LICENSE-2.0

use std::fs::File;
use std::io::{BufWriter, Result, Write};
use std::path::PathBuf;

use rvbl_model::interface::Generator;
use rvbl_model::model::{self, Addressable};

use crate::templates;

pub struct AsciiDocGenerator {
    addressables_header: Vec<&'static str>,
    parameters_header: Vec<&'static str>,
    values_header: Vec<&'static str>,
    default: String,
}

impl Default for AsciiDocGenerator {
    fn default() -> Self {
        AsciiDocGenerator {
            addressables_header: vec!["Addressable", "Base", "Size"],
            parameters_header: vec!["Parameter", "Stage", "Type", "Value", "Description"],
            values_header: vec!["Value Name", "Value"],
            default: String::from("_"),
        }
    }
}

impl AsciiDocGenerator {
    fn addressables(
        self: &AsciiDocGenerator,
        addressables: &Vec<model::Addressable>,
        peripheral: &model::Peripheral,
        machine: &model::Machine,
        writer: &mut BufWriter<File>,
        level: usize,
    ) -> Result<()> {
        templates::table_header(writer, &self.addressables_header)?;

        for addressable in addressables {
            let cells = vec![
                addressable.name.as_str(),
                addressable.base.as_str(),
                addressable.size.as_str(),
            ];

            templates::table_row(writer, &cells)?;
        }

        templates::table_footer(writer)?;

        for addressable in addressables {
            if addressable.description.is_some() || addressable.registers.is_some() {
                templates::section(
                    writer,
                    level,
                    format!("Addressable {}", addressable.name).as_str(),
                )?;
                templates::paragraph_optional(
                    writer,
                    &addressable
                        .description
                        .as_ref()
                        .map_or(None, |d| Some(d.content.clone())),
                )?;
                templates::new_line(writer)?;

                if addressable.registers.is_some() {
                    self.registers(
                        addressable.registers.as_ref().unwrap(),
                        addressable,
                        peripheral,
                        machine,
                        writer,
                        level + 1,
                    )?;
                }
            }
        }

        Ok(())
    }

    fn parameters(
        self: &AsciiDocGenerator,
        parameters: &Vec<model::Parameter>,
        writer: &mut BufWriter<File>,
        _level: usize,
    ) -> Result<()> {
        templates::table_header(writer, &self.parameters_header)?;

        for parameter in parameters.iter() {
            let description = parameter
                .description
                .as_ref()
                .map_or(None, |d| Some(d.content.clone()));
            let cells = vec![
                parameter.name.as_str(),
                parameter.stage.as_str(),
                parameter.class.as_str(),
                parameter.value.as_ref().unwrap_or(&self.default).as_str(),
                &description.as_ref().unwrap_or(&self.default),
            ];

            templates::table_row(writer, &cells)?;
        }

        templates::table_footer(writer)?;

        Ok(())
    }

    fn values(
        self: &AsciiDocGenerator,
        values: &Vec<model::Value>,
        writer: &mut BufWriter<File>,
        _level: usize,
    ) -> Result<()> {
        templates::table_header(writer, &self.values_header)?;

        for value in values.iter() {
            let cells = vec![value.name.as_str(), value.value.as_str()];

            templates::table_row(writer, &cells)?;
        }

        templates::table_footer(writer)?;

        Ok(())
    }

    fn registers(
        self: &AsciiDocGenerator,
        registers: &Vec<model::Register>,
        addressable: &model::Addressable,
        peripheral: &model::Peripheral,
        machine: &model::Machine,
        writer: &mut BufWriter<File>,
        level: usize,
    ) -> Result<()> {
        for register in registers {
            templates::section(
                writer,
                level,
                format!("Register {}", register.name).as_str(),
            )?;
            templates::paragraph_optional(
                writer,
                &register
                    .description
                    .as_ref()
                    .map_or(None, |d| Some(d.content.clone())),
            )?;
            templates::horizontal_list(
                writer,
                &vec![
                    ("Type", register.resolve_class(addressable).as_str()),
                    ("Offset", register.offset.as_str()),
                    (
                        "Width (bits)",
                        register
                            .resolve_width(addressable, peripheral, machine)
                            .to_string()
                            .as_str(),
                    ),
                    (
                        "Indexed",
                        match register.indexing {
                            Some(_) => "Yes",
                            None => "No",
                        },
                    ),
                ],
            )?;

            if register.indexing.is_some() {
                templates::horizontal_list(
                    writer,
                    &vec![
                        (
                            "Lower bound",
                            register
                                .indexing
                                .as_ref()
                                .unwrap()
                                .lower_bound
                                .as_ref()
                                .unwrap_or(&String::from("0"))
                                .as_str(),
                        ),
                        (
                            "Upper bound",
                            register.indexing.as_ref().unwrap().upper_bound.as_str(),
                        ),
                        (
                            "Stride",
                            register
                                .indexing
                                .as_ref()
                                .unwrap()
                                .resolve_stride(register, addressable, peripheral, machine)
                                .to_string()
                                .as_str(),
                        ),
                    ],
                )?;
            }

            templates::new_line(writer)?;

            if register.values.is_some() {
                self.values(register.values.as_ref().unwrap(), writer, level + 1)?;
            }
            templates::new_line(writer)?;

            if register.fields.is_some() {
                for field in register.fields.as_ref().unwrap().iter() {
                    templates::section(
                        writer,
                        level + 1,
                        format!("Field {}", field.name).as_str(),
                    )?;
                    templates::paragraph_optional(
                        writer,
                        &field
                            .description
                            .as_ref()
                            .map_or(None, |d| Some(d.content.clone())),
                    )?;
                    templates::horizontal_list(
                        writer,
                        &vec![
                            (
                                "Position (bits)",
                                field
                                    .resolve_position(register, addressable, peripheral, machine)
                                    .to_string()
                                    .as_str(),
                            ),
                            (
                                "Length (bits)",
                                field
                                    .resolve_length(register, addressable, peripheral, machine)
                                    .to_string()
                                    .as_str(),
                            ),
                        ],
                    )?;
                    templates::new_line(writer)?;

                    if field.values.is_some() {
                        self.values(field.values.as_ref().unwrap(), writer, level + 2)?;
                        templates::new_line(writer)?;
                    }
                }
            }
        }

        Ok(())
    }

    fn peripheral(
        self: &AsciiDocGenerator,
        peripheral: &model::Peripheral,
        machine: &model::Machine,
        writer: &mut BufWriter<File>,
    ) -> Result<()> {
        templates::section(
            writer,
            2,
            format!("Peripheral {}", peripheral.name).as_str(),
        )?;
        templates::paragraph_optional(
            writer,
            &peripheral
                .description
                .as_ref()
                .map_or(None, |d| Some(d.content.clone())),
        )?;
        templates::new_line(writer)?;

        if !peripheral.addressables.is_empty() {
            self.addressables(
                &peripheral
                    .addressables
                    .iter()
                    .filter_map(|r| r.dereference().ok())
                    .map(|a| a.clone())
                    .collect::<Vec<Addressable>>(),
                peripheral,
                machine,
                writer,
                3,
            )?;
        }

        if peripheral.parameters.is_some() {
            templates::section(writer, 3, "Parameters")?;
            self.parameters(&peripheral.parameters.as_ref().unwrap(), writer, 3)?;
        }

        Ok(())
    }

    #[allow(unused)]
    pub fn generate_peripheral(
        &self,
        peripheral: &model::Peripheral,
        machine: &model::Machine,
    ) -> Result<()> {
        let file = File::create("model.adoc")?;
        let mut writer = BufWriter::new(file);

        self.peripheral(peripheral, machine, &mut writer)
    }

    #[allow(unused)]
    pub fn generate_register_set(
        &self,
        registers: &Vec<model::Register>,
        addressable: &model::Addressable,
        peripheral: &model::Peripheral,
        machine: &model::Machine,
    ) -> Result<()> {
        let file = File::create("model.adoc")?;
        let mut writer = BufWriter::new(file);

        self.registers(registers, addressable, peripheral, machine, &mut writer, 1)
    }
}

impl Generator for AsciiDocGenerator {
    fn generate(&self, machine: &model::Machine) -> Result<()> {
        let file = File::create("model.adoc").unwrap();
        let mut writer = BufWriter::new(file);

        templates::section(&mut writer, 1, format!("Machine {}", machine.name).as_str())?;

        templates::paragraph(&mut writer, &r"Quintauris GmbH".to_string())?;
        templates::new_line(&mut writer)?;

        templates::paragraph_optional(
            &mut writer,
            &machine
                .description
                .as_ref()
                .map_or(None, |d| Some(d.content.clone())),
        )?;
        templates::new_line(&mut writer)?;

        for peripheral in machine
            .peripherals
            .iter()
            .filter_map(|r| r.dereference().ok())
        {
            self.peripheral(&peripheral, machine, &mut writer)?;
        }

        if !machine.parameters.is_empty() {
            templates::section(&mut writer, 2, "Parameters")?;
            self.parameters(&machine.parameters, &mut writer, 2)?;
        }

        writer.flush()?;

        Ok(())
    }

    fn byproducts(&self, _machine: &model::Machine) -> Vec<PathBuf> {
        return vec![PathBuf::from("model.adoc")];
    }
}
