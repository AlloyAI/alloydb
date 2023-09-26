use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Record {
    pub id: String,
    pub values: Vec<f32>,
    pub metadata: serde_json::Value,
}

impl Record {
    pub fn new(id: &str, values: Vec<f32>, metadata: serde_json::Value) -> Self {
        Self {
            id: id.to_owned(),
            values,
            metadata,
        }
    }
}
