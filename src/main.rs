use rs_text_compression::text_compressor;
use std::fs::File;
use std::io::prelude::*;
use std::time::SystemTime;

fn main() {
    //> prepair word_to_index and index_to_word
        let index_pairs = text_compressor::generate_english_tables();

    //<> retrieve string to compress from file
        let mut file = File::open("./codebreaker/frankenstein.txt").expect("Failed to open file");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Failed to read to string");
    //<

    println!("done pre-processing");

    //> compess tokens into bytes
        let start_time = SystemTime::now();
        let compressed_bytes = text_compressor::compress(&contents, &index_pairs)
            .expect("Can't compress non ASCII character.");
        let compression_duration = SystemTime::now()
            .duration_since(start_time)
            .expect("Time went backwards");

    //<> decompress compressed message
        let start_time = SystemTime::now();
        let decompressed = text_compressor::decompress(&compressed_bytes, &index_pairs)
            .expect("Compressed data is malformed");
        let decompression_duration = SystemTime::now()
            .duration_since(start_time)
            .expect("Time went backwards");

    //<> print decompressed string
        println!("{}", decompressed);

    //<> print compression statistics
        println!("=== Program Completion Stats ===");
        println!("Input file size:  {}", contents.len());
        println!("Output file size: {}", compressed_bytes.len());
        println!(
            "Compressed file size now {:.1}% of original",
            (compressed_bytes.len() as f64 / contents.len() as f64) * 100.0
        );

        println!("Compression time: {:?}", compression_duration);
        println!("Decompression time: {:?}", decompression_duration);
    //<
}
