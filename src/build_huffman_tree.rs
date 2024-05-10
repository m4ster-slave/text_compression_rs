//TODO: has to be odered from big -> small freq
#[derive(Debug, Clone)]
pub struct HuffmanTree {
    value: char,
    frequency: u32,
    huffman_code: String,
    left_node: Option<Box<HuffmanTree>>,
    right_node: Option<Box<HuffmanTree>>,
}

impl HuffmanTree {
    pub fn build_tree(mut unique_chars: Vec<HuffmanTree>) -> HuffmanTree {
        while unique_chars.len() > 2 {
            // getting last two elements in ordered vec
            let left_node = unique_chars.pop().unwrap().clone();
            let right_node = unique_chars.pop().unwrap().clone();

            // create new node from leave nodes
            let new_node = Self::add(left_node, right_node);

            //insert the new node
            for i in 0..unique_chars.len() {
                if unique_chars[i].frequency < new_node.frequency {
                    unique_chars.insert(i, new_node);
                    break;
                }
            }
        }

        if unique_chars.len() == 2 {
            let left_node = unique_chars[1].clone(); //always left node bc smaller
            let right_node = unique_chars[0].clone();
            return Self::add(left_node, right_node);
        }

        unique_chars[0].clone()
    }

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

    pub fn print(tree: HuffmanTree) {
        // character    code-word
        println!("char\t freq \t codes\t\t ascii");
        Self::print_recursive(&tree);
    }

    fn print_recursive(tree: &HuffmanTree) {
        if let Some(left) = &tree.left_node {
            Self::print_recursive(left);
        }

        if tree.value as u8 == 10 {
            println!(
                "\'\\n\'\t {}\t {}\t\t {}",
                tree.frequency, 
                tree.huffman_code,
                tree.value as u8,
            );           
        } else if tree.value != '\0' {
            // Print the current node
            // TODO: print '\n' instead of newline
            println!(
                "{}\t {}\t {}\t\t {}",
                tree.value, 
                tree.frequency, 
                tree.huffman_code,
                tree.value as u8,
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

        let mut huffman_tree = HuffmanTree::build_tree(freq_arr);
        HuffmanTree::generate_codes(&mut huffman_tree, String::new());

        HuffmanTree::print(huffman_tree.clone());

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
