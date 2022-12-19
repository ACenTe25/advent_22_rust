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
