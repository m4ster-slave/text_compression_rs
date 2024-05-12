mod build_huffman_tree;
mod read_file;
mod encoder;
mod decoder;

use self::build_huffman_tree::*;
use self::read_file::*;
use self::encoder::*;
use self::decoder::*;

use std::fs;
use std::collections::HashMap;

fn main() {
    let mut huffman_tree = HuffmanTree::build_tree(read_file("test.txt"));
    HuffmanTree::generate_codes(&mut huffman_tree, String::new());
    
    HuffmanTree::print(huffman_tree.clone());

    let encoded_string = encode("test.txt", huffman_tree.clone());
    println!("ENCODED STRING: \n{}\n\n\n", encoded_string);

    let decoded_string = decode(encoded_string, huffman_tree);
    println!("DECODED STRING: \n{}", decoded_string);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {}
}
