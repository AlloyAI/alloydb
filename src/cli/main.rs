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
    let filename = "test.alloy";
    delete_if_exists(filename)?;
    let mut db = alloydb::Db::new("test.alloy".to_string());
    db.insert("Hello, world!")?;
    db.insert("The Big Brown Fox Jumped Over The Lazy Dog")?;

    let all_data = db.get_all()?;

    for data in all_data {
        println!(
            "Vector: uuid: {}, original_text: {}, vector_data_truncated: {:?}, vector_length: {}",
            data.id,
            data.original_text,
            data.vector[0..5].to_vec(),
            data.vector.len()
        );
    }

    Ok(())
}
