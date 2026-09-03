// Copyright 2026 Quintauris GmbH
// Licensed under the Apache License, Version 2.0 (the "License").
// https://www.apache.org/licenses/LICENSE-2.0

use crate::templates;
use rvbl_model::evaluator::{parse_expression, parse_expression_option};
use rvbl_model::{interface::Generator, model};
use std::collections::HashSet;
use std::fs::{File, create_dir_all};
use std::io::{BufWriter, Result, Write};
use std::path::{Path, PathBuf};

const DEFAULT_PARAMETER_VALUE: &'static str = "0";

fn machine_header(machine: &model::Machine) -> Result<()> {
    let file = File::create("include/rvbl/machine/rvbl_machine.h").unwrap();
    let mut writer = BufWriter::new(file);
    let device_types: HashSet<String> = machine
        .peripherals
        .iter()
        .map(|x| x.name.clone())
        .collect::<HashSet<String>>();

    templates::header_guard_begin(&mut writer, machine.name.as_str())?;
    templates::documentation_block(
        &mut writer,
        format!("= Machine {} =\nQuintauris GmbH\n:toc: left", machine.name).as_str(),
    )?;
    templates::new_line(&mut writer)?;

    templates::include(&mut writer, Path::new("rvbl_configuration.h"))?;
    templates::new_line(&mut writer)?;

    for cpu in machine.cpus.iter() {
        let name = &cpu.name;

        templates::documentation_block(
            &mut writer,
            format!("include::rvbl_{name}.adoc[]").as_str(),
        )?;
        templates::include(&mut writer, Path::new(format!("rvbl_{name}.h").as_str()))?;
    }

    templates::new_line(&mut writer)?;

    for memory_region in machine.memory_map.iter() {
        templates::define(
            &mut writer,
            format!("rvbl_memory_region_{}_base", &memory_region.name).as_str(),
            format!("0x{:x}", parse_expression(memory_region.base.as_str())).as_str(),
        )?;
        templates::define(
            &mut writer,
            format!("rvbl_memory_region_{}_size", &memory_region.name).as_str(),
            format!("{}", parse_expression(&memory_region.size)).as_str(),
        )?;
    }

    templates::new_line(&mut writer)?;

    for device_type in device_types.iter() {
        templates::documentation_block(
            &mut writer,
            format!("include::rvbl_{device_type}.adoc[]").as_str(),
        )?;
        templates::include(
            &mut writer,
            Path::new(format!("rvbl_{device_type}.h").as_str()),
        )?;
    }

    templates::new_line(&mut writer)?;

    for device in machine.peripherals.iter() {
        let type_ = &device.name;
        let instance = &device.instance;

        templates::extern_const_struct_declaration(
            &mut writer,
            format!("rvbl_{type_}_t",).as_str(),
            format!("rvbl_{type_}_instance_{instance}").as_str(),
        )?;
    }

    templates::header_guard_end(&mut writer, machine.name.as_str())?;

    writer.flush()?;

    Ok(())
}

fn configuration_header(machine: &model::Machine) -> Result<()> {
    let file = File::create("include/rvbl/machine/rvbl_configuration.h").unwrap();
    let mut writer = BufWriter::new(file);

    templates::header_guard_begin(
        &mut writer,
        format!("configuration_{}", machine.name).as_str(),
    )?;
    templates::new_line(&mut writer)?;

    templates::documentation_block(
        &mut writer,
        format!(
            "= Configuration of machine {} =\nQuintauris GmbH\n:toc: left",
            machine.name
        )
        .as_str(),
    )?;
    templates::new_line(&mut writer)?;

    templates::include(&mut writer, Path::new("rvbl/type/rvbl_types.h"))?;
    templates::new_line(&mut writer)?;

    for parameter in machine.parameters.iter() {
        match parameter.stage.as_str() {
            "compile" => templates::machine_compile_parameter(
                &mut writer,
                "=".repeat(2).as_str(),
                parameter.name.as_str(),
                machine.name.as_str(),
                parameter.class.as_str(),
                parse_expression_option(&parameter.value)
                    .to_string()
                    .as_str(),
            )?,
            "link" => templates::machine_link_parameter(
                &mut writer,
                "=".repeat(2).as_str(),
                parameter.name.as_str(),
                machine.name.as_str(),
                parameter.class.as_str(),
            )?,
            "run" => templates::machine_run_parameter(
                &mut writer,
                "=".repeat(2).as_str(),
                parameter.name.as_str(),
                machine.name.as_str(),
                parameter.class.as_str(),
            )?,
            _ => (),
        }
    }

    templates::new_line(&mut writer)?;
    templates::header_guard_end(&mut writer, machine.name.as_str())?;

    writer.flush()?;

    Ok(())
}

