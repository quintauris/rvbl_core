// Copyright 2026 Quintauris GmbH
// Licensed under the Apache License, Version 2.0 (the "License").
// https://www.apache.org/licenses/LICENSE-2.0

use std::fs::File;
use std::io::{BufWriter, Result, Write};

use rvbl_model::{evaluator::parse_expression, model};

pub trait MemoryLayoutTemplate {
    fn open(&self, writer: &mut BufWriter<File>) -> Result<()>;
    fn entry(&self, writer: &mut BufWriter<File>, region: &model::Addressable) -> Result<()>;
    fn close(&self, writer: &mut BufWriter<File>) -> Result<()>;
}

pub struct Gnu {}

impl Gnu {
    fn attribute(value: &Option<bool>, letter: char, default: bool) -> String {
        match value {
            Some(b) => match b {
                true => letter.to_string(),
                false => String::from(""),
            },
            None => match default {
                true => letter.to_string(),
                false => String::from(""),
            },
        }
    }
}

impl MemoryLayoutTemplate for Gnu {
    fn open(&self, writer: &mut BufWriter<File>) -> Result<()> {
        writeln!(writer, "MEMORY {{")?;

        Ok(())
    }

    fn entry(&self, writer: &mut BufWriter<File>, region: &model::Addressable) -> Result<()> {
        writeln!(
            writer,
            "{}({}{}{}) : ORIGIN = {:#010x}, LENGTH = {:#010x}",
            &region.name,
            Gnu::attribute(&region.read, 'r', true),
            Gnu::attribute(&region.write, 'w', true),
            Gnu::attribute(&region.execute, 'x', false),
            parse_expression(&region.base),
            parse_expression(&region.size)
        )?;

        Ok(())
    }

    fn close(&self, writer: &mut BufWriter<File>) -> Result<()> {
        writeln!(writer, "}}")?;

        Ok(())
    }
}

pub struct Iar {}

impl MemoryLayoutTemplate for Iar {
    fn open(&self, _: &mut BufWriter<File>) -> Result<()> {
        Ok(())
    }

    fn entry(&self, writer: &mut BufWriter<File>, region: &model::Addressable) -> Result<()> {
        writeln!(
            writer,
            "define region {} = Mem:[from {:#010x} size {:#010x}];",
            &region.name,
            parse_expression(&region.base),
            parse_expression(&region.size)
        )?;

        Ok(())
    }

    fn close(&self, _: &mut BufWriter<File>) -> Result<()> {
        Ok(())
    }
}

pub struct Tasking {}

impl MemoryLayoutTemplate for Tasking {
    fn open(&self, _: &mut BufWriter<File>) -> Result<()> {
        Ok(())
    }

    fn entry(&self, writer: &mut BufWriter<File>, region: &model::Addressable) -> Result<()> {
        writeln!(
            writer,
            r#"memory {}
{{
  type = ram;
  mau = 8;
  size = {:#010x};
  map(dest_offset={:#010x}, size={:#010x}, dest=bus:rv32i:main_bus);
}}"#,
            &region.name,
            parse_expression(&region.size),
            parse_expression(&region.base),
            parse_expression(&region.size),
        )?;
        writeln!(writer)?;

        Ok(())
    }

    fn close(&self, _: &mut BufWriter<File>) -> Result<()> {
        Ok(())
    }
}
