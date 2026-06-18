// Copyright 2026 Quintauris GmbH
// Licensed under the Apache License, Version 2.0 (the "License").
// https://www.apache.org/licenses/LICENSE-2.0

use std::io::{Result, Write};
use std::{fs::File, io::BufWriter};

pub fn section(writer: &mut BufWriter<File>, level: usize, text: &str) -> Result<()> {
    writeln!(writer, "{} {}", "=".repeat(level), text)?;

    Ok(())
}

pub fn paragraph(writer: &mut BufWriter<File>, text: &str) -> Result<()> {
    writeln!(writer, "{}", &text)?;

    Ok(())
}

pub fn paragraph_optional(writer: &mut BufWriter<File>, text: &Option<String>) -> Result<()> {
    if text.is_some() {
        paragraph(writer, text.as_ref().unwrap())?;
    }

    Ok(())
}

pub fn new_line(writer: &mut BufWriter<File>) -> Result<()> {
    writeln!(writer, "")?;

    Ok(())
}

pub fn table_header(writer: &mut BufWriter<File>, items: &Vec<&str>) -> Result<()> {
    writeln!(
        writer,
        "[cols=\"{}\",options=\"header\"]",
        std::iter::repeat_n("1".to_string(), items.len())
            .collect::<Vec<String>>()
            .join(",")
    )?;
    writeln!(writer, "|===")?;
    writeln!(writer, "|{}", items.join("\n|"))?;
    new_line(writer)?;

    Ok(())
}

pub fn table_row(writer: &mut BufWriter<File>, items: &Vec<&str>) -> Result<()> {
    writeln!(writer, "|{}", items.join("\n|"))?;
    new_line(writer)?;

    Ok(())
}

pub fn table_footer(writer: &mut BufWriter<File>) -> Result<()> {
    writeln!(writer, "|===")?;

    Ok(())
}

pub fn horizontal_list(writer: &mut BufWriter<File>, items: &Vec<(&str, &str)>) -> Result<()> {
    writeln!(writer, "[horizontal]")?;

    for item in items.iter() {
        writeln!(writer, "{}:: {}", item.0, item.1)?;
    }

    Ok(())
}
