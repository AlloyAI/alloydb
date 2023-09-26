use std::collections::HashMap;

use anyhow::{anyhow, Result};
use sled::Db;

use crate::{index::Index, record::Record};

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

    pub fn create_index(&mut self, name: &str) -> Result<()> {
        let index = Index::new(name, &self.db)?;
        self.indexes.insert(name.to_owned(), index);

        Ok(())
    }

    pub fn upsert(&self, index_id: &str, record: Record) -> Result<()> {
        let index = self
            .indexes
            .get(index_id)
            .ok_or_else(|| anyhow!("Index not found"))?;
        index.insert(record)?;

        Ok(())
    }
}
