use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use regex::Regex;

use crate::models::{Error, Modifier};

pub fn parse_fragment(content: &str) -> Result<HashMap<String, Modifier>, Error> {
    let mut modifiers = HashMap::new();

    let function_pattern = Regex::new("<span class=\"nf\">(\\w+)</span>").unwrap();
    let optional_pattern =
        Regex::new("<dt>(\\w+) <span><a class=\"token\" href=\"(.+)\" title=\"Optional\">Opt")
            .unwrap();
    let output_pattern =
        Regex::new("<dt>(\\w+) <span><a class=\"token\" href=\"(.+)\" title=\"Output\">Out")
            .unwrap();

    // <div class="highlight language-c">

    let mut functions = vec![];
    for line in content.lines() {
        if line.contains("<div class=\"language-selector\">") {
            functions = vec![];
        }
        if let Some(captures) = function_pattern.captures(line) {
            let function = captures.get(1).unwrap().as_str();
            functions.push(function);
            // println!("fn: {}", function);
        } else if let Some(captures) = output_pattern.captures(line) {
            let argument = captures.get(1).unwrap().as_str();
            // println!("-------> {}", argument);
            for function in &functions {
                let key = format!("{}+{}", function, argument);
                modifiers.insert(key, Modifier::Out);
            }
        } else if let Some(captures) = optional_pattern.captures(line) {
            let argument = captures.get(1).unwrap().as_str();
            for function in &functions {
                let key = format!("{}+{}", function, argument);
                modifiers.insert(key, Modifier::Opt);
            }
        }
    }
    Ok(modifiers)
}

pub fn parse_parameter_modifiers(paths: &[PathBuf]) -> Result<HashMap<String, Modifier>, Error> {
    let mut output = HashMap::new();
    for path in paths {
        let html = fs::read_to_string(path)?;
        output.extend(parse_fragment(&html)?)
    }
    Ok(output)
}
