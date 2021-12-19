// initialization
    // import dependancies
        use std::fs::{File};
        use std::io::prelude::*;
        use std::time::SystemTime;
        use rs_text_compression::text_compressor;

fn main() {

    // prepair word_to_index and index_to_word
        let index_pairs = text_compressor::generate_english_tables();

    // retrieve string to compress from file
        let mut file = File::open("./input.txt").expect("Failed to open file");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("Failed to read to string");

    println!("done pre-processing");
            
    // compess tokens into bytes
        let start_time = SystemTime::now();
        let compressed_bytes = text_compressor::compress(&contents, &index_pairs).expect("Can't compress non ASCII character.");
        let compression_duration = SystemTime::now()
            .duration_since(start_time)
            .expect("Time went backwards");

    // decompress compressed message
        let start_time = SystemTime::now();
        let decompressed = text_compressor::decompress(&compressed_bytes, &index_pairs[3].1).expect("Compressed data is malformed");
        let decompression_duration = SystemTime::now()
            .duration_since(start_time)
            .expect("Time went backwards");

    // print decompressed string     
        println!("{}",decompressed);

    // print compression statistics
        println!("=== Program Completion Stats ===");
        println!("Input file size:  {}",contents.len());
        println!("Output file size: {}",compressed_bytes.len());
        println!("Compressed file size now {:.1}% of original", (compressed_bytes.len() as f64 / contents.len() as f64) * 100.0);

        println!("Compression time: {:?}",compression_duration);
        println!("Decompression time: {:?}",decompression_duration);

    // write compressed data to file
        // use std::fs::OpenOptions;
        // let mut file = OpenOptions::new()
        //     .write(true)
        //     // either use ? or unwrap since it returns a Result
        //     .open("./compressed").unwrap();
        // file.write_all(&compressed_bytes).expect("failed to save compressed file");

    // let mut top_total:f64 = 0.0;
    // let split_num = 32;
    // for x in 1..=split_num{
    //     top_total += 1.0/x as f64;
    // }
    // println!("top_total: {}",top_total);

    // let mut bottom_total:f64 = 0.0;
    // for x in split_num..=100{
    //     bottom_total += 1.0/x as f64;
    // }
    // println!("bottom_total: {}",bottom_total);

    // println!("{} ",  top_total / (top_total + bottom_total));


}