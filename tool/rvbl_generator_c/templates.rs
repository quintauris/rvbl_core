// Copyright 2026 Quintauris GmbH
// Licensed under the Apache License, Version 2.0 (the "License").
// https://www.apache.org/licenses/LICENSE-2.0

use std::io::{Result, Write};
use std::path::Path;
use std::{fs::File, io::BufWriter};

use rvbl_model::evaluator;

pub fn header_guard_begin(writer: &mut BufWriter<File>, id: &str) -> Result<()> {
    let capitalized = id.to_uppercase();

    writeln!(
        writer,
        "#ifndef _{capitalized}_H_
#define _{capitalized}_H_"
    )?;

    Ok(())
}

pub fn header_guard_end(writer: &mut BufWriter<File>, id: &str) -> Result<()> {
    let capitalized = id.to_uppercase();

    writeln!(writer, "#endif /* _{capitalized}_H_ */")?;

    Ok(())
}

pub fn documentation_block(writer: &mut BufWriter<File>, text: &str) -> Result<()> {
    for line in text.split('\n') {
        writeln!(writer, "/// {line}")?;
    }

    Ok(())
}

pub fn new_line(writer: &mut BufWriter<File>) -> Result<()> {
    writeln!(writer, "")?;

    Ok(())
}

pub fn include(writer: &mut BufWriter<File>, path: &Path) -> Result<()> {
    let x = path.to_str().unwrap();

    writeln!(writer, r#"#include "{x}""#)?;

    Ok(())
}

pub fn define(writer: &mut BufWriter<File>, id: &str, value: &str) -> Result<()> {
    let capitalized = id.to_uppercase();

    writeln!(writer, r#"#define {capitalized} {value}"#)?;

    Ok(())
}

pub fn extern_const_struct_declaration(
    writer: &mut BufWriter<File>,
    type_: &str,
    name: &str,
) -> Result<()> {
    writeln!(writer, "extern const struct {type_} {name};")?;

    Ok(())
}

pub fn const_struct_definition(
    writer: &mut BufWriter<File>,
    type_: &str,
    name: &str,
    items: &Vec<&str>,
) -> Result<()> {
    let joined = items.join(", ");

    writeln!(writer, "const struct {type_} {name} = {{ {joined} }};")?;

    Ok(())
}

pub fn enumeration(
    writer: &mut BufWriter<File>,
    level: &str,
    name: &str,
    item_prefix: &str,
    items: &Vec<(&str, &str)>,
) -> Result<()> {
    documentation_block(writer, &format!("{level} Enumeration `{name}`"))?;
    writeln!(writer, "enum {name} {{")?;

    for (item_name, value) in items.iter() {
        documentation_block(writer, &format!("* `{name}_{item_name}` = `{value}`"))?;
        writeln!(writer, "    {item_prefix}_{item_name} = {value},")?;
    }

    writeln!(writer, "}};")?;
    new_line(writer)?;

    Ok(())
}

pub fn structure(
    writer: &mut BufWriter<File>,
    name: &str,
    items: &Vec<(&str, &str)>,
) -> Result<()> {
    documentation_block(writer, &format!("=== Type `{name}`"))?;
    writeln!(writer, "struct {name} {{")?;

    for (type_, name) in items.iter() {
        documentation_block(writer, &format!("* `{type_}` `{name}`"))?;
        writeln!(writer, "    {type_} {name};")?;
    }

    writeln!(writer, "}};")?;
    new_line(writer)?;

    Ok(())
}

pub fn bitfield(
    writer: &mut BufWriter<File>,
    name: &str,
    items: &Vec<(&str, &str, u32)>,
) -> Result<()> {
    documentation_block(writer, &format!("=== Type `{name}`"))?;
    writeln!(writer, "struct {name} {{")?;

    for (type_, name, size) in items.iter() {
        documentation_block(writer, &format!("* `{type_}` `{name}`"))?;
        writeln!(writer, "    {type_} {name} : {size};")?;
    }

    writeln!(writer, "}};")?;
    new_line(writer)?;

    Ok(())
}

pub fn union(writer: &mut BufWriter<File>, name: &str, items: &Vec<(&str, &str)>) -> Result<()> {
    documentation_block(writer, &format!("=== Type `{name}`"))?;
    writeln!(writer, "union {name} {{")?;

    for (type_, name) in items.iter() {
        documentation_block(writer, &format!("* `{type_}` `{name}`"))?;
        writeln!(writer, "    {type_} {name};")?;
    }

    writeln!(writer, "}};")?;
    new_line(writer)?;

    Ok(())
}

pub fn typedef(writer: &mut BufWriter<File>, aliased: &str, alias: &str) -> Result<()> {
    documentation_block(writer, &format!("=== Type `{alias}`"))?;
    writeln!(writer, "typedef {aliased} {alias};")?;
    new_line(writer)?;

    Ok(())
}

pub fn memory_mapped_register(
    writer: &mut BufWriter<File>,
    level: &str,
    peripheral_id: &str,
    peripheral_symbol: &str,
    addressable_id: &str,
    register_id: &str,
    register_type: &str,
    register_offset: &str,
    register_is_bitfield: bool,
    index_lower: i64,
    index_upper: &str,
    index_stride: i64,
) -> Result<()> {
    let peripheral_id_upper = peripheral_id.to_uppercase();
    let addressable_id_upper = addressable_id.to_uppercase();
    let register_id_upper = register_id.to_uppercase();
    let target = if register_is_bitfield {
        "value.w"
    } else {
        "value"
    };

    writeln!(
        writer,
        r#"/// {level} Macro `{peripheral_id_upper}_{addressable_id_upper}_{register_id_upper}_OFFSET`
///
/// Access to memory-mapped register `{register_id}` offset.
#define {peripheral_id_upper}_{addressable_id_upper}_{register_id_upper}_OFFSET {register_offset}

/// {level} Function `rvbl_{peripheral_id}_{addressable_id}_{register_id}_write`
///
/// Sets the value of the *{register_id}* memory-mapped register (offset `{register_offset}`).
///
/// {level}= Parameters
/// `{register_type} value`:: (in) New value of the *{register_id}* register.
/// `const rvbl_uword_t`:: (in) Register index.
/// `const struct {peripheral_symbol}* const instance`:: (in) Device instance control block.
RVBL_INLINE()
void rvbl_{peripheral_id}_{addressable_id}_{register_id}_write(const struct {peripheral_symbol}* const instance, const rvbl_uword_t index, const {register_type} value)
{{"#
    )?;

    if index_lower > 0 {
        writeln!(
            writer,
            r#"
    if ((index >= ({index_lower})) && (index <= ({index_upper})))"#
        )?;
    } else {
        writeln!(
            writer,
            r#"
    if (index <= ({index_upper}))"#
        )?;
    }

    writeln!(
        writer,
        r#"
    {{
        *((volatile {register_type}*)(instance->{addressable_id}_region_address + {peripheral_id_upper}_{addressable_id_upper}_{register_id_upper}_OFFSET + (index*{index_stride}))) = value;
    }}
}}

/// {level} Function `rvbl_{peripheral_id}_{addressable_id}_{register_id}_read`
///
/// Gets the value of the *{register_id}* memory-mapped register (offset `{register_offset}`) .
///
/// {level}= Parameters
/// `const struct {peripheral_symbol}* const instance`:: (in) Device instance control block.
/// `const rvbl_uword_t`:: (in) Register index.
///
/// {level}= Return value
/// `{register_type}`:: (in) Current value of the *{register_id}* register.
RVBL_INLINE()
{register_type} rvbl_{peripheral_id}_{addressable_id}_{register_id}_read(const struct {peripheral_symbol}* const instance, const rvbl_uword_t index)
{{
    {register_type} value;
"#
    )?;

    if index_lower > 0 {
        writeln!(
            writer,
            r#"
    if ((index >= ({index_lower})) && (index <= ({index_upper})))"#
        )?;
    } else {
        writeln!(
            writer,
            r#"
    if (index <= ({index_upper}))"#
        )?;
    }

    writeln!(
        writer,
        r#"
    {{
        value = *((volatile {register_type}*)(instance->{addressable_id}_region_address + {peripheral_id_upper}_{addressable_id_upper}_{register_id_upper}_OFFSET + (index*{index_stride})));
    }} else {{
        {target} = 0;
    }}

    return value;
}}
"#
    )?;

    Ok(())
}

pub fn control_status_register(
    writer: &mut BufWriter<File>,
    level: &str,
    peripheral_id: &str,
    peripheral_symbol: &str,
    addressable_id: &str,
    register_id: &str,
    register_type: &str,
    register_offset: &str,
    register_is_bitfield: bool,
    _: i64,
    _: &str,
    _: i64,
) -> Result<()> {
    let register_offset_parsed = evaluator::parse_expression(register_offset);
    let target = if register_is_bitfield {
        "value.w"
    } else {
        "value"
    };

    writeln!(
        writer,
        r#"/// {level} Function `rvbl_{peripheral_id}_{addressable_id}_{register_id}_write`
///
/// Sets the value of the *{register_id}* control-status register (offset `{register_offset}`).
///
/// {level}= Parameters
/// `{register_type} value`:: (in) New value of the *{register_id}* register.
/// `const rvbl_uword_t`:: (in) Register index.
/// `const struct {peripheral_symbol}* const instance`:: (in) Device instance control block.
RVBL_INLINE()
void rvbl_{peripheral_id}_{addressable_id}_{register_id}_write(const struct {peripheral_symbol}* const instance, const rvbl_uword_t index, const {register_type} value)
{{
    (void)instance;
    (void)index;

    __asm__(
        "csrw 0x{register_offset_parsed:x}, %0\n\t"
        :
        : "r"({target})
        :
    );
}}

/// {level} Function `rvbl_{peripheral_id}_{addressable_id}_{register_id}_read`
///
/// Gets the value of the *{register_id}* control-status register (offset `{register_offset}`) .
///
/// {level}= Parameters
/// `const struct {peripheral_symbol}* const instance`:: (in) Device instance control block.
/// `const rvbl_uword_t`:: (in) Register index.
///
/// {level}= Return value
/// `{register_type}`:: (in) Current value of the *{register_id}* register.
RVBL_INLINE()
{register_type} rvbl_{peripheral_id}_{addressable_id}_{register_id}_read(const struct {peripheral_symbol}* const instance, const rvbl_uword_t index)
{{
    {register_type} value;

    (void)instance;
    (void)index;

    __asm__(
        "csrr %0, 0x{register_offset_parsed:x}\n\t"
        : "=r"({target})
        :
        :
    );

    return value;
}}"#
    )?;

    Ok(())
}

pub fn machine_compile_parameter(
    writer: &mut BufWriter<File>,
    level: &str,
    id: &str,
    parent: &str,
    parameter_type: &str,
    value: &str,
) -> Result<()> {
    let id_upper = id.to_uppercase();

    writeln!(
        writer,
        r#"/// {level} Macro `RVBL_CONFIGURATION_{id_upper}`
///
/// Access to compile-time `{parameter_type}` parameter `{id}` of `{parent}`.
#define RVBL_CONFIGURATION_{id_upper} ((rvbl_{parameter_type}_t){value})"#
    )?;

    Ok(())
}

pub fn machine_link_parameter(
    writer: &mut BufWriter<File>,
    level: &str,
    id: &str,
    parent: &str,
    parameter_type: &str,
) -> Result<()> {
    let id_upper = id.to_uppercase();

    writeln!(
        writer,
        r#"/// {level} Constant `rvbl_configuration_{id}`
///
/// Linker symbol for `{parameter_type}` constant `{id}` of `{parent}`.
const extern rvbl_{parameter_type}_t rvbl_configuration_{id};

/// {level} Macro `RVBL_CONFIGURATION_{id_upper}`
///
/// Access to link-time `{parameter_type}` parameter `{id}` of `{parent}`.
#define RVBL_CONFIGURATION_{id_upper} rvbl_configuration_{id}"#
    )?;

    Ok(())
}

pub fn machine_run_parameter(
    writer: &mut BufWriter<File>,
    level: &str,
    id: &str,
    parent: &str,
    parameter_type: &str,
) -> Result<()> {
    let id_upper = id.to_uppercase();

    writeln!(
        writer,
        r#"/// {level} Function `rvbl_configuration_{id}_fetch`
///
/// Linker symbol for `{parameter_type}` access function to `{id}` of `{parent}`.
const extern {parameter_type} rvbl_configuration_{id}(void);

/// {level} Macro `RVBL_CONFIGURATION_{id_upper}`
///
/// Access to run-time `{parameter_type}` parameter `{id}` of `{parent}`.
#define RVBL_CONFIGURATION_{id_upper} rvbl_configuration_{id}_fetch()"#
    )?;

    Ok(())
}
