use crate::tokenizer::bpe_tokenize_string;
use anyhow::Result;
use rkyv::{Archive, Deserialize, Serialize};
use std::{
    fs::{File, OpenOptions},
    io::{BufRead, BufReader, BufWriter, Read, Write},
};
use uuid::Uuid;

#[derive(Archive, Deserialize, Serialize, Debug, PartialEq)]
pub struct Vector {
    pub id: String,
    pub original_text: String,
    pub vector: Vec<f32>,
}

impl Vector {
    pub fn new(id: String, original_text: String, vector: Vec<f32>) -> Self {
        Self {
            id,
            original_text,
            vector,
        }
    }

    pub fn save(&self, filename: &str) -> Result<()> {
        let bytes = rkyv::to_bytes::<_, 256>(self)?;

        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(filename)?;
        let mut writer = BufWriter::new(file);

        // First write the size of the serialized Vector
        writer.write_all(&(bytes.len() as u64).to_le_bytes())?;

        // Then write the serialized Vector
        writer.write_all(&bytes)?;

        Ok(())
    }
}

fn one_hot_encode(token_id: u32, vocab_size: usize) -> Vec<f32> {
    let mut vect = vec![0.0f32; vocab_size];
    vect[token_id as usize] = 1.0;

    vect
}

fn one_hot_encode_batch(token_ids: Vec<u32>, vocab_size: usize) -> Vec<Vec<f32>> {
    let mut vect = Vec::new();
    for token_id in token_ids {
        vect.push(one_hot_encode(token_id, vocab_size));
    }

    vect
}

fn average_embeddings(embeddings: Vec<Vec<f32>>) -> Vec<f32> {
    let token_count = embeddings.len() as f32;
    let mut averaged = vec![0.0f32; embeddings[0].len()];

    for embedding in embeddings {
        for (i, &val) in embedding.iter().enumerate() {
            averaged[i] += val;
        }
    }

    for val in averaged.iter_mut() {
        *val /= token_count;
    }

    averaged
}

pub fn vectorize_text(text: &str) -> Result<Vector> {
    let tokenized = bpe_tokenize_string(text)?;
    // NOTE: This vocab size is temporary
    let encoded_vector = one_hot_encode_batch(tokenized, 100000);
    let merged_vector = average_embeddings(encoded_vector);

    Ok(Vector::new(
        Uuid::new_v4().to_string(),
        text.to_string(),
        merged_vector,
    ))
}

/// Read vectors from file
pub fn read_vectors_from_file(filename: &str) -> Result<Vec<Vector>> {
    let file = File::open(filename)?;
    let mut reader = BufReader::new(file);
    let mut vectors: Vec<Vector> = Vec::new();

    while let Ok(size_bytes) = reader.fill_buf() {
        if size_bytes.len() < std::mem::size_of::<u64>() {
            break; // Not enough bytes left to read a size, so we're done
        }

        // Read the size of the next serialized Vector
        let serialized_vector_size = u64::from_le_bytes(size_bytes[0..8].try_into().unwrap());

        reader.consume(std::mem::size_of::<u64>()); // Consume the size bytes

        let mut buffer = vec![0u8; serialized_vector_size as usize];
        reader.read_exact(&mut buffer)?;

        // Deserialize the Vector with explicit type hint
        let view: &rkyv::Archived<Vector> = unsafe { rkyv::archived_root::<Vector>(&buffer) };
        let deserialized_vector: Vector = view.deserialize(&mut rkyv::Infallible)?;
        vectors.push(deserialized_vector);
    }

    Ok(vectors)
}
