// Copyright 2026 Quintauris GmbH
// Licensed under the Apache License, Version 2.0 (the "License").
// https://www.apache.org/licenses/LICENSE-2.0

use crate::templates;
use rvbl_model::evaluator::{parse_expression, parse_expression_option};
use rvbl_model::{interface::Generator, model};
use std::collections::HashSet;
use std::fs::{File, create_dir_all};
use std::io::{BufWriter, Error, ErrorKind, Result, Write};
use std::path::{Path, PathBuf};
use std::{format, vec};

const DEFAULT_PARAMETER_VALUE: &'static str = "0";

fn machine_header(machine: &model::Machine) -> Result<()> {
    let file = File::create("include/rvbl/machine/rvbl_machine.h").unwrap();
    let mut writer = BufWriter::new(file);
    let peripheral_types: HashSet<String> = machine
        .peripherals
        .iter()
        .filter_map(|r| r.dereference().ok())
        .map(|p| p.name.clone())
        .collect::<HashSet<String>>();

    templates::header_guard_begin(&mut writer, machine.name.as_str())?;
    templates::documentation_block(
        &mut writer,
        format!("= Machine {} =\nQuintauris GmbH\n:toc: left", machine.name).as_str(),
    )?;
    templates::new_line(&mut writer)?;

    templates::include(&mut writer, Path::new("rvbl_configuration.h"))?;
    templates::new_line(&mut writer)?;
    templates::new_line(&mut writer)?;

    for addressable in machine
        .memory_map
        .iter()
        .filter_map(|r| r.dereference().ok())
    {
        templates::define(
            &mut writer,
            &format!("rvbl_memory_region_{}_base", &addressable.name),
            &format!("0x{:x}", parse_expression(&addressable.base)),
        )?;
        templates::define(
            &mut writer,
            &format!("rvbl_memory_region_{}_size", &addressable.name),
            &format!("{}", parse_expression(&addressable.size)),
        )?;
    }

    templates::new_line(&mut writer)?;

    for peripheral_type in peripheral_types.iter() {
        templates::documentation_block(
            &mut writer,
            format!("include::rvbl_{peripheral_type}.adoc[]").as_str(),
        )?;
        templates::include(
            &mut writer,
            Path::new(format!("rvbl_{peripheral_type}.h").as_str()),
        )?;
    }

    templates::include(&mut writer, Path::new("rvbl/hardware/rvbl_hardware.h"))?;

    templates::new_line(&mut writer)?;

    for peripheral in machine
        .peripherals
        .iter()
        .filter_map(|r| r.dereference().ok())
    {
        let type_ = &peripheral.name;
        let instance = &peripheral.instance;

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

    for peripheral in machine
        .peripherals
        .iter()
        .filter_map(|r| r.dereference().ok())
    {
        let type_ = &peripheral.name;
        let instance = &peripheral.instance;
        let mut initializers = peripheral
            .addressables
            .iter()
            .filter_map(|r| r.dereference().ok())
            .map(|x| parse_expression(&x.base))
            .collect::<Vec<i64>>();

        if peripheral.parameters.is_some() {
            initializers.extend(peripheral.parameters.as_ref().unwrap().iter().map(|x| {
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

fn peripheral_header_register(
    register: &model::Register,
    addressable: &model::Addressable,
    peripheral: &model::Peripheral,
    peripheral_type: &String,
    machine: &model::Machine,
    mut writer: &mut BufWriter<File>,
) -> Result<()> {
    let mut bit_fields: Vec<(String, String, u32, u32)> = vec![];
    let register_integer_type = format!(
        "rvbl_uint{}_t",
        register.resolve_width(addressable, peripheral, machine)
    );
    let mut register_internal_type = register_integer_type.clone();
    let mut register_internal_type_prefix = String::new();
    let register_namespace = format!(
        "rvbl_{}_{}_{}",
        peripheral.name, addressable.name, register.name
    );
    let register_type = format!("{register_namespace}_t");
    let register_width = register.resolve_width(addressable, peripheral, machine);

    let mut indexing_lower_bound: i64 = 0;
    let mut indexing_upper_bound = String::from("0");
    let mut indexing_stride: i64 = (register_width / 8).into();

    if let Some(indexing) = &register.indexing {
        indexing_lower_bound = parse_expression_option(&indexing.lower_bound);
        indexing_upper_bound = indexing.upper_bound.clone();
        indexing_stride = match &indexing.stride {
            Some(stride) => parse_expression(&stride),
            None => indexing_stride,
        }
    }

    templates::documentation_block(
        &mut writer,
        format!("=== Register `{}`", register.name).as_str(),
    )?;
    templates::new_line(&mut writer)?;

    if register.values.is_some() {
        register_internal_type = format!("{register_namespace}_values_t");
        register_internal_type_prefix = String::from("enum");
        templates::enumeration(
            &mut writer,
            "=".repeat(4).as_str(),
            &register_internal_type,
            &format!("{register_namespace}_values"),
            &register
                .values
                .as_ref()
                .unwrap()
                .iter()
                .map(|v| (v.name.as_str(), v.value.as_str()))
                .collect::<Vec<(&str, &str)>>(),
        )?;
    }

    if let Some(fields) = &register.fields {
        for field in fields {
            let mut field_type = register_internal_type.clone();
            let mut field_type_prefix = register_internal_type_prefix.clone();
            let field_length = field.resolve_length(register, addressable, peripheral, machine);
            let field_position = field.resolve_position(register, addressable, peripheral, machine);
            let field_namespace = format!("{register_namespace}_{}", field.name);

            if field.values.is_some() {
                field_type = format!("{field_namespace}_t");
                field_type_prefix = String::from("enum");

                templates::enumeration(
                    &mut writer,
                    "=".repeat(4).as_str(),
                    &field_type,
                    &format!("{field_namespace}_values"),
                    &field
                        .values
                        .as_ref()
                        .unwrap()
                        .iter()
                        .map(|v| (v.name.as_str(), v.value.as_str()))
                        .collect::<Vec<(&str, &str)>>(),
                )?;
            }

            bit_fields.push((
                format!("{field_type_prefix} {field_type}"),
                field.name.clone(),
                field_position,
                field_length,
            ));
        }

        {
            let mut bit_fields_with_unused: Vec<(String, String, u32, u32)> = vec![];
            let mut i = 0;

            if let Some(first) = bit_fields.first()
                && first.2 > 0
            {
                bit_fields_with_unused.push((
                    String::from("rvbl_uword_t"),
                    format!("unused_{i}"),
                    0,
                    first.2,
                ));
                i += 1;
            }

            for pair in bit_fields.windows(2) {
                if let Some(a) = pair.first()
                    && let Some(b) = pair.last()
                {
                    let a_end = a.2 + a.3;
                    let b_begin = b.2;
                    bit_fields_with_unused.push(a.clone());

                    if a_end < b_begin {
                        bit_fields_with_unused.push((
                            String::from("rvbl_uword_t"),
                            format!("unused_{i}"),
                            a_end,
                            b_begin - a_end,
                        ));
                        i += 1;
                    }
                }
            }

            if let Some(last) = bit_fields.last() {
                bit_fields_with_unused.push(last.clone());

                if last.2 + last.3 < register_width {
                    bit_fields_with_unused.push((
                        String::from("rvbl_uword_t"),
                        format!("unused_{i}"),
                        last.2 + last.3,
                        register_width - (last.2 + last.3),
                    ));
                }
            }

            bit_fields = bit_fields_with_unused;
        }

        templates::bitfield(
            writer,
            &format!("{register_namespace}_fields_t"),
            &bit_fields
                .iter()
                .map(|f| (f.0.as_str(), f.1.as_str(), f.3))
                .collect(),
        )?;

        register_internal_type = format!("{register_namespace}_union_t");
        register_internal_type_prefix = String::from("union");

        templates::union(
            writer,
            &register_internal_type,
            &vec![
                (&format!("struct {register_namespace}_fields_t"), "f"),
                (&register_integer_type, "w"),
                (&register_integer_type, "s"),
            ],
        )?;
    }

    templates::typedef(
        writer,
        &format!("{register_internal_type_prefix} {register_internal_type}"),
        &register_type,
    )?;

    match register.resolve_class(addressable).as_str() {
        "memory_mapped" => templates::memory_mapped_register(
            &mut writer,
            "=".repeat(4).as_str(),
            &peripheral.name,
            &peripheral_type,
            &addressable.name,
            &register.name,
            &register_type,
            &register.offset,
            register.fields.is_some(),
            indexing_lower_bound,
            &indexing_upper_bound,
            indexing_stride,
        ),
        "control_status" => templates::control_status_register(
            &mut writer,
            "=".repeat(4).as_str(),
            &peripheral.name,
            &peripheral_type,
            &addressable.name,
            &register.name,
            &register_type,
            &register.offset,
            register.fields.is_some(),
            indexing_lower_bound,
            &indexing_upper_bound,
            indexing_stride,
        ),
        _ => Result::Err(Error::from(ErrorKind::InvalidInput)),
    }?;

    templates::new_line(&mut writer)?;

    Ok(())
}

fn peripheral_header(peripheral: &model::Peripheral, machine: &model::Machine) -> Result<()> {
    let peripheral_type = format!("rvbl_{}_t", peripheral.name);
    let file = File::create(format!("include/rvbl/machine/rvbl_{}.h", peripheral.name)).unwrap();
    let mut writer = BufWriter::new(file);
    let mut initializers = peripheral
        .addressables
        .iter()
        .filter_map(|r| r.dereference().ok())
        .map(|m| {
            (
                "rvbl_uword_t".to_string(),
                format!("{}_region_address", m.name),
            )
        })
        .collect::<Vec<(String, String)>>();

    templates::header_guard_begin(
        &mut writer,
        format!("rvbl_device_{}", &peripheral.name).as_str(),
    )?;
    templates::new_line(&mut writer)?;

    templates::include(&mut writer, Path::new("rvbl/type/rvbl_types.h"))?;
    templates::new_line(&mut writer)?;

    templates::documentation_block(
        &mut writer,
        format!("== Device `{}`", peripheral.name).as_str(),
    )?;
    templates::new_line(&mut writer)?;

    if peripheral.parameters.is_some() {
        initializers.extend(peripheral.parameters.as_ref().unwrap().iter().map(|p| {
            (
                format!("rvbl_{}_t", p.class),
                format!("{}_parameter", p.name),
            )
        }));
    }

    templates::structure(
        &mut writer,
        peripheral_type.as_str(),
        &initializers
            .iter()
            .map(|s| (s.0.as_str(), s.1.as_str()))
            .collect::<Vec<(&str, &str)>>(),
    )?;

    for addressable in peripheral
        .addressables
        .iter()
        .filter_map(|r| r.dereference().ok())
    {
        if addressable.registers.is_some() {
            for register in addressable.registers.as_ref().unwrap().iter() {
                match register.resolve_class(addressable).as_str() {
                    "memory_mapped" => peripheral_header_register(
                        register,
                        addressable,
                        peripheral,
                        &peripheral_type,
                        machine,
                        &mut writer,
                    ),
                    "control_status" => peripheral_header_register(
                        register,
                        addressable,
                        peripheral,
                        &peripheral_type,
                        machine,
                        &mut writer,
                    ),
                    _ => Result::Err(ErrorKind::InvalidInput.into()),
                }?
            }
        }
    }

    templates::new_line(&mut writer)?;
    templates::header_guard_end(
        &mut writer,
        format!("rvbl_device_{}", &peripheral.name).as_str(),
    )?;

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

        for peripheral in machine
            .peripherals
            .iter()
            .filter_map(|r| r.dereference().ok())
        {
            peripheral_header(&peripheral, machine)?
        }

        Ok(())
    }

    fn byproducts(&self, machine: &model::Machine) -> Vec<PathBuf> {
        let mut paths: HashSet<String> = [].into();

        paths.insert("include/rvbl/machine/rvbl_machine.h".to_string());
        paths.insert("include/rvbl/machine/rvbl_configuration.h".to_string());
        paths.insert("source/rvbl_machine_instances.c".to_string());

        for peripheral in machine
            .peripherals
            .iter()
            .filter_map(|r| r.dereference().ok())
        {
            paths.insert(format!("include/rvbl/machine/rvbl_{}.h", peripheral.name));
        }

        return paths.iter().map(|x| PathBuf::from(x)).collect();
    }
}
