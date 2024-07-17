use rayon::prelude::*;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::fs::File;
use std::io::{prelude::*, BufReader};

mod huffman_model;
mod node;
mod helpers;

use crate::huffman_model::{compress_file};
use std::path::Path;

/* ————— Analysis tools ————— */

// Return the number of words in a file
fn count_words(file_path: &str) -> u32 {
    let file = match File::open(file_path) {
        Ok(file) => file,
        Err(_) => {
            eprintln!("Error encountered: file not found or could not be opened.");
            return 0;
        }
    };    

    let file_reader  = BufReader::new(file);

    let word_count: u32 = file_reader
        .lines() 
        .par_bridge() 
        .map(|line_result| {
            let line = match line_result {
                Ok(line) => line,
                Err(_) => {
                    eprintln!("Error encountered: error reading line.");
                    return 0; 
                }
            };

            line.split_whitespace()
                .filter(|word| !word.is_empty())
                .count() as u32
        })
        .sum();

    return word_count;
}

fn main() {
    let input_path = Path::new("test_data/Large/bible_compressed.txt");
    let num_threads = 4;

    // Configure Rayon's thread pool builder
    rayon::ThreadPoolBuilder::new()
        .num_threads(num_threads)
        .build_global()
        .expect("Failed to build thread pool");

    // if let Err(err) = huffman_model::compress_file(&input_path) {
    //     println!("Error encountered while compressing the file: {}", err);
    //     return;
    // }

    // if let Err(err) = huffman_model::decompress_file(&input_path) {
    //     println!("Error encountered while decompressing the file: {}", err);
    //     return;
    // }

    println!("File compressed successfully.");
}
