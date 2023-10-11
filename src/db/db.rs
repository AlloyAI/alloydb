use std::collections::HashMap;

use anyhow::{anyhow, Result};
use sled::Db;

use crate::index::Index;

pub struct Record {
    pub metadata: HashMap<String, serde_json::Value>,
    pub vector: Vec<f32>,
}

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
    pub fn create_index(&mut self, field: &str) -> Result<Index> {
        let index = Index::new(field, &self.db)?;

        Ok(index)
    }

    pub fn upsert(&self, index_id: &str, record: Record) -> Result<()> {
        let index = self
            .indexes
            .get(index_id)
            .ok_or_else(|| anyhow!("Index not found"))?;
        index.insert(record.metadata, record.vector)?;

        Ok(())
    }

    /// Queries the DB for records matching the given metadata.
    ///
    /// # Arguments
    ///
    /// * `metadata` - The metadata to match.
    ///
    /// # Returns
    ///
    /// A vector of records matching the given metadata.
    pub fn query(&self) -> Result<Vec<Record>> {
        unimplemented!()
    }
}
