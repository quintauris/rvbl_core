// Copyright 2026 Quintauris GmbH
// Licensed under the Apache License, Version 2.0 (the "License").
// https://www.apache.org/licenses/LICENSE-2.0

use eval;

fn parse_number(text: &str) -> i64 {
    if !text.is_empty() {
        let mut radix = 10;
        let mut first = 0;
        let mut last = text.len();
        let multiplier: i64 = match text.chars().last().unwrap() {
            'K' => {
                last -= 1;
                1024
            }
            'M' => {
                last -= 1;
                1024 * 1024
            }
            'G' => {
                last -= 1;
                1024 * 1024 * 1024
            }
            'k' => {
                last -= 1;
                1000
            }
            'm' => {
                last -= 1;
                1000 * 1000
            }
            'g' => {
                last -= 1;
                1000 * 1000 * 1000
            }
            _ => 1,
        };

        if text.starts_with("0x") {
            radix = 16;
            first = 2;
        }

        match i64::from_str_radix(&text[first..last], radix) {
            Ok(x) => x * multiplier,
            Err(_) => panic!("Unable to parse number '{}'.", text),
        }
    } else {
        0
    }
}

#[cfg(test)]
#[test]
fn test_parse_number() {
    assert_eq!(parse_expression("3K"), 3 * 1024);
}

pub fn parse_expression(text: &str) -> i64 {
    let pattern = regex_static::static_regex!(r"((0x[\dA-F]+)|\d+)[KMGkmg]?");
    let values: Vec<&str> = pattern.find_iter(text).map(|m| m.as_str()).collect();
    let mut updated = String::from(text);

    for value in values {
        updated = updated.replace(value, parse_number(value).to_string().as_str());
    }

    match eval::eval(&updated) {
        Ok(x) => {
            if x.is_i64() {
                x.as_i64().unwrap()
            } else if x.is_f64() {
                x.as_f64().unwrap().floor() as i64
            } else {
                panic!(
                    "Expression result from '{}' not convertible to integer",
                    text
                );
            }
        }
        Err(e) => panic!("Unable to evaluate expression '{}': {e}", text),
    }
}

#[cfg(test)]
#[test]
fn test_parse_expression_add_sub() {
    assert_eq!(
        parse_expression("1m + 16K + 0x5A32 + 21"),
        (1000 * 1000) + (16 * 1024) + 0x5A32 + 21
    );
}

#[cfg(test)]
#[test]
fn test_parse_expression_mul_div() {
    assert_eq!(parse_expression("16K * 32 / 16"), (16 * 1024) * 32 / 16);
}

#[cfg(test)]
#[test]
fn test_parse_expression_mod() {
    assert_eq!(parse_expression("16K % 4"), 0);
    assert_eq!(parse_expression("8 % 3"), 2);
}

#[cfg(test)]
#[test]
fn test_parse_expression_builtin() {
    assert_eq!(parse_expression("min(10, 8, 2)"), 2);
    assert_eq!(parse_expression("max(10, 8, 32)"), 32);
}

#[cfg(test)]
#[test]
fn test_parse_expression_decimal() {
    assert_eq!(parse_expression("3.5 + 1.5"), 5);
    assert_eq!(parse_expression("3.6 + 1.5"), 5);
    assert_eq!(parse_expression("3.9 + 1.9"), 5);
}

#[allow(unused)]
pub fn parse_expression_option(text: &Option<String>) -> i64 {
    match text {
        Some(x) => parse_expression(x),
        None => 0,
    }
}
