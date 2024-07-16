use rayon::prelude::*; 
use serde_json; 
use std::collections::HashMap; 
use std::fs::File; 
use std::io::{self, BufReader, Read, Write}; 
use std::path::Path; 
use std::sync::Mutex; 

pub fn build_huffman_tree(freq_map: &HashMap<char, usize>) -> Node {
    println!("Generating Huffman tree...");
    
    let mut heap: std::collections::BinaryHeap<Node> = freq_map
        .par_iter()
        .map(|(&char, &freq)| Node::new_leaf(freq, char))
        .collect();

    // Build Huffman tree from frequency map
    while (heap.len() > 1) {
        let left = heap.pop().unwrap();
        let right = heap.pop().unwrap();
        let merged = Node::new_internal(left.freq + right.freq, left, right);

        heap.push(merged);
    }

    println!("Huffman tree built successfully.");

    heap.pop().unwrap()
}

// Returns the output file path for a given input file path.
pub fn get_output_file_path(input_path: &Path) -> Path {
    let output_path = input_path.with_file_name(format!(
        "{}_compressed.txt",
        input_path.file_stem().unwrap().to_string_lossy()
    ));

    output_path
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
            *acc.entry(char).or_insert(0) += 1;
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
        input_path.file_stem().unwrap().to_string_lossy()
    ));

    println!("Writing encoded data to output file: {:?}", output_path);
    write_encoded_file(&output_path, &encoded_data)?;

    println!("Compression completed successfully.");
    Ok(())
}

// Encodes input text.
pub fn encode_text(text: &str, codes: &HashMap<char, String>) -> String {
    println!("Encoding text...");

    let encoded_text = text
        .par_chars() 
        .map(|char| codes.get(&char).unwrap().as_str())
        .collect::<Vec<&str>>()
        .join(""); 

    println!("Text encoded successfully.");
    encoded_text
}
