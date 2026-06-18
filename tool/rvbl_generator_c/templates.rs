// Copyright 2026 Quintauris GmbH
// Licensed under the Apache License, Version 2.0 (the "License").
// https://www.apache.org/licenses/LICENSE-2.0

use std::io::{Result, Write};
use std::path::Path;
use std::{fs::File, io::BufWriter};

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
    items: &Vec<(&str, &str)>,
) -> Result<()> {
    documentation_block(writer, &format!("{level} Enumeration `{name}`"))?;
    writeln!(writer, "typedef enum {name} {{")?;

    for (item_name, value) in items.iter() {
        documentation_block(writer, &format!("* `{name}_{item_name}` = `{value}`"))?;
        writeln!(writer, "    {name}_{item_name} = {value},")?;
    }

    writeln!(writer, "}} {name};")?;
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

pub fn memory_mapped_register(
    writer: &mut BufWriter<File>,
    level: &str,
    namespace: &str,
    register_id: &str,
    register_type: &str,
    register_offset: &str,
    memory_region: &str,
    type_symbol: &str,
    index_parameter: &str,
    index_check: &str,
    index_operation: &str,
    index_comment: &str,
) -> Result<()> {
    let id_upper = register_id.to_uppercase();

    writeln!(
        writer,
        r#"/// {level} Macro `{namespace}_{id_upper}_OFFSET`
///
/// Access to memory-mapped register `{register_id}` offset.
#define {namespace}_{id_upper}_OFFSET {register_offset}

/// {level} Function `rvbl_{register_id}_write`
///
/// Sets the value of the *{register_id}* memory-mapped register (offset `{register_offset}`).
///
/// {level}= Parameters
/// `{register_type} value`:: (in) New value of the *{register_id}* register.
/// {index_comment}
/// `const struct {type_symbol}* const instance`:: (in) Device instance control block.
RVBL_INLINE()
void rvbl_{register_id}_write(const struct {type_symbol}* const instance{index_parameter}, const {register_type} value)
{{
    {index_check}
    {{
        *((volatile {register_type}*)(instance->{memory_region}_region_address + {namespace}_{id_upper}_OFFSET{index_operation})) = value;
    }}
}}

/// {level} Function `rvbl_{register_id}_read`
///
/// Gets the value of the *{register_id}* memory-mapped register (offset `{register_offset}`) .
///
/// {level}= Parameters
/// `const struct {type_symbol}* const instance`:: (in) Device instance control block.
/// {index_comment}
///
/// {level}= Return value
/// `{register_type}`:: (in) Current value of the *{register_id}* register.
RVBL_INLINE()
{register_type} rvbl_{register_id}_read(const struct {type_symbol}* const instance{index_parameter})
{{
    {register_type} result = (({register_type})0);

    {index_check}
    {{
        result = *((volatile {register_type}*)(instance->{memory_region}_region_address + {namespace}_{id_upper}_OFFSET{index_operation}));
    }}

    return result;
}}"#
    )?;

    Ok(())
}

pub fn memory_mapped_register_field(
    writer: &mut BufWriter<File>,
    level: &str,
    register_id: &str,
    field_id: &str,
    field_type: &str,
    field_position: u32,
    field_length: u32,
    type_symbol: &str,
    index_parameter: &str,
    index_value: &str,
    index_comment: &str,
) -> Result<()> {
    let mask: u32 = ((1 << field_length) - 1) << field_position;

    writeln!(
        writer,
        r#"/// {level} Function `rvbl_{register_id}_{field_id}_write`
///
/// Sets the value of *{field_id}* within the *{register_id}* register.
///
/// {level}= Parameters
/// `{field_type} value`:: (in) New value of *{field_id}*.
/// {index_comment}
/// `const struct {type_symbol}* const instance`:: (in) Device instance control block.
RVBL_INLINE()
void rvbl_{register_id}_{field_id}_write(const struct {type_symbol}* const instance{index_parameter}, const {field_type} value)
{{
    rvbl_{register_id}_write(
        instance{index_value},
        (rvbl_{register_id}_read(instance{index_value}) & (~0x{mask:08x})) | ((value << {field_position}) & 0x{mask:08x})
    );
}}

/// {level} Function `rvbl_{register_id}_{field_id}_read`
///
/// Gets the value of *{field_id}* within the *{register_id}* register.
///
/// {level}= Parameters
/// `const struct {type_symbol}* const instance`:: (in) Device instance control block.
/// {index_comment}
///
/// {level}= Return value
/// `{field_type}`:: Current value of *{field_id}*.
RVBL_INLINE()
{field_type}
rvbl_{register_id}_{field_id}_read(const struct {type_symbol}* const instance{index_parameter})
{{
    return ({field_type})((rvbl_{register_id}_read(instance{index_value}) & 0x{mask:08x}) >> {field_position});
}}

/// {level} Function `rvbl_{register_id}_{field_id}`
///
/// Gets the value of *{field_id}* within the *{register_id}* register in boolean form.
///
/// {level}= Parameters
/// `const struct {type_symbol}* const instance`:: (in) Device instance control block.
/// {index_comment}
///
/// {level}= Return value
/// `{field_type}`:: Current boolean value of *{field_id}*.
RVBL_INLINE()
rvbl_bool_t
rvbl_{register_id}_{field_id}(const struct {type_symbol}* const instance{index_parameter})
{{
    return (rvbl_bool_t)rvbl_{register_id}_{field_id}_read(instance{index_value});
}}

/// {level} Function `rvbl_{register_id}_{field_id}_set`
///
/// Sets *{field_id}* within the *{register_id}* register to 1.
///
/// {level}= Parameters
/// `const struct {type_symbol}* const instance`:: (in) Device instance control block.
/// {index_comment}
RVBL_INLINE()
void
rvbl_{register_id}_{field_id}_set(const struct {type_symbol}* const instance{index_parameter})
{{
    rvbl_{register_id}_write(
        instance{index_value},
        rvbl_{register_id}_read(instance{index_value}) | 0x{mask:08x}
    );
}}

/// {level} Function `rvbl_{register_id}_{field_id}_clear`
///
/// Sets *{field_id}* within the *{register_id}* register to 0.
///
/// {level}= Parameters
/// `const struct {type_symbol}* const instance`:: (in) Device instance control block.
/// {index_comment}
RVBL_INLINE()
void
rvbl_{register_id}_{field_id}_clear(const struct {type_symbol}* const instance{index_parameter})
{{
    rvbl_{register_id}_write(
        instance{index_value},
        rvbl_{register_id}_read(instance{index_value}) & (~0x{mask:08x})
    );
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

pub fn control_status_register(
    writer: &mut BufWriter<File>,
    level: &str,
    register_id: &str,
    register_type: &str,
    register_offset: i64,
) -> Result<()> {
    writeln!(
        writer,
        r#"/// {level} Function `rvbl_{register_id}_write`
///
/// Sets the value of the *{register_id}* CSR (offset `0x{register_offset:x}`) .
///
/// {level}= Parameters
/// `{register_type} value`:: (in) New value of the *{register_id}* CSR.
RVBL_INLINE()
void rvbl_{register_id}_write(const {register_type} value)
{{
    __asm__(
        "csrw 0x{register_offset:x}, %0\n\t"
        :
        : "r"(value)
        :
    );
}}

/// {level} Function `rvbl_{register_id}_read`
///
/// Gets the value of the *{register_id}* CSR (offset `0x{register_offset:x}`) .
///
/// {level}= Return value
/// `{register_type}`:: Current value of the *{register_id}* CSR.
RVBL_INLINE()
{register_type} rvbl_{register_id}_read(void)
{{
    {register_type} result;

    __asm__(
        "csrr %0, 0x{register_offset:x}\n\t"
        : "=r"(result)
        :
        :
    );

    return result;
}}"#
    )?;

    Ok(())
}

pub fn control_status_register_field(
    writer: &mut BufWriter<File>,
    level: &str,
    register_id: &str,
    register_type: &str,
    field_id: &str,
    field_type: &str,
    field_position: u32,
    field_length: u32,
) -> Result<()> {
    let mask: u32 = ((1 << field_length) - 1) << field_position;

    writeln!(
        writer,
        r#"/// {level} Function `rvbl_{register_id}_{field_id}_write`
///
/// Sets the value of *{field_id}* within the *{register_id}* register.
///
/// {level}= Parameters
/// `{field_type} value`:: (in) New value of *{field_id}*.
RVBL_INLINE()
void rvbl_{register_id}_{field_id}_write(const {field_type} value)
{{
    rvbl_{register_id}_write(
        (rvbl_{register_id}_read() & (({register_type})~{mask})) | ((value << {field_position}) & ({register_type}){mask})
    );
}}

/// {level} Function `rvbl_{register_id}_{field_id}_read`
///
/// Gets the value of *{field_id}* within the *{register_id}* register.
///
/// {level}= Return value
/// `{field_type}`:: Current value of *{field_id}*.
RVBL_INLINE()
{field_type}
rvbl_{register_id}_{field_id}_read(void)
{{
    return ({field_type})((rvbl_{register_id}_read() & {mask}) >> {field_position});
}}

/// {level} Function `rvbl_{register_id}_{field_id}`
///
/// Gets the value of *{field_id}* within the *{register_id}* register in boolean form.
///
/// {level}= Return value
/// `{field_type}`:: Current boolean value of *{field_id}*.
RVBL_INLINE()
rvbl_bool_t
rvbl_{register_id}_{field_id}(void)
{{
    return (rvbl_bool_t)rvbl_{register_id}_{field_id}_read();
}}"#
    )?;

    Ok(())
}

pub fn control_status_register_field_bits_fast(
    writer: &mut BufWriter<File>,
    level: &str,
    register_id: &str,
    register_offset: i64,
    field_id: &str,
    field_position: u32,
    field_length: u32,
) -> Result<()> {
    let mask: u32 = ((1 << field_length) - 1) << field_position;

    writeln!(
        writer,
        r#"/// {level} Function `rvbl_{register_id}_{field_id}_set`
///
/// Sets *{field_id}* within the *{register_id}* CSR to 1.
RVBL_INLINE()
void
rvbl_{register_id}_{field_id}_set(void)
{{
    __asm__(
       "csrs 0x{register_offset:x}, %0\n\t"
       :
       : "r"(0x{mask:x})
       :
    );
}}

/// {level} Function `rvbl_{register_id}_{field_id}_clear`
///
/// Sets *{field_id}* within the *{register_id}* CSR to 0.
RVBL_INLINE()
void
rvbl_{register_id}_{field_id}_clear(void)
{{
    __asm__(
       "csrc 0x{register_offset:x}, %0\n\t"
       :
       : "r"(0x{mask:x})
       :
    );
}}"#
    )?;

    Ok(())
}

pub fn control_status_register_field_bits_slow(
    writer: &mut BufWriter<File>,
    level: &str,
    register_id: &str,
    field_id: &str,
    field_position: u32,
    field_length: u32,
) -> Result<()> {
    let mask: u32 = ((1 << field_length) - 1) << field_position;

    writeln!(
        writer,
        r#"/// {level} Function `rvbl_{register_id}_{field_id}_set`
///
/// Sets *{field_id}* within the *{register_id}* register to 1.
RVBL_INLINE()
void
rvbl_{register_id}_{field_id}_set(void)
{{
    rvbl_{register_id}_write(
        rvbl_{register_id}_read() | 0x{mask:x}
    );
}}

/// {level} Function `rvbl_{register_id}_{field_id}_clear`
///
/// Sets *{field_id}* within the *{register_id}* register to 0.
RVBL_INLINE()
void
rvbl_{register_id}_{field_id}_clear(void)
{{
    rvbl_{register_id}_write(
        rvbl_{register_id}_read() & (~0x{mask:x})
    );
}}"#
    )?;

    Ok(())
}
