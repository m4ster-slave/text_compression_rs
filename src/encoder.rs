use crate::*;

pub fn encode(file_name: &str, huffman_tree: HuffmanTree) -> String {
    // read in each character again
    // look up code in tree
    // append code to array
    let mut encoded_string: String = String::new();

    let contents = fs::read(file_name).expect("Should have been able to read the file");

    for value in contents {
        encoded_string.push_str(&huffman_tree.get_code(value as char).unwrap());
    }

    encoded_string
}

pub fn write_into_file_as_bits(filename: &str, string: String) {
    let filename = create_new_file_name(filename);

    let mut file = File::create(filename).unwrap();

    let mut byte_counter = 0;
    let mut byte_to_write: u8 = 0;

    for value in string.as_bytes() {
        // creating a byte to write
        byte_to_write = match value {
            b'1' => (byte_to_write << 1) | 1,  // change lsb to 1
            b'0' => byte_to_write << 1, // change lsb to 0
            _ => panic!("Invalid bit character"),
        };

        byte_counter += 1;

        if byte_counter == 8 {
            let _ = file.write_all(&[byte_to_write]);
            byte_counter = 0;
        }
    }
    // Write any remaining bits
    if byte_counter > 0 {
        // Shift the byte to the right to align bits
        byte_to_write <<= 8 - byte_counter;
        // print!("{:08b}", byte_to_write);
        let _ = file.write_all(&[byte_to_write]);
    }
}

fn create_new_file_name(filename: &str) -> String {
    let base_filename = match filename.rfind('.') {
        Some(idx) => &filename[..idx],
        None => filename,
    };
    format!("{}.rhc", base_filename)
}
