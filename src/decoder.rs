use crate::*;

pub fn decode(content: String, huffman_root: HuffmanTree) -> String{
    let mut decoded_string: String = String::new();

    let mut current_node = &huffman_root;

    for value in content.as_bytes() {
        if *value == '0' as u8 {
            //curr -> left 
            if let Some(left) = &current_node.left_node {
                current_node = left;
            }
        } else {
            //curr -> right
            if let Some(right) = &current_node.right_node {
                current_node = right;
            }
        }

        if current_node.is_leaf() {
            decoded_string.push(current_node.value);
            // print!("{}   ", current_node.value);
            current_node = &huffman_root;
        }
    }


    decoded_string
}

pub fn decode_encoded_tree(encoded_string: &str) -> Vec<HuffmanTree> {
        let mut nodes = Vec::new();
        let mut i = 0;

        while i < encoded_string.len() {
            // Extract 8 bits for the character
            let char_binary = &encoded_string[i..i + 8];
            let value = u8::from_str_radix(char_binary, 2).unwrap() as char;

            // Extract 32 bits for the frequency
            let freq_binary = &encoded_string[i + 8..i + 40];
            let frequency = u32::from_str_radix(freq_binary, 2).unwrap();

            // Create a new HuffmanTree node
            nodes.push(HuffmanTree::new_char(value, frequency));

            i += 40; // Move to the next pair of character and frequency
        }

        nodes
    }
