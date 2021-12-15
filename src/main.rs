// initialization
    // import dependancies
        use std::fs::File;
        use std::io::prelude::*;
        use rs_text_compression::text_compressor;

fn main() {

    // prepair word_to_index and index_to_word
    let (word_to_index, index_to_word) = text_compressor::generate_english_tables();

    // retrieve string to compress from file
        let mut file = File::open("./input.txt").expect("Failed to open file");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("Failed to read to string");

    println!("done pre-processing");
            
    // compess tokens into bytes
        let compressed_bytes = text_compressor::compress(&contents, word_to_index).expect("Can't compress non ASCII character.");

    // decompress compressed message
        let decompressed = text_compressor::decompress(&compressed_bytes, index_to_word).expect("Compressed data is malformed");

    // print decompressed string     
        println!("{}",decompressed);

    // print compression statistics
        println!("=== Program Completion Stats ===");
        println!("Input file size:  {}",contents.len());
        println!("Output file size: {}",compressed_bytes.len());
        println!("Compressed file size now {:.1}% of original", (compressed_bytes.len() as f64 / contents.len() as f64) * 100.0);
}