use std::collections::HashMap;

use anyhow::Result;
use serde_json::Value;
use sled::{Db, Tree};
use uuid::Uuid;

use crate::record::{MetadataRecord, VectorRecord};

const METADATA_PREFIX: &str = "metadata.";
const VECTOR_PREFIX: &str = "vector.";

/// An index for storing records
pub struct Index {
    /// The name of the index
    pub name: String,
    /// The tree used to store the metadata
    pub metadata_tree: Tree,
    /// The tree used to store the vector data
    pub vector_data_tree: Tree,
}

impl Index {
    /// Creates a new Index with the given name and database.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the index.
    /// * `db` - The database to use for storing the data.
    pub fn new(name: &str, db: &Db) -> Result<Self> {
        let metadata_tree = db.open_tree(format!("{}.meta.data", name))?;
        let vector_data_tree = db.open_tree(format!("{}.vector.data", name))?;

        Ok(Self {
            name: name.to_owned(),
            metadata_tree,
            vector_data_tree,
        })
    }

    // Inserts a record into the index.
    ///
    /// # Arguments
    ///
    /// * `record` - The record to insert.
    pub fn insert(&self, metadata: HashMap<String, Value>, vector: Vec<f32>) -> Result<()> {
        let uuid = Uuid::new_v4().to_string();

        let metadata_record = MetadataRecord::new(&uuid, metadata);
        let vector_record = VectorRecord::new(&uuid, vector);

        // Serialize the records
        // TODO: Look into serde_bincode instead of serde_json
        let metadata_bytes = serde_json::to_vec(&metadata_record)?;
        let vector_bytes = serde_json::to_vec(&vector_record)?;

        self.metadata_tree.insert(
            format!("{}.{}.{}", self.name, METADATA_PREFIX, uuid.clone()),
            metadata_bytes,
        )?;
        self.vector_data_tree.insert(
            format!("{}.{}.{}", self.name, VECTOR_PREFIX, uuid),
            vector_bytes,
        )?;

        Ok(())
    }

    /// Gets a metadata record from the index.
    pub fn get_metadata(&self, id: &str) -> Result<Option<MetadataRecord>> {
        let key = format!("{}.{}.{}", self.name, METADATA_PREFIX, id);
        let value = self
            .metadata_tree
            .get(key)?
            .map(|v| serde_json::from_slice(&v))
            .transpose()?;

        Ok(value)
    }

    /// Gets a vector record from the index.
    pub fn get_vector(&self, id: &str) -> Result<Option<VectorRecord>> {
        let key = format!("{}.{}.{}", self.name, VECTOR_PREFIX, id);
        let value = self
            .metadata_tree
            .get(key)?
            .map(|v| serde_json::from_slice(&v))
            .transpose()?;

        Ok(value)
    }

    /// Get all metadata records in the index.
    pub fn all_metadata(&self) -> Result<Vec<MetadataRecord>> {
        let iter = self.metadata_tree.scan_prefix(METADATA_PREFIX).values();

        let mut results = Vec::new();

        for value in iter {
            // TODO: Look into serde_bincode instead of serde_json
            let value = serde_json::from_slice::<MetadataRecord>(&value?)?;

            results.push(value);
        }

        Ok(results)
    }

    /// Get all metadata records in the index.
    pub fn all_vectors(&self) -> Result<Vec<VectorRecord>> {
        let iter = self.metadata_tree.scan_prefix(VECTOR_PREFIX).values();

        let mut results = Vec::new();

        for value in iter {
            let value = serde_json::from_slice::<VectorRecord>(&value?)?;

            results.push(value);
        }

        Ok(results)
    }

    // NOTE (Casey): This is a very naive implementation of querying. It will be replaced with a
    // dynamic index based querying system for metadata and an id index. Database Indexes not to be confused with
    // our internal Index struct.
    /// Queries the index for records matching the given metadata.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the record to fetch.
    /// * `metadata` - The metadata to match.
    ///
    /// # Returns
    ///
    /// A vector of records matching the given metadata.
    pub fn query(
        &self,
        id: Option<&str>,
        metadata: Option<&HashMap<String, Value>>,
    ) -> Result<Vec<VectorRecord>> {
        let mut metadata_records: Vec<MetadataRecord> = Vec::new();

        // If metadata is provided, scan and filter the records
        if let Some(search_metadata) = metadata {
            let iter = self.metadata_tree.scan_prefix(METADATA_PREFIX).values();
            let mut results = Vec::<MetadataRecord>::new();

            for value in iter {
                let record = serde_json::from_slice::<MetadataRecord>(&value?)?;

                if &record.metadata == search_metadata {
                    metadata_records.push(record);
                }
            }
        }

        // Now find the associated vector records
        let mut vector_records = Vec::<VectorRecord>::with_capacity(metadata_records.len());

        // If an ID is provided, directly fetch and return the corresponding record
        if let Some(record_id) = id {
            let key = format!("{}.{}.{}", self.name, VECTOR_PREFIX, record_id);
            let value = self.vector_data_tree.get(&key)?;
            if let Some(v) = value {
                vector_records.push(serde_json::from_slice::<VectorRecord>(&v)?);

                return Ok(vector_records);
            }
        } else {
            // Otherwise search through the metadata records and fetch the corresponding vector records
            for metadata_record in metadata_records {
                let key = format!(
                    "{}.{}.{}",
                    self.name,
                    VECTOR_PREFIX,
                    metadata_record.get_id()
                );
                let value = self.vector_data_tree.get(&key)?;
                if let Some(v) = value {
                    vector_records.push(serde_json::from_slice::<VectorRecord>(&v)?);
                };
            }

            return Ok(vector_records);
        }

        // TODO (Casey): Add vectory similarity search after finding the vector records

        // If neither ID nor metadata is provided
        Err(anyhow::anyhow!(
            "Either ID or metadata should be provided for query"
        ))
    }
}
