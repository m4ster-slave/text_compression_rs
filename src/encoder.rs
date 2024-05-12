use crate::*;

pub fn encode(file_name: &str, huffman_tree: HuffmanTree) -> String{
    // read in each character again 
    // look up code in tree 
    // append code to array
    let mut encoded_string: String = String::new(); 
    
    let contents = fs::read(file_name)
        .expect("Should have been able to read the file");

    for value in contents {
        encoded_string.push_str(&huffman_tree.get_code(value as char).unwrap());
    }

   encoded_string 
}
