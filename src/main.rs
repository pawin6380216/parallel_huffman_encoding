use rayon::prelude::*;
use std::path::{Path, PathBuf};
use std::fs::File;
use std::env;
use std::io::{prelude::*, BufReader};

mod huffman_model;
mod node;
mod helpers;

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

// Checks if the given file path exists.
pub fn check_file_exists(input_path: &Path) -> bool {
    input_path.exists()
}

// Parses command-line arguments to extract input path, mode, and number of tasks.
pub fn parse_arguments(args: &[String]) -> (Option<PathBuf>, String, usize) {
    let mut input_path = None;
    let mut mode = "compress".to_string(); // Default mode is compression
    let mut num_thread = num_cpus::get(); // Default number of tasks is number of CPUs

    for i in 1..args.len() {
        match args[i].as_str() {
            "-f" => {
                if i + 1 < args.len() {
                    input_path = Some(PathBuf::from(args[i + 1].clone()));
                }
            }
            "-m" => {
                if i + 1 < args.len() {
                    mode = args[i + 1].clone();
                }
            }
            "-t" => {
                if i + 1 < args.len() {
                    num_thread = args[i + 1].parse().expect("Invalid number of tasks");
                }
            }
            _ => (),
        }
    }

    (input_path, mode, num_thread)
}

fn main() {
    // let input_path = Path::new("test_data/Large/bible.txt");

    // let num_threads = 4; // Number of threads to use for parallel processing

    // if let Err(err) = huffman_model::compress_file(&input_path) {
    //     println!("Error encountered while compressing the file: {}", err);
    //     return;
    // }

    // if let Err(err) = huffman_model::decompress_file(&input_path) {
    //     println!("Error encountered while decompressing the file: {}", err);
    //     return;
    // }

    let args: Vec<String> = env::args().collect();

    let (input_path, mode, num_threads) = parse_arguments(&args);

    // Determine input path based on user input
    let input_path = match input_path {
        Some(path) => path,
        None => {
            println!("Error: No input file specified.");
            return;
        }
    };

    // Check if the specified file exists
    if !check_file_exists(&input_path) {
        println!("Error: File does not exist.");

        return;
    }

    // Configure Rayon's thread pool builder
    rayon::ThreadPoolBuilder::new()
        .num_threads(num_threads)
        .build_global()
        .expect("Failed to build thread pool");

    // Perform the operation based on the specified mode
    match mode.as_str() {
        "compress" => match huffman_model::compress_file(&input_path) {
            Ok(_) => println!("File compressed successfully."),
            Err(err) => println!("Error compressing file: {}", err),
        },
        "decompress" => match huffman_model::decompress_file(&input_path) {
            Ok(_) => println!("File decompressed successfully."),
            Err(err) => println!("Error decompressing file: {}", err),
        },
        _ => eprintln!("Error: Invalid mode: {}", mode),
    }
}
