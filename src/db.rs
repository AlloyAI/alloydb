use crate::vectorizor::{read_vectors_from_file, vectorize_text, Vector};
use anyhow::Result;

pub struct Db {
    path: String,
}

impl Db {
    /// Create a new Db
    pub fn new(path: String) -> Self {
        Self { path }
    }

    pub fn get_all(&self) -> Result<Vec<Vector>> {
        read_vectors_from_file(&self.path)
    }

    // TODO: Handle errors
    pub fn insert(&mut self, text: &str) -> Result<()> {
        let vector = vectorize_text(text)?;
        vector.save(&self.path)?;

        Ok(())
    }
}
