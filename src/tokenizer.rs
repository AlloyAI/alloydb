use anyhow::Result;
use tokenizers::models::bpe::BPE;
use tokenizers::tokenizer::Tokenizer;

pub fn bpe_tokenize_string(input: &str) -> Result<Vec<u32>> {
    let bpe_builder = BPE::from_file("./data/vocab.json", "./data/merges.txt");
    // TODO: Handle errors with custom anyhow error return and ?
    let bpe = bpe_builder.dropout(0.1).build().unwrap();
    let tokenizer = Tokenizer::new(bpe);

    let encoding = tokenizer.encode(input, false).unwrap();

    Ok(encoding.get_ids().to_vec())
}

//TODO: Add more tokenizers
