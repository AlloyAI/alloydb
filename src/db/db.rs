use crate::db::metadata::MetaData;
use anyhow::Result;
use byteorder::{BigEndian, WriteBytesExt};
use fs2::FileExt;
use std::{
    fs::File,
    io::{Seek, SeekFrom, Write},
};

// AlloyDB Database File
const MAGIC_NUMBER: &[u8; 4] = b"ADBF";
const VERSION: u16 = 1;

pub struct AlloyDB {
    file_path: String,
}

struct AlloyDBHeader {
    magic_number: [u8; 4],
    version: u16,
    metadata_offset: u64,
    index_offset: u64,
    vector_data_offset: u64,
    total_records: u64,
}

struct AlloyDBOffsets {
    initial_metadata_offset: u64,
    initial_index_offset: u64,
    initial_vector_data_offset: u64,
}

impl AlloyDB {
    pub fn new(file_path: String) -> Self {
        Self { file_path }
    }

    pub fn create(&self, metadata: MetaData) -> Result<()> {
        let mut file = File::create(&self.file_path)?;

        // Lock the file for exclusive access
        file.lock_exclusive()?;

        let header = AlloyDBHeader {
            magic_number: *MAGIC_NUMBER,
            version: VERSION,
            metadata_offset: 0,
            index_offset: 0,
            vector_data_offset: 0,
            total_records: 0,
        };

        // Write header information
        file.write_all(MAGIC_NUMBER)?;
        file.write_all(&VERSION.to_be_bytes())?;

        // Initial offsets
        let initial_metadata_offset: u64 = file.seek(SeekFrom::Current(0))? + 24;
        let initial_index_offset: u64 = initial_metadata_offset + metadata.size() as u64;
        let initial_vector_data_offset: u64 = initial_index_offset;

        // Write our offsets to the file.
        file.write_u64::<BigEndian>(initial_metadata_offset)?;
        file.write_u64::<BigEndian>(initial_index_offset)?;
        file.write_u64::<BigEndian>(initial_vector_data_offset)?;
        // Total records, starting at 0
        file.write_u64::<BigEndian>(0)?;

        // Unlock the file
        file.unlock()?;

        Ok(())
    }
}
