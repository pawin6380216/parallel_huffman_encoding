<!-- ABOUT THE PROJECT -->
# Parallel Data Compression Algorithm using parallel Huffman coding

## Algorithm Overview

- **Frequency analysis**: Calculate the frequency of each character in the input text.
- **Huffman Tree Construction**: Build a tree where characters with higher frequencies are closer to the root.
- **Code Generation**: Assign binary codes to each character based on their position in the Huffman tree.
- **Encoding and Decoding**: Encode the input text using the generated Huffman codes for compression and decode it back to the original text during decompression.

------

## Parallelism Overview
- **Frequency Analysis**: Parallel computation of character frequencies across multiple threads.
- **Huffman Tree Construction**: Concurrent processing of nodes in the Huffman tree to speed up tree building.
- **Encoding and Decoding**: Parallel processing of text encoding and decoding using multiple threads for faster execution.

------

##  Project Structure

    .
    ├── src
    │   ├── main.rs                 # File operations and mode selection (compression / decompression)
    │   ├── huffman_model           # Main implementation of Huffman encoding and decoding algorithm
    │   ├── node                    # Node definition for the Huffman tree and encoded data for serialized data storage
    |   ├── helpers                 # Utility functions
    |   └── old_implementations     # Archieved old implementation of Huffman  
    |
    |
    ├── test_data
    |   ├── LoremIpsum.txt
    |   └── Large
    |       ├── bible.txt
    |       └── world192.txt
    |
    ├── Parallel Data Compression.pdf
    ├── Cargo.lock
    ├── Cargo.toml
    └── README.md

------

<!-- USAGE EXAMPLES -->
## Usage
### Compression

    cargo run -- -m compress -f input.txt -t num_threads

`-m compress` : using compression mode \
`-f input.txt` : the input text for file compression \
`-t num_threads` : the numbers of threads for parallel processing

### Decompression

    cargo run -- -m decompress -f input_compressed.txt -t num_threads

`-m compress` : using decompression mode \
`-f input.txt` : the input text for file decompression \
`-t num_threads` : the numbers of threads for parallel processing