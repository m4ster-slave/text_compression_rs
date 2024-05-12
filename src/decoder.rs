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

        if current_node.is_leave() {
            decoded_string.push(current_node.value);
            // print!("{}   ", current_node.value);
            current_node = &huffman_root;
        }
    }


    decoded_string
}
