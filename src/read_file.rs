use crate::*;

pub fn read_file_string(file_name: &str) -> Vec<HuffmanTree> {
    let contents = fs::read(file_name)
        .expect("Should have been able to read the file");

    let mut map: HashMap<u8, u32> = HashMap::new();

    for &value in &contents {
        if let Some(count) = map.get_mut(&value) {
            *count += 1
        } else {
            map.insert(value, 1);
        }
    }

    //sort entries into vector
    let mut sorted_entries: Vec<_> = map.iter().collect();
    sorted_entries.sort_by(|a, b| b.1.cmp(a.1));
    println!("entries: {:?}\n\n", sorted_entries);

    let mut huffman_trees: Vec<HuffmanTree> = Vec::new();

    // Iterate over the sorted vector
    for (character, freq) in sorted_entries {
        huffman_trees.push(HuffmanTree::new_char(*character as char, *freq));
    }

    huffman_trees
}

pub fn read_file_bits(file_name: &str) -> String {
    let mut file = match File::open(file_name) {
        Ok(file) => file,
        Err(_) => panic!("Failed to open file."),
    };

    // Read the file into a vector of u8
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).expect("should have been able to read the file");

    let mut bit_string = String::new();
    for byte in buffer {
        let binary_string = format!("{:08b}", byte);
        bit_string.push_str(&binary_string);
    }
    
    bit_string
}