fn machine_instances_source(machine: &model::Machine) -> Result<()> {
    let file = File::create("source/rvbl_machine_instances.c").unwrap();
    let mut writer = BufWriter::new(file);

    templates::include(&mut writer, Path::new("rvbl/machine/rvbl_machine.h"))?;
    templates::new_line(&mut writer)?;

    for device in machine.peripherals.iter() {
        let type_ = &device.name;
        let instance = &device.instance;
        let mut initializers = device
            .addressables
            .iter()
            .map(|x| parse_expression(&x.base))
            .collect::<Vec<i64>>();

        if device.parameters.is_some() {
            initializers.extend(device.parameters.as_ref().unwrap().iter().map(|x| {
                parse_expression(
                    &x.value
                        .as_ref()
                        .unwrap_or(&DEFAULT_PARAMETER_VALUE.to_string()),
                )
            }));
        }

        let strings = initializers
            .iter()
            .map(|x| format!("0x{:08x}", x))
            .collect::<Vec<String>>();

        templates::const_struct_definition(
            &mut writer,
            format!("rvbl_{type_}_t").as_str(),
            format!("rvbl_{type_}_instance_{instance}").as_str(),
            &strings.iter().map(|x| x.as_str()).collect(),
        )?;
    }

    writer.flush()?;

    Ok(())
}

