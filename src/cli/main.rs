use anyhow::Result;
use std::{fs, path::Path};

extern crate alloydb;

fn delete_if_exists(filename: &str) -> Result<()> {
    let path = Path::new(filename);
    if path.exists() {
        fs::remove_file(path)?;
    }
    Ok(())
}

fn main() -> Result<()> {
    Ok(())
}
