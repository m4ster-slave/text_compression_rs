# text_compression_rs
Simple text compression using Huffman tree encoding written in rust

## Compress/Decompress: 
- `$ cargo run -c <text file>` this is gonna produce the same file but with the `.rhc` file ending.
- `$ cargo run -x <text file>` this is gonna decompress the file resulting in the original text file.

> its important to note that the algorithm is gonna make the file bigger if it is really small (<200bytes) because the huffman tree needs to be encoded into the compressed file resulting in more data being in the file than before.
