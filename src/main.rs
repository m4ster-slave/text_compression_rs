mod build_huffman_tree;
mod read_file;
mod encoder;
mod decoder;

use self::build_huffman_tree::*;
use self::read_file::*;
use self::encoder::*;
use self::decoder::*;

use std::fs;
use std::fs::File;
use std::io::{Write, Read};
use std::collections::HashMap;

fn main() {
    let mut huffman_tree = HuffmanTree::build_tree(read_file_string("test.txt"));
    HuffmanTree::generate_codes(&mut huffman_tree, String::new());
    HuffmanTree::print_codes(huffman_tree.clone());

    let encoded_string = encode("test.txt", huffman_tree.clone());
    println!("ENCODED STRING: \n{}\n", encoded_string.clone());
    write_into_file_as_bits("test.txt", encoded_string.clone());

    let file_bits = read_file_bits("test.rhc");

    print!("{file_bits}\n");

    let decoded_string = decode(file_bits, huffman_tree);
    println!("DECODED STRING: \n{}", decoded_string);
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test() {}
// }
