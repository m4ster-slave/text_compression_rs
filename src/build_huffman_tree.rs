use std::u32;

//TODO: has to be odered from big -> small freq
#[derive(Debug, Clone)]
pub struct HuffmanTree {
    pub(crate) value: char,
    frequency: u32,
    huffman_code: String,
    pub(crate) left_node: Option<Box<HuffmanTree>>,
    pub(crate) right_node: Option<Box<HuffmanTree>>,
}

impl HuffmanTree {
    
    pub fn sort_frequency_array(mut node_arr: Vec<HuffmanTree>) -> Vec<HuffmanTree> {
        node_arr.sort_by(|a, b| {
            a.frequency.cmp(&b.frequency)
                .then_with(|| a.value.cmp(&b.value))
        });

        node_arr
    }

    // Helper function to convert a frequency to a binary string TODO: Do this with function templates
    fn frequency_to_binary(frequency: u32) -> String {
        format!("{:032b}", frequency)
    }
    fn char_to_binary(value: char) -> String {
        format!("{:08b}", value as u8)
    }

    // Recursive function to traverse the Huffman tree and collect the encoded pairs
    fn get_encoded_huffman_tree(&self, encoded_pairs: &mut String, total_characters: &mut u32) {
        if self.is_leaf() {
            // Convert frequency and value to binary and append to the result string
            let binary_frequency = HuffmanTree::frequency_to_binary(self.frequency);
            let binary_value = HuffmanTree::char_to_binary(self.value);
            encoded_pairs.push_str(&format!("{}{}", binary_value, binary_frequency));
            println!("{}\t{}", binary_value, binary_frequency);
            *total_characters = *total_characters + 1;
        } else {
            // Recursively traverse the left and right subtrees
            if let Some(ref left) = self.left_node {
                left.get_encoded_huffman_tree(encoded_pairs, total_characters);
            }
            if let Some(ref right) = self.right_node {
                right.get_encoded_huffman_tree(encoded_pairs, total_characters);
            }
        }
    }

    // Public function to start the traversal and get the encoded tree
    pub fn get_encoded_tree(&self) -> String {
        let mut encoded_pairs = String::new();
        let mut total_characters: u32 = 0;
        self.get_encoded_huffman_tree(&mut encoded_pairs, &mut total_characters);

        // push in the number of leaf nodes in the beginnning of the tree
        let total_characters_as_bin = HuffmanTree::frequency_to_binary(total_characters);
        encoded_pairs = format!("{}{}", total_characters_as_bin, encoded_pairs);
        
        encoded_pairs
    }

    pub fn is_leaf(&self) -> bool {
        if self.left_node.is_none() && self.right_node.is_none() {
            return true;
        }

        false
    }

    pub fn get_code(&self, target: char) -> Option<String> {
        if self.value == target {
            // If the current node is the target character, return its Huffman code
            return Some(self.huffman_code.clone());
        } else {
            // Recursively search in the left and right subtrees
            if let Some(ref left) = self.left_node {
                if let Some(code) = left.get_code(target) {
                    return Some(code);
                }
            }
            if let Some(ref right) = self.right_node {
                if let Some(code) = right.get_code(target) {
                    return Some(code);
                }
            }
            // If the character is not found in the subtree rooted at this node, return None
            return None;
        }
    }

    // pub fn build_huffman_tree(mut unique_chars: Vec<HuffmanTree>) -> HuffmanTree {
    //     while unique_chars.len() > 2 {
    //         // getting last two elements in ordered vec
    //         let left_node = unique_chars.pop().unwrap().clone();
    //         let right_node = unique_chars.pop().unwrap().clone();
    //
    //         // create new node from leave nodes
    //         let new_node = Self::add(left_node, right_node);
    //
    //         //insert the new node
    //         for i in 0..unique_chars.len() {
    //             if unique_chars[i].frequency < new_node.frequency {
    //                 unique_chars.insert(i, new_node);
    //                 break;
    //             }
    //         }
    //     }
    //
    //     if unique_chars.len() == 2 {
    //         let left_node = unique_chars[1].clone(); //always left node bc smaller
    //         let right_node = unique_chars[0].clone();
    //         return Self::add(left_node, right_node);
    //     }
    //
    //     // return single root node
    //     unique_chars[0].clone()
    // }

