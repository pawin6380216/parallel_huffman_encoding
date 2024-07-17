use rayon::prelude::*; 
use std::collections::HashMap; 
use std::sync::Mutex; 

use crate::node::Node; // Import Node struct from models module

// Builds the Huffman tree 
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

// Recursively generates Huffman codes from the Huffman tree nodes.
pub fn generate_codes(node: &Node, prefix: String, codes: &Mutex<HashMap<char, String>>) {
    if let Some(char) = node.char {
        codes.lock()
            .unwrap()
            .insert(char, prefix.clone()); // Insert character and its Huffman code into the HashMap

    } else {
        // Traverse left child with '0' prefix
        if let Some(ref left) = node.left {
            generate_codes(left, format!("{}0", prefix), codes); 
        }
        // Traverse right child with '1' prefix
        if let Some(ref right) = node.right {
            generate_codes(right, format!("{}1", prefix), codes); 
        }
    }
}

// Decodes Huffman encoded text using pre-generated Huffman codes.
pub fn decode_text(encoded_text: &str, codes: &HashMap<char, String>) -> String {
    println!("Decoding text...");
    let mut reverse_codes: HashMap<String, char> = HashMap::new();

    for (char, code) in codes {
        reverse_codes.insert(code.clone(), *char); 
    }

    let mut decoded_text = String::new();
    let mut current_code = String::new();

    for bit in encoded_text.chars() {
        current_code.push(bit); 

        if let Some(&char) = reverse_codes.get(&current_code) {
            decoded_text.push(char); 
            current_code.clear(); 
        }
    }

    println!("Text decoded successfully.");
    
    decoded_text
}