mod build_huffman_tree;
mod read_file;

use self::build_huffman_tree::*;
use self::read_file::*;

use std::fs;
use std::collections::HashMap;

fn main() {
    let mut huffman_tree = HuffmanTree::build_tree(read_file("test.txt"));
    HuffmanTree::generate_codes(&mut huffman_tree, String::new());
    
    HuffmanTree::print(huffman_tree);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {}
}
