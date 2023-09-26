use anyhow::Result;
use sled::{Db, Tree};

use crate::record::Record;

/// An index for storing records
pub struct Index {
    /// The name of the index
    pub name: String,
    /// The tree used to store the data
    pub data_tree: Tree,
}

impl Index {
    /// Creates a new Index with the given name and database.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the index.
    /// * `db` - The database to use for storing the data.
    pub fn new(name: &str, db: &Db) -> Result<Self> {
        let data_tree = db.open_tree(format!("{}.data", name))?;

        Ok(Self {
            name: name.to_owned(),
            data_tree,
        })
    }

    // Inserts a record into the index.
    ///
    /// # Arguments
    ///
    /// * `record` - The record to insert.
    pub fn insert(&self, record: Record) -> Result<()> {
        let key = format!("{}.{}", self.name, record.id);
        let value = serde_json::to_vec(&record)?;

        self.data_tree.insert(key, value)?;

        Ok(())
    }

    /// Gets a record from the index.
    pub fn get(&self, id: &str) -> Result<Option<Record>> {
        let key = format!("{}.{}", self.name, id);
        let value = self
            .data_tree
            .get(key)?
            .map(|v| serde_json::from_slice(&v))
            .transpose()?;

        Ok(value)
    }

    /// Lists all records in the index.
    pub fn list(&self) -> Result<Vec<Record>> {
        let prefix = format!("{}.", self.name);
        let iter = self.data_tree.scan_prefix(prefix).values();

        let mut results = Vec::new();

        for value in iter {
            let value = serde_json::from_slice::<Record>(&value?)?;

            results.push(value);
        }

        Ok(results)
    }

    /// Queries the index for records matching the given metadata.
    ///
    /// # Arguments
    ///
    /// * `metadata` - The metadata to match.
    ///
    /// # Returns
    ///
    /// A vector of records matching the given metadata.
    pub fn query(&self, metadata: serde_json::Value) -> Result<Vec<Record>> {
        let prefix = format!("{}.", self.name);
        let iter = self.data_tree.scan_prefix(prefix).values();

        let mut results = Vec::new();

        for value in iter {
            let value = serde_json::from_slice::<Record>(&value?)?;

            if value.metadata == metadata {
                results.push(value);
            }
        }

        Ok(results)
    }
}