fn device_header(device: &model::Peripheral, machine: &model::Machine) -> Result<()> {
    let device_type = format!("rvbl_{}_t", device.name);
    let file = File::create(format!("include/rvbl/machine/rvbl_{}.h", device.name)).unwrap();
    let mut writer = BufWriter::new(file);
    let mut initializers = device
        .addressables
        .iter()
        .map(|m| {
            (
                "rvbl_uword_t".to_string(),
                format!("{}_region_address", m.name),
            )
        })
        .collect::<Vec<(String, String)>>();

    templates::header_guard_begin(
        &mut writer,
        format!("rvbl_device_{}", &device.name).as_str(),
    )?;
    templates::new_line(&mut writer)?;

    templates::include(&mut writer, Path::new("rvbl/type/rvbl_types.h"))?;
    templates::new_line(&mut writer)?;

    templates::documentation_block(&mut writer, format!("== Device `{}`", device.name).as_str())?;
    templates::new_line(&mut writer)?;

    if device.parameters.is_some() {
        initializers.extend(device.parameters.as_ref().unwrap().iter().map(|p| {
            (
                format!("rvbl_{}_t", p.class),
                format!("{}_parameter", p.name),
            )
        }));
    }

    templates::structure(
        &mut writer,
        device_type.as_str(),
        &initializers
            .iter()
            .map(|s| (s.0.as_str(), s.1.as_str()))
            .collect::<Vec<(&str, &str)>>(),
    )?;

    for memory_region in device.addressables.iter() {
        if memory_region.registers.is_some() {
            for register in memory_region.registers.as_ref().unwrap().iter() {
                let mut register_type = format!(
                    "rvbl_uint{}_t",
                    register.resolve_width(&machine.cpus.first().unwrap())
                );
                let mut indexing_parameter = String::from("");
                let mut indexing_check = String::from("");
                let mut indexing_operation = String::from("");
                let mut indexing_value = String::from("");
                let mut indexing_comment = String::from("");

                templates::documentation_block(
                    &mut writer,
                    format!("=== Register `{}`", register.name).as_str(),
                )?;
                templates::new_line(&mut writer)?;

                if register.values.is_some() {
                    register_type = format!("{}_{}_values", device.name, register.name);
                    templates::enumeration(
                        &mut writer,
                        "=".repeat(4).as_str(),
                        register_type.as_str(),
                        &register
                            .values
                            .as_ref()
                            .unwrap()
                            .iter()
                            .map(|v| (v.name.as_str(), v.value.as_str()))
                            .collect::<Vec<(&str, &str)>>(),
                    )?;
                }

                if register.indexing.is_some() {
                    let indexing: &model::Indexing = register.indexing.as_ref().unwrap();

                    indexing_parameter = String::from(", const rvbl_uword_t index");
                    indexing_check.push_str("if (");

                    if indexing.lower_bound.is_some()
                        && indexing.lower_bound.as_ref().unwrap().as_str() > "0"
                    {
                        indexing_check.push_str(&format!(
                            "(index >= {}) && ",
                            indexing.lower_bound.as_ref().unwrap()
                        ));
                    }

                    indexing_check.push_str(&format!("(index <= {}))", indexing.upper_bound));
                    indexing_operation = format!(
                        " + (index*{})",
                        indexing.resolve_stride(register, machine.cpus.first().as_ref().unwrap())
                    );
                    indexing_value = String::from(", index");
                    indexing_comment = String::from("`const rvbl_uword_t`:: (in) Register index.")
                }

                templates::memory_mapped_register(
                    &mut writer,
                    "=".repeat(4).as_str(),
                    format!(
                        "{}_{}",
                        device.name.to_uppercase(),
                        memory_region.name.to_uppercase()
                    )
                    .as_str(),
                    format!("{}_{}", device.name, register.name).as_str(),
                    register_type.as_str(),
                    register.offset.as_str(),
                    memory_region.name.as_str(),
                    device_type.as_str(),
                    indexing_parameter.as_str(),
                    indexing_check.as_str(),
                    indexing_operation.as_str(),
                    indexing_comment.as_str(),
                )?;

                if register.fields.is_some() {
                    for field in register.fields.as_ref().unwrap().iter() {
                        let mut field_type = format!(
                            "rvbl_uint{}_t",
                            register.resolve_width(&machine.cpus.first().unwrap())
                        );

                        if field.values.is_some() {
                            field_type =
                                format!("{}_{}_{}_values", device.name, register.name, field.name);
                            templates::enumeration(
                                &mut writer,
                                "=".repeat(5).as_str(),
                                field_type.as_str(),
                                &field
                                    .values
                                    .as_ref()
                                    .unwrap()
                                    .iter()
                                    .map(|v| (v.name.as_str(), v.value.as_str()))
                                    .collect::<Vec<(&str, &str)>>(),
                            )?;
                        }

                        templates::memory_mapped_register_field(
                            &mut writer,
                            "=".repeat(5).as_str(),
                            format!("{}_{}", device.name, register.name).as_str(),
                            field.name.as_str(),
                            field_type.as_str(),
                            field.resolve_position(register, &machine.cpus.first().unwrap()),
                            field.resolve_length(register, &machine.cpus.first().unwrap()),
                            device_type.as_str(),
                            indexing_parameter.as_str(),
                            indexing_value.as_str(),
                            indexing_comment.as_str(),
                        )?;
                    }
                }
            }
        }
    }

    templates::new_line(&mut writer)?;
    templates::header_guard_end(
        &mut writer,
        format!("rvbl_device_{}", &device.name).as_str(),
    )?;

    writer.flush()?;

    Ok(())
}

