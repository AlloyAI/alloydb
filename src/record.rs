use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct MetadataRecord {
    id: String,
    pub metadata: HashMap<String, Value>,
}

impl MetadataRecord {
    pub fn new(id: &str, metadata: HashMap<String, Value>) -> Self {
        Self {
            id: id.to_owned(),
            metadata,
        }
    }

    pub fn get_id(&self) -> &String {
        &self.id
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VectorRecord {
    id: String,
    pub values: Vec<f32>,
}

impl VectorRecord {
    pub fn new(id: &str, values: Vec<f32>) -> Self {
        Self {
            id: id.to_owned(),
            values,
        }
    }

    pub fn get_id(&self) -> &String {
        &self.id
    }
}
