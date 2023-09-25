use std::collections::HashMap;
use uuid::Uuid;

// Define the sizes in bytes of each possible type for custom fields.
const SIZE_I8: usize = 1;
const SIZE_I16: usize = 2;
const SIZE_I32: usize = 4;
const SIZE_I64: usize = 8;

const SIZE_U8: usize = 1;
const SIZE_U16: usize = 2;
const SIZE_U32: usize = 4;
const SIZE_U64: usize = 8;

const SIZE_F32: usize = 4;
const SIZE_F64: usize = 8;

const SIZE_UUID: usize = 16;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum FieldType {
    UUID,
    String,
    U8,
    U16,
    U32,
    U64,
    I8,
    I16,
    I32,
    I64,
    F32,
    F64,
}

#[derive(Debug, Clone)]
pub enum FieldValue {
    UUIDValue(Uuid),
    StringValue(String),
    U8Value(u8),
    U16Value(u16),
    U32Value(u32),
    U64Value(u64),
    I8Value(i8),
    I16Value(i16),
    I32Value(i32),
    I64Value(i64),
    F32Value(f32),
    F64Value(f64),
}

pub struct MetaData {
    pub id: Uuid,
    pub original_text: String,
    pub vector_offset: u64,
    pub custom_fields: HashMap<String, FieldValue>,
}

impl MetaData {
    pub fn new(
        id: Uuid,
        original_text: String,
        vector_offset: u64,
        custom_fields: HashMap<String, FieldValue>,
    ) -> Self {
        MetaData {
            id,
            original_text,
            vector_offset,
            custom_fields,
        }
    }

    pub fn size(&self) -> usize {
        let mut total_size = SIZE_UUID + self.dynamic_string_size(&self.original_text) + SIZE_U64;

        for (key, value) in &self.custom_fields {
            // Size of the key string and its length prefix
            total_size += self.dynamic_string_size(key);

            // Size based on the field type
            total_size += match value {
                FieldValue::UUIDValue(_) => SIZE_UUID,
                FieldValue::StringValue(s) => self.dynamic_string_size(&s),
                FieldValue::U8Value(_) => SIZE_U8,
                FieldValue::U16Value(_) => SIZE_U16,
                FieldValue::U32Value(_) => SIZE_U32,
                FieldValue::U64Value(_) => SIZE_U64,
                FieldValue::I8Value(_) => SIZE_I8,
                FieldValue::I16Value(_) => SIZE_I16,
                FieldValue::I32Value(_) => SIZE_I32,
                FieldValue::I64Value(_) => SIZE_I64,
                FieldValue::F32Value(_) => SIZE_F32,
                FieldValue::F64Value(_) => SIZE_F64,
            };
        }

        total_size
    }

    fn dynamic_string_size(&self, s: &String) -> usize {
        SIZE_U64 + s.len()
    }
}
