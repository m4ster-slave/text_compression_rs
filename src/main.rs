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
use std::env;

fn main() {
    
    let args: Vec<String> = env::args().collect();
    let mut i = 2;

    if args[1] == "-c" {
        while i < args.len() {
            // read each characters frequency into an array of the huffman tree objects 
            // read character frequencies into array and build huffman tree from that
            let mut freq_array = read_file_string_into_huffman_array(&args[i]);
            freq_array = HuffmanTree::sort_frequency_array(freq_array);
            let mut huffman_tree = HuffmanTree::build_huffman_tree(freq_array.clone());


            // generate each characters unique huffman code
            HuffmanTree::generate_codes(&mut huffman_tree, String::new());
            HuffmanTree::print_codes(huffman_tree.clone());
            
            // read the file and 
            let mut encoded_string = encode(&args[i], huffman_tree.clone());
            println!("ENCODED STRING: \n{}", encoded_string.clone());


            // add encoded huffman into string --> frequency
            encoded_string = format!("{}{}", huffman_tree.get_encoded_tree(), encoded_string);
            

            write_into_file_as_bits(&args[i], encoded_string.clone());
         
            i += 1;
        }   
    } else if args[1] == "-x" {
        while i < args.len() {
            let file_bits = read_file_bits(&args[i]);
            print!("READ FILE: \n{file_bits}\n");
            
            // getting the first u32 int from the file that says how many character we have in the tree
            let mut file_bits = file_bits.split_at(32);
            let total_chars = u32::from_str_radix(file_bits.0, 2).unwrap();

            // split the encoded frequency vector and the compressed file
            file_bits = file_bits.1.split_at((total_chars * 40) as usize);

         
            // same steps as above
            let mut freq_array = decode_encoded_tree(file_bits.0);
            freq_array = HuffmanTree::sort_frequency_array(freq_array);
            let mut huffman_tree = HuffmanTree::build_huffman_tree(freq_array.clone());
            HuffmanTree::generate_codes(&mut huffman_tree, String::new());
            HuffmanTree::print_codes(huffman_tree.clone());

            // decode the string with the newly constructed tree
            let decoded_string = decode(file_bits.1.to_owned(), huffman_tree.clone());
            println!("\nDECODED STRING: \n\"{}\"", decoded_string);
           
            let file_name = args[i].replace(".rhc", ".txt");

            let mut file = File::create(file_name).unwrap();
            write!(file, "{}", decoded_string).unwrap();

            i += 1;
        }
    } else {
        println!("Huffman tree compression for text files:");
        println!("use -c <*.txt> to compress");
        println!("use -x <*.rhc> to decompress");
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test() {}
// }
