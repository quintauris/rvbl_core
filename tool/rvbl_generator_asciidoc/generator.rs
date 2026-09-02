// Copyright 2026 Quintauris GmbH
// Licensed under the Apache License, Version 2.0 (the "License").
// https://www.apache.org/licenses/LICENSE-2.0

use std::fs::File;
use std::io::{BufWriter, Result, Write};
use std::path::PathBuf;

use rvbl_model::interface::Generator;
use rvbl_model::model;

use crate::templates;

pub struct AsciiDocGenerator {
    memory_regions_header: Vec<&'static str>,
    parameters_header: Vec<&'static str>,
    values_header: Vec<&'static str>,
    default: String,
}

impl Default for AsciiDocGenerator {
    fn default() -> Self {
        AsciiDocGenerator {
            memory_regions_header: vec!["Memory Region", "Base", "Size"],
            parameters_header: vec!["Parameter", "Stage", "Type", "Value", "Description"],
            values_header: vec!["Value Name", "Value"],
            default: String::from("_"),
        }
    }
}

impl AsciiDocGenerator {
    fn cpu(self: &AsciiDocGenerator, cpu: &model::Cpu, writer: &mut BufWriter<File>) -> Result<()> {
        templates::section(writer, 2, format!("CPU {}", cpu.name).as_str())?;
        templates::paragraph_optional(writer, &cpu.description)?;
        templates::new_line(writer)?;

        for register_set in cpu.register_sets.iter() {
            self.registers(&register_set, cpu, writer, 3)?;
        }

        Ok(())
    }

    fn memory_regions(
        self: &AsciiDocGenerator,
        memory_regions: &Vec<model::MemoryRegion>,
        cpu: &model::Cpu,
        writer: &mut BufWriter<File>,
        level: usize,
    ) -> Result<()> {
        templates::table_header(writer, &self.memory_regions_header)?;

        for memory_region in memory_regions {
            let cells = vec![
                memory_region.name.as_str(),
                memory_region.base.as_str(),
                memory_region.size.as_str(),
            ];

            templates::table_row(writer, &cells)?;
        }

        templates::table_footer(writer)?;

        for memory_region in memory_regions {
            if memory_region.description.is_some() || memory_region.registers.is_some() {
                templates::section(
                    writer,
                    level,
                    format!("Memory Region {}", memory_region.name).as_str(),
                )?;
                templates::paragraph_optional(writer, &memory_region.description)?;
                templates::new_line(writer)?;

                if memory_region.registers.is_some() {
                    self.registers(
                        memory_region.registers.as_ref().unwrap(),
                        cpu,
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
            let cells = vec![
                parameter.name.as_str(),
                parameter.stage.as_str(),
                parameter.class.as_str(),
                parameter.value.as_ref().unwrap_or(&self.default).as_str(),
                parameter
                    .description
                    .as_ref()
                    .unwrap_or(&self.default)
                    .as_str(),
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
        cpu: &model::Cpu,
        writer: &mut BufWriter<File>,
        level: usize,
    ) -> Result<()> {
        for register in registers {
            templates::section(
                writer,
                level,
                format!("Register {}", register.name).as_str(),
            )?;
            templates::paragraph_optional(writer, &register.description)?;
            templates::horizontal_list(
                writer,
                &vec![
                    ("Type", register.class.as_str()),
                    ("Offset", register.offset.as_str()),
                    (
                        "Width (bits)",
                        register.resolve_width(cpu).to_string().as_str(),
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
                                .resolve_stride(register, cpu)
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
                    templates::paragraph_optional(writer, &field.description)?;
                    templates::horizontal_list(
                        writer,
                        &vec![
                            (
                                "Position (bits)",
                                field.resolve_position(register, cpu).to_string().as_str(),
                            ),
                            (
                                "Length (bits)",
                                field.resolve_length(register, cpu).to_string().as_str(),
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

    fn device(
        self: &AsciiDocGenerator,
        device: &model::Peripheral,
        cpu: &model::Cpu,
        writer: &mut BufWriter<File>,
    ) -> Result<()> {
        templates::section(writer, 2, format!("Device {}", device.name).as_str())?;
        templates::paragraph_optional(writer, &device.description)?;
        templates::new_line(writer)?;

        if !device.memory_regions.is_empty() {
            self.memory_regions(&device.memory_regions, cpu, writer, 3)?;
        }

        if device.parameters.is_some() {
            templates::section(writer, 3, "Parameters")?;
            self.parameters(&device.parameters.as_ref().unwrap(), writer, 3)?;
        }

        Ok(())
    }

    #[allow(unused)]
    pub fn generate_cpu(&self, cpu: &model::Cpu) -> Result<()> {
        let file = File::create("model.adoc")?;
        let mut writer = BufWriter::new(file);

        self.cpu(&cpu, &mut writer)
    }

    #[allow(unused)]
    pub fn generate_device(&self, device: &model::Peripheral) -> Result<()> {
        let file = File::create("model.adoc")?;
        let mut writer = BufWriter::new(file);
        let cpu = model::Cpu {
            name: String::from("Default"),
            description: None,
            xlen: 32,
            register_sets: vec![],
        };

        self.device(device, &cpu, &mut writer)
    }

    #[allow(unused)]
    pub fn generate_register_set(&self, registers: &Vec<model::Register>) -> Result<()> {
        let file = File::create("model.adoc")?;
        let mut writer = BufWriter::new(file);
        let cpu = model::Cpu {
            name: String::from("Default"),
            description: None,
            xlen: 32,
            register_sets: vec![],
        };

        self.registers(registers, &cpu, &mut writer, 1)
    }
}

impl Generator for AsciiDocGenerator {
    fn generate(&self, machine: &model::Machine) -> Result<()> {
        let file = File::create("model.adoc").unwrap();
        let mut writer = BufWriter::new(file);

        templates::section(&mut writer, 1, format!("Machine {}", machine.name).as_str())?;

        templates::paragraph(&mut writer, &r"Quintauris GmbH".to_string())?;
        templates::new_line(&mut writer)?;

        templates::paragraph_optional(&mut writer, &machine.description)?;
        templates::new_line(&mut writer)?;

        for cpu in machine.cpus.iter() {
            self.cpu(cpu, &mut writer)?;
        }

        self.memory_regions(
            &machine.memory_regions,
            &machine.cpus.first().unwrap(),
            &mut writer,
            2,
        )?;

        for device in machine.peripherals.iter() {
            self.device(device, machine.cpus.first().unwrap(), &mut writer)?;
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
