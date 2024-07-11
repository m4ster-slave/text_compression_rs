mod build_huffman_tree;
mod decoder;
mod encoder;
mod read_file;

use self::build_huffman_tree::*;
use self::decoder::*;
use self::encoder::*;
use self::read_file::*;

use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::{Read, Write};

fn main() {
    // read each characters frequency into an array of the huffman tree objects 
    // read character frequencies into array and build huffman tree from that
    let mut huffman_tree = HuffmanTree::build_huffman_tree(read_file_string_into_huffman_array("test.txt"));


    // generate each characters unique huffman code
    HuffmanTree::generate_codes(&mut huffman_tree, String::new());
    HuffmanTree::print_codes(huffman_tree.clone());
    
    // read the file and 
    let encoded_string = encode("test.txt", huffman_tree.clone());
    println!("ENCODED STRING: \n{}", encoded_string.clone());

    write_into_file_as_bits("test.txt", encoded_string.clone());

    let file_bits = read_file_bits("test.rhc");

    print!("FILE INPUT: \n{file_bits}\n");

    let decoded_string = decode(file_bits, huffman_tree);
    println!("\nDECODED STRING: \n\"{}\"", decoded_string);
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test() {}
// }