    pub fn build_huffman_tree(mut unique_chars: Vec<HuffmanTree>) -> HuffmanTree {
        // Sort the vector by frequency in non-decreasing order

        while unique_chars.len() > 1 {
            // Get the two nodes with the smallest frequencies
            let left_node = unique_chars.remove(0);
            let right_node = unique_chars.remove(0);

            // Create a new node from the two nodes
            let new_node = Self::add(left_node, right_node);

            // Find the correct position to insert the new node to maintain the sorted order
            let pos = unique_chars
                .binary_search_by(|node| node.frequency.cmp(&new_node.frequency))
                .unwrap_or_else(|e| e);
            unique_chars.insert(pos, new_node);
        }

        // Return the single remaining node, which is the root of the Huffman tree
        unique_chars.pop().unwrap()
    }

    // recursive function to generate the huffman code
    pub fn generate_codes(tree: &mut HuffmanTree, codes: String) {
        if let Some(ref mut left) = tree.left_node {
            Self::generate_codes(left, format!("{}0", codes));
        }

        if tree.value != '\0' {
            tree.huffman_code = codes.clone();
        }

        if let Some(ref mut right) = tree.right_node {
            Self::generate_codes(right, format!("{}1", codes));
        }
    }

    fn add(left_node: HuffmanTree, right_node: HuffmanTree) -> HuffmanTree {
        Self {
            value: '\0',
            frequency: left_node.frequency + right_node.frequency,
            huffman_code: String::new(),
            left_node: Some(Box::new(left_node)),
            right_node: Some(Box::new(right_node)),
        }
    }

    pub fn new_char(value: char, frequency: u32) -> HuffmanTree {
        Self {
            value,
            frequency,
            huffman_code: String::new(),
            left_node: None,
            right_node: None,
        }
    }

    pub fn print_codes(tree: HuffmanTree) {
        // character    code-word
        println!("char\t freq \t codes\t\t ascii");
        println!("--------------------------------------------");
        Self::print_recursive(&tree);
    }

    fn print_recursive(tree: &HuffmanTree) {
        if let Some(left) = &tree.left_node {
            Self::print_recursive(left);
        }

        // if we encounter a newline character
        if tree.value as u8 == 10 {
            println!(
                "\'\\n\'\t {}\t {}\t\t {}",
                tree.frequency, tree.huffman_code, tree.value as u8,
            );
        } else if tree.value != '\0' {
            println!(
                "{}\t {}\t {}\t\t {}",
                tree.value, tree.frequency, tree.huffman_code, tree.value as u8,
            );
        }

        if let Some(right) = &tree.right_node {
            Self::print_recursive(right);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tre_generation() {
        let freq_arr = vec![
            HuffmanTree::new_char('f', 45),
            HuffmanTree::new_char('e', 16),
            HuffmanTree::new_char('d', 13),
            HuffmanTree::new_char('c', 12),
            HuffmanTree::new_char('b', 9),
            HuffmanTree::new_char('a', 5),
        ];

        let mut huffman_tree = HuffmanTree::build_huffman_tree(freq_arr);
        HuffmanTree::generate_codes(&mut huffman_tree, String::new());

        HuffmanTree::print_codes(huffman_tree.clone());

        if let Some(left) = &huffman_tree.left_node {
            assert_eq!(left.huffman_code, String::from("0"));
        }

        if let Some(right_1) = &huffman_tree.right_node {
            assert_eq!(right_1.huffman_code, String::from(""));
            if let Some(right_2) = &right_1.right_node {
                assert_eq!(right_2.huffman_code, String::from(""));
                if let Some(right_3) = &right_2.right_node {
                    assert_eq!(right_3.huffman_code, String::from("111"));
                }
            }
        }
    }
}
