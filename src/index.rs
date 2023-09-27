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
    /// * `metadata` - The metadata record to insert.
    /// * `vector` - The vector record to insert.
    pub fn insert(&self, metadata: HashMap<String, Value>, vector: Vec<f32>) -> Result<String> {
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

        Ok(uuid)
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
            .vector_data_tree
            .get(key)?
            .map(|v| serde_json::from_slice(&v))
            .transpose()?;

        Ok(value)
    }

    /// Get all metadata records in the index.
    fn all_metadata(&self) -> Result<Vec<MetadataRecord>> {
        let iter = self
            .metadata_tree
            .scan_prefix(format!("{}.{}", self.name, METADATA_PREFIX))
            .values();

        let mut results = Vec::new();

        for value in iter {
            // TODO: Look into serde_bincode instead of serde_json
            let value = serde_json::from_slice::<MetadataRecord>(&value?)?;

            results.push(value);
        }

        Ok(results)
    }

    /// Get all metadata records in the index.
    fn all_vectors(&self) -> Result<Vec<VectorRecord>> {
        let iter = self
            .metadata_tree
            .scan_prefix(format!("{}.{}", self.name, VECTOR_PREFIX))
            .values();

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
        let mut vector_records = Vec::<VectorRecord>::new();
        let mut metadata_records: Vec<MetadataRecord> = Vec::new();

        // If an ID is provided, directly fetch and return the corresponding record
        if let Some(record_id) = id {
            let key = format!("{}.{}.{}", self.name, VECTOR_PREFIX, record_id);
            let value = self.vector_data_tree.get(&key)?;
            if let Some(v) = value {
                vector_records.push(serde_json::from_slice::<VectorRecord>(&v)?);

                return Ok(vector_records);
            }
        }

        // If metadata is provided, scan and filter the records
        if let Some(search_metadata) = metadata {
            let iter = self
                .metadata_tree
                .scan_prefix(format!("{}.{}", self.name, METADATA_PREFIX))
                .values();

            for value in iter {
                let record = serde_json::from_slice::<MetadataRecord>(&value?)?;

                if &record.metadata == search_metadata {
                    metadata_records.push(record);
                }
            }
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

#[cfg(test)]
mod tests {

    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_insert_and_get() -> Result<()> {
        let dir = tempdir()?;
        let db = sled::open(&dir)?;
        let index = Index::new("test", &db)?;

        let metadata = HashMap::new();
        let vector = vec![1.0, 2.0, 3.0];

        let id = index.insert(metadata.clone(), vector.clone())?;

        let metadata_record = index.get_metadata(&id)?.unwrap();
        let vector_record = index.get_vector(&id)?.unwrap();

        assert_eq!(metadata_record.metadata, metadata);
        assert_eq!(vector_record.values, vector);

        Ok(())
    }

    #[test]
    fn test_query_by_id() -> Result<()> {
        let dir = tempdir()?;
        let db = sled::open(&dir)?;
        let index = Index::new("test", &db)?;

        let metadata = HashMap::new();
        let vector = vec![1.0, 2.0, 3.0];

        let id = index.insert(metadata.clone(), vector.clone())?;

        let records = index.query(Some(&id), None)?;

        assert_eq!(records.len(), 1);
        assert_eq!(records[0].values, vector);

        Ok(())
    }

    #[test]
    fn test_query_by_metadata() -> Result<()> {
        let dir = tempdir()?;
        let db = sled::open(&dir)?;
        let index = Index::new("test", &db)?;

        let metadata1 = HashMap::from([("foo".to_owned(), Value::from("bar"))]);
        let vector1 = vec![1.0, 2.0, 3.0];

        index.insert(metadata1.clone(), vector1.clone())?;

        let search_metadata = HashMap::from([("foo".to_owned(), Value::from("bar"))]);
        let records = index.query(None, Some(&search_metadata))?;

        assert_eq!(records.len(), 1);
        assert_eq!(records[0].values, vector1);

        Ok(())
    }

    #[test]
    fn test_query_by_id_and_metadata() -> Result<()> {
        let dir = tempdir()?;
        let db = sled::open(&dir)?;
        let index = Index::new("test", &db)?;

        let metadata1 = HashMap::from([("foo".to_owned(), Value::from("bar"))]);
        // let metadata2 = HashMap::from([("baz".to_owned(), Value::from("qux"))]);
        let vector1 = vec![1.0, 2.0, 3.0];
        // let vector2 = vec![4.0, 5.0, 6.0];

        let id1 = index.insert(metadata1.clone(), vector1.clone())?;
        // let id2 = index.insert(metadata2.clone(), vector2.clone())?;

        let search_metadata = HashMap::from([("foo".to_owned(), Value::from("bar"))]);
        let records = index.query(Some(&id1), Some(&search_metadata))?;

        assert_eq!(records.len(), 1);
        assert_eq!(records[0].values, vector1);

        Ok(())
    }
}
