// Standard Library
use std::fs::File;
use std::path::Path;
use std::io::Read;

// External crates
use anyhow::{Context, Result, anyhow};

// Read a text input file to a string
pub fn read_text_input(path: &str) -> Result<String> {

    // Convert to path, and check if exists
    let path_inner = Path::new(path);
    
    if path_inner.try_exists()
    .context("reading text input file")? {

        let mut file = File::open(path_inner)
        .context("reading text input file")?;

        let mut input_string = String::new();

        file.read_to_string(&mut input_string)
        .context("reading text input file")?;

        Ok(input_string)

    } else {

        Err(anyhow!("File '{}' does not exist", path))
    }
}

pub fn get_input_lines(path: &str) -> Result<Vec<String>> {

    // Get input file as a string
    let input_string = read_text_input(path)
    .context("getting input lines")?;

    // Turn lines into vector of Strings
    let mut input_lines: Vec<String> = Vec::new();

    for line in input_string.lines() {
        input_lines.push(line.to_string());
    }

    Ok(input_lines)
}
