use rayon::prelude::*; 
use serde_json; 
use std::collections::HashMap; 
use std::fs::File; 
use std::io::{self, BufReader, Read, Write}; 
use std::path::Path; 
use std::sync::Mutex; 

use crate::node::{Node, EncodedData};
use crate::helpers::{build_huffman_tree, generate_codes, encode_text, decode_text};

// Writes encoded data to a file using JSON serialization.
fn write_encoded_file(output_path: &Path, encoded_data: &EncodedData) -> io::Result<()> {
    println!("Writing encoded data to file...");

    let file = File::create(output_path)?;
    
    // Serialize encoded_data to JSON and write to file
    serde_json::to_writer(file, encoded_data)?; 

    println!("Encoded data written to file successfully.");
    Ok(())
}

// Compresses the contents of a file.
pub fn compress_file(input_path: &Path) -> io::Result<()> {
    println!("Starting compression...");

    // Read input file into a string
    let file_content = File::open(input_path)?;
    let mut reader = BufReader::new(file_content);
    let mut text = String::new();
    reader.read_to_string(&mut text)?;

    let freq_map: HashMap<char, usize> = text
        .par_chars() 
        .fold(HashMap::new, |mut acc, char| { 
            *acc.entry(char).or_insert(0) += 1; // Insert entry of 0 into HashMap or increment count
            acc
        })
        .reduce(|| HashMap::new(), |mut acc, map| { 
            for (char, count) in map {
                *acc.entry(char).or_insert(0) += count;
            }
            acc
        });

    // Build Huffman tree from frequency map
    println!("Building Huffman tree from frequency map...");
    let huffman_tree = build_huffman_tree(&freq_map);

    // Generate Huffman codes for each character
    // Don't know any other way to do this.
    let codes = Mutex::new(HashMap::new()); 

    println!("Generating Huffman codes for each characters...");
    generate_codes(&huffman_tree, String::new(), &codes);

    // Encode text using Huffman codes
    println!("Encoding text using Huffman codes...");
    let encoded_text = encode_text(&text, &codes.lock().unwrap());

    // Prepare encoded data structure
    let encoded_data = EncodedData {
        codes: codes.lock().unwrap().clone(),
        encoded_text,
    };

    // Generate output file path
    let output_path = input_path.with_file_name(format!(
        "{}_compressed.txt",
        input_path.file_stem()
                .unwrap()
                .to_string_lossy()
    ));

    println!("Writing encoded data to output file: {:?}", output_path);
    write_encoded_file(&output_path, &encoded_data)?;

    println!("Compression completed successfully.");
    Ok(())
}

// Reads encoded data from a file using JSON deserialization.
fn read_encoded_file(input_path: &Path) -> io::Result<EncodedData> {
    println!("Reading encoded data from file...");

    let file = File::open(input_path)?;
    let encoded_data: EncodedData = serde_json::from_reader(file)?; 

    println!("Encoded data read from file successfully.");
    Ok(encoded_data)
}

// Decompresses a file that was previously compressed using Huffman coding.
pub fn decompress_file(input_path: &Path) -> io::Result<()> {
    println!("Starting decompression...");

    let encoded_data = read_encoded_file(input_path)?;

    // Decode text using Huffman codes
    println!("Decoding text using Huffman codes...");
    let decoded_text = decode_text(&encoded_data.encoded_text, &encoded_data.codes);

    // Generate output file path
    let output_path = input_path.with_file_name(format!(
        "{}_decompressed.txt",
        input_path.file_stem()
                .unwrap()
                .to_string_lossy()
    ));

    // Write decoded text to output file
    println!("Writing decoded text to output file: {:?}", output_path);

    let mut file = File::create(output_path)?;
    file.write_all(decoded_text.as_bytes())?;

    println!("Decompression completed successfully.");
    Ok(())
}
