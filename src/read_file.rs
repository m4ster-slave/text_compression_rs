use crate::*;

pub fn read_file_string_into_huffman_array(file_name: &str) -> Vec<HuffmanTree> {
    let contents = fs::read(file_name)
        .expect("Should have been able to read the file");

    let mut map: HashMap<u8, u32> = HashMap::new();

    //count each character and input it into a hashmap as key/value pair
    for &value in &contents {
        if let Some(count) = map.get_mut(&value) {
            *count += 1
        } else {
            map.insert(value, 1);
        }
    }

    //sort entries from hashmap into vector TODO: input it directly into vector?
    let mut sorted_entries: Vec<_> = map.iter().collect();
    sorted_entries.sort_by(|a, b| b.1.cmp(a.1));

    for (&c, &f) in &sorted_entries {
        if c == 10{
            println!("character: {}, freq: {}", "\\n", f);
        } else if c == 32{
            println!("character: {}, freq: {}", "<space>", f);
        } else{
            println!("character: {}, freq: {}", c as char, f);
        }
    }

    let mut huffman_trees: Vec<HuffmanTree> = Vec::new();

    // create huffman root nodes from sorted array
    for (&character, &freq) in sorted_entries {
        huffman_trees.push(HuffmanTree::new_char(character as char, freq));
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