fn cpu_header(cpu: &model::Cpu) -> Result<()> {
    let file = File::create(format!("include/rvbl/machine/rvbl_{}.h", cpu.name)).unwrap();
    let mut writer = BufWriter::new(file);

    templates::header_guard_begin(&mut writer, format!("rvbl_cpu_{}", &cpu.name).as_str())?;
    templates::new_line(&mut writer)?;

    templates::include(&mut writer, Path::new("rvbl/type/rvbl_types.h"))?;
    templates::new_line(&mut writer)?;

    templates::documentation_block(&mut writer, format!("== CPU `{}`", cpu.name).as_str())?;
    templates::new_line(&mut writer)?;

    for register_set in cpu.register_sets.iter() {
        for register in register_set.iter() {
            let mut register_type = format!("rvbl_uint{}_t", register.resolve_width(cpu));

            templates::documentation_block(
                &mut writer,
                format!("=== Register `{}`", register.name).as_str(),
            )?;
            templates::new_line(&mut writer)?;

            if register.values.is_some() {
                register_type = format!("{}_values", register.name);
                templates::enumeration(
                    &mut writer,
                    "=".repeat(4).as_str(),
                    register_type.as_str(),
                    &register
                        .values
                        .as_ref()
                        .unwrap()
                        .iter()
                        .map(|v| (v.name.as_str(), v.value.as_str()))
                        .collect::<Vec<(&str, &str)>>(),
                )?;
            }

            templates::control_status_register(
                &mut writer,
                "=".repeat(4).as_str(),
                register.name.as_str(),
                register_type.as_str(),
                parse_expression(&register.offset),
            )?;
            templates::new_line(&mut writer)?;

            if register.fields.is_some() {
                for field in register.fields.as_ref().unwrap().iter() {
                    let mut field_type = format!("rvbl_uint{}_t", register.resolve_width(cpu));

                    if field.values.is_some() {
                        field_type = format!("{}_{}_values", register.name, field.name);
                        templates::enumeration(
                            &mut writer,
                            "=".repeat(5).as_str(),
                            field_type.as_str(),
                            &field
                                .values
                                .as_ref()
                                .unwrap()
                                .iter()
                                .map(|v| (v.name.as_str(), v.value.as_str()))
                                .collect::<Vec<(&str, &str)>>(),
                        )?;
                    }

                    templates::control_status_register_field(
                        &mut writer,
                        "=".repeat(5).as_str(),
                        register.name.as_str(),
                        register_type.as_str(),
                        field.name.as_str(),
                        field_type.as_str(),
                        field.resolve_position(register, cpu),
                        field.resolve_length(register, cpu),
                    )?;

                    match field.resolve_length(register, cpu) {
                        1 => templates::control_status_register_field_bits_fast(
                            &mut writer,
                            "=".repeat(5).as_str(),
                            register.name.as_str(),
                            parse_expression(&register.offset),
                            field.name.as_str(),
                            field.resolve_position(register, cpu),
                            field.resolve_length(register, cpu),
                        ),
                        _ => templates::control_status_register_field_bits_slow(
                            &mut writer,
                            "=".repeat(5).as_str(),
                            register.name.as_str(),
                            field.name.as_str(),
                            field.resolve_position(register, cpu),
                            field.resolve_length(register, cpu),
                        ),
                    }?
                }
            }
        }
    }

    templates::new_line(&mut writer)?;
    templates::header_guard_end(&mut writer, format!("rvbl_device_{}", &cpu.name).as_str())?;

    writer.flush()?;

    Ok(())
}

pub struct CGenerator {}

impl Generator for CGenerator {
    fn generate(&self, machine: &model::Machine) -> Result<()> {
        create_dir_all("include/rvbl/machine")?;
        create_dir_all("source")?;

        machine_header(machine)?;
        configuration_header(machine)?;
        machine_instances_source(machine)?;

        for cpu in machine.cpus.iter() {
            cpu_header(cpu)?
        }

        for device in machine.peripherals.iter() {
            device_header(device, machine)?
        }

        Ok(())
    }

    fn byproducts(&self, machine: &model::Machine) -> Vec<PathBuf> {
        let mut paths: HashSet<String> = [].into();

        paths.insert("include/rvbl/machine/rvbl_machine.h".to_string());
        paths.insert("include/rvbl/machine/rvbl_configuration.h".to_string());
        paths.insert("source/rvbl_machine_instances.c".to_string());

        for cpu in machine.cpus.iter() {
            paths.insert(format!("include/rvbl/machine/rvbl_{}.h", cpu.name));
        }

        for device in machine.peripherals.iter() {
            paths.insert(format!("include/rvbl/machine/rvbl_{}.h", device.name));
        }

        return paths.iter().map(|x| PathBuf::from(x)).collect();
    }
}
