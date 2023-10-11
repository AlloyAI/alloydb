use std::collections::HashMap;

use anyhow::Result;
use sled::Db;

use crate::index::Index;

/// The main database struct that holds the sled database and the indexes
pub struct AlloyDB {
    pub db: Db,
    pub indexes: HashMap<String, Index>,
}

impl AlloyDB {
    pub fn create(database_path: &str) -> Result<Self> {
        let db = sled::open(format!("{}.alloy", database_path))?;
        let indexes = HashMap::new();
        Ok(Self { db, indexes })
    }

    /// Creates a new index based on a given field for the database
    pub fn create_index(&mut self, name: &str, dimensions: usize) -> Result<Index> {
        let index = Index::new(name, &self.db, dimensions)?;

        Ok(index)
    }
}
