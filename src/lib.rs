#[cfg(test)]
mod tests {
    use crate::text_compressor;
    use std::fs::File;
    use std::io::prelude::*;

    #[test]
    fn integration_test(){

        // retrieve string to compress from file
        let mut file = File::open("./codebreaker/frankenstein.txt").expect("Failed to open file");
        let mut text = String::new();
        file.read_to_string(&mut text).expect("Failed to read to string");

        // generate english tables
            let index_pairs = text_compressor::generate_english_tables();
        // compess tokens into bytes
            let compressed_bytes = text_compressor::compress(&text, &index_pairs).expect("Can't compress non ASCII character.");
        // decompress compressed message
            let decompressed = text_compressor::decompress(&compressed_bytes, &index_pairs).unwrap();
        // ensure compression/decompression was lossless
            assert_eq!(text,decompressed);
    }

    #[test]
    fn reject_non_ascii(){

        let text = "I 😀 am already far north of London, and as I walk in the streets of
        Petersburgh.";

        // generate english tables
        let index_pairs = text_compressor::generate_english_tables();
        // compess tokens into bytes
            let compressed_bytes = text_compressor::compress(&text, &index_pairs);
        // ensure compression/decompression was lossless
            assert_eq!(compressed_bytes,None);
    }

    #[test]
    fn compress_decompress_1(){
        let text = " of";
        
        // generate english tables
            let index_pairs = text_compressor::generate_english_tables();
        // compess tokens into bytes
            let compressed_bytes = text_compressor::compress(&text, &index_pairs).unwrap();
        // ensure correct compression
            assert_eq!(compressed_bytes, &[0b10100011]);
        // decompress compressed message
            let decompressed = text_compressor::decompress(&compressed_bytes, &index_pairs).unwrap();
        // ensure compression/decompression was lossless
            assert_eq!(decompressed, text);
    }

    #[test]
    fn compress_decompress_2(){
        let text = " Accommodation";
        
        // generate english tables
            let index_pairs = text_compressor::generate_english_tables();
        // compess tokens into bytes
            let compressed_bytes = text_compressor::compress(&text, &index_pairs).unwrap();
        // ensure correct compression
            // 2 bytes compressed word with capital first letter and space =
            // 0b11001

            // Accommodation = 
            // 1348
            // 101 01000100
            assert_eq!(compressed_bytes, vec![0b11001101, 0b01000100]);
        // decompress compressed message
            let decompressed = text_compressor::decompress(&compressed_bytes, &index_pairs).unwrap();
        // ensure compression/decompression was lossless
            assert_eq!(decompressed, text);
    }

    #[test]
    fn compress_decompress_3(){
        let text = " Frankenstein";
        
        // generate english tables
        let index_pairs = text_compressor::generate_english_tables();
        // compess tokens into bytes
            let compressed_bytes = text_compressor::compress(&text, &index_pairs).unwrap();
        // ensure compression/decompression was lossless

            // 3 bytes compressed word with capital first lette and space =
            // 0b011110

            // Frankenstein = 
            // 146326
            // 010 00111011 10010110

            assert_eq!(compressed_bytes, &[0b11101010, 0b00111011, 0b10010110]);

        // decompress compressed message
            let decompressed = text_compressor::decompress(&compressed_bytes, &index_pairs).unwrap();
        // ensure compression/decompression was lossless
            assert_eq!(decompressed, text);
    }

    #[test]
    fn catch_invalid_encoding_length(){
        // generate english tables
            let index_pairs = text_compressor::generate_english_tables();
            let decompressed = text_compressor::decompress(&[0b10000000], &index_pairs);
            assert_eq!(None,decompressed);
    }
}

pub mod text_compressor{
    use std::{collections::HashMap};

    // fn encode_word(word: &str){
    // }

    fn gen_index_tables_from(text: &str)  -> (HashMap<String, u32>, HashMap<u32, String>){
        // divide text by lines
            let lines: Vec<&str> = text.split('\n').collect();

        // remove return character from strings
            let mut fixed_lines: Vec<String> = Vec::new();
            let carrage_return = 13 as char;
            for line in lines{
                fixed_lines.push(line.replace(carrage_return, ""));
            }

        // put words into word_to_index (for faster searching)
            // create word_to_index table
            let mut word_to_index: HashMap<String, u32> = HashMap::new();
            // create index_to_word table
            let mut index_to_word: HashMap<u32, String> = HashMap::new();


            for (i, line) in fixed_lines.iter().enumerate() {
                word_to_index.insert(line.to_string(), i as u32);
                index_to_word.insert(i as u32, line.to_string());
            }

            (word_to_index, index_to_word)
    }

    pub fn generate_english_tables() -> Vec<(HashMap<String, u32>, HashMap<u32, String>)>{
        
        let mut index_pairs = vec![];

        // create index for 1 byte encoding
            // retrieve words from file
                let bytes = include_bytes!("../top_32.txt");

                let contents = match std::str::from_utf8(bytes) {
                    Ok(v) => v,
                    Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
                };

            // get the 0 index out of the way
            // this goes unused
            index_pairs.push(gen_index_tables_from(contents));

            // append indexes for 1 byte
            index_pairs.push(gen_index_tables_from(contents));

        // create index for 2 byte encoding
            // retrieve words from file
                let bytes = include_bytes!("../top_2048.txt");
                let contents = match std::str::from_utf8(bytes) {
                    Ok(v) => v,
                    Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
                };

            index_pairs.push(gen_index_tables_from(&contents.to_lowercase()));
        
        // create index for 3 byte encoding
            // retrieve words from file
                let bytes = include_bytes!("../english-words/words.txt");

                let contents = match std::str::from_utf8(bytes) {
                    Ok(v) => v,
                    Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
                };

            index_pairs.push(gen_index_tables_from(&contents.to_lowercase()));

        index_pairs
    }

    fn is_valid_capitalization(word: &str) -> bool{
        if word == word.to_lowercase(){
            return true
        }

        // if first char is uppercase, and rest are lowercase
        if word[0..1] == word[0..1].to_uppercase() && word[1..] == word[1..].to_lowercase(){
            return true
        }
        
        false
    }

    fn decompress_1(byte: u8, index_to_word: &HashMap<u32, String>) -> Option<String> {

        // get word
        let word_index: u32 = (byte & 0b00011111) as u32;

        if index_to_word.contains_key(&word_index){
            return Some(" ".to_string() + &index_to_word[&word_index])
        }
        None
    }

    fn decompress_2(bytes: &[u8], index_to_word: &HashMap<u32, String>) -> Option<String> {
        // get word index
            // convert bytes to u32 number
            let mut word_index: u32 = ((bytes[0] as u32) << 8) | bytes[1] as u32;
            // limit to 11 bits
            word_index &= 0b000000000000011111111111;
            
        // get word
            let word = &index_to_word[&word_index];

        // get previous char
        // (0 = ' ', 1 = '\n')
            let prev_char = match bytes[0] & 0b00010000{
                0 => ' ',
                _ => '\n',
            };

        // get first char case
        // (0 = lower, 1 = upper)
            match bytes[0] & 0b00001000{
                0 => Some(prev_char.to_string() + &word.to_lowercase()),
                _ => Some(prev_char.to_string() + &word[0..1].to_uppercase() + &word[1..].to_lowercase()), 
            }
    }

    fn decompress_3(bytes: &[u8], index_to_word: &HashMap<u32, String>) -> Option<String> {
        // get word index
            // convert bytes to u32 number
            let mut word_index: u32 = ((bytes[0] as u32) << 16) | ((bytes[1] as u32) << 8) | bytes[2] as u32;
            // limit to 19 bits
            word_index &= 0b000001111111111111111111;
        // get word
            let word = &index_to_word[&word_index];

        // get previous char
        // (0 = ' ', 1 = '\n')
            let prev_char = match bytes[0] & 0b00010000{
                0 => ' ',
                _ => '\n',
            };

        // get first char case
        // (0 = lower, 1 = upper)
            match bytes[0] & 0b00001000{
                0 => Some(prev_char.to_string() + &word.to_lowercase()),
                _ => Some(prev_char.to_string() + &word[0..1].to_uppercase() + &word[1..].to_lowercase()), 
            }
    }

    pub fn decompress(compressed_bytes: &[u8], index_pairs: &[(HashMap<String, u32>, HashMap<u32, String>)]) -> Option<String>{

        let mut decompressed_text: String = String::new();
        let mut i = 0;
        while i < compressed_bytes.len(){
            // if next three bytes are a compressed word
            if compressed_bytes[i] & 128 != 0{

                // get encoding length
                    let encoding_length = (compressed_bytes[i] & 0b01100000) >> 5;

                // decode encoded word
                    // println!("encoding_length: {} i: {}",encoding_length, i);
                    // println!("{}",decompressed_text);
                    let decoded = match encoding_length{
                        1 => {
                            let decompressed = decompress_1(compressed_bytes[i], &index_pairs[1].1);
                            i+=1;
                            decompressed
                        },
                        2 => {
                            let decompressed = decompress_2(&compressed_bytes[i..i+2], &index_pairs[2].1);
                            i+=2;
                            decompressed
                        }
                        3 => {
                            let decompressed = decompress_3(&compressed_bytes[i..i+3], &index_pairs[3].1);
                            i+=3;
                            decompressed
                        },
                        _ => None,
                    };

                // store decompressed text
                    match decoded{
                        Some(x) => decompressed_text += &x,
                        _ => return None,
                    }
            }else{
                decompressed_text.push(compressed_bytes[i] as char);
                i += 1;
            }
        }
        Some(decompressed_text)
    }

    fn compress_beginning(token: &str, word_to_index: &HashMap<String, u32>, compressed_bytes: &mut Vec<u8>, last_was_plaintext: bool) -> Option<Vec<u8>>{

        let mut max_len = 4;
        if token.len() < 4{
            max_len = token.len();
        }

        for i in (1..=max_len).rev(){
            match compress_1(&token[0..i], word_to_index, compressed_bytes, last_was_plaintext){
                Some(mut x) => {
                    let new_vec = token[i..].as_bytes();
                    x.extend(new_vec);
                    return Some(x)
                },
                _ => continue,
            }
        }   
        None
    }

    fn compress_1(token: &str, word_to_index: &HashMap<String, u32>, compressed_bytes: &mut Vec<u8>, last_was_plaintext: bool) -> Option<Vec<u8>>{
        if last_was_plaintext && compressed_bytes[compressed_bytes.len() - 1] as char == ' ' && word_to_index.contains_key(&token.to_string()){
            // remove previously encoded space
            // it will be encoded in the compressed word
                compressed_bytes.pop();

            let mut compressed_byte = (word_to_index[token] as u8) & 0b00011111;
            compressed_byte |= 0b10100000;

            Some(vec![compressed_byte])
        }else{
            None
        }
    }

    fn compress_2(token: &str, word_to_index: &HashMap<String, u32>, compressed_bytes: &mut Vec<u8>, last_was_plaintext: bool) -> Option<Vec<u8>>{

        if !last_was_plaintext {
            return None
        }

        let last_char = compressed_bytes[compressed_bytes.len() - 1] as char;

        if (last_char == ' ' || last_char == '\n') && token.len() >= 2 && is_valid_capitalization(token) && word_to_index.contains_key(&token.to_lowercase()){

            // bytes that store compressed word
            let mut word_bytes = vec![0, 0];

            // encode and store word
                let word_index = word_to_index[&token.to_lowercase()];

                word_bytes[1] = word_index as u8;
                word_bytes[0] = (word_index >> 8) as u8 & 0b00000111;

            // store previous char
            // (0 = ' ', 1 = '\n')
                if last_char == '\n'{
                    word_bytes[0] |= 0b00010000;
                }
                // erase previous char from encoding
                compressed_bytes.pop();

            // store first char case
            // (0 = lower, 1 = upper)
                if token[0..1] == token[0..1].to_uppercase(){
                    word_bytes[0] |= 0b00001000;
                }

            // signals the start of a 2 byte compressed word
            word_bytes[0] |= 0b11000000;

            Some(word_bytes)
        }else{
            None
        }
    }

    fn compress_3(token: &str, word_to_index: &HashMap<String, u32>, compressed_bytes: &mut Vec<u8>, last_was_plaintext: bool) -> Option<Vec<u8>>{

        if !last_was_plaintext {
            return None
        }

        let last_char = compressed_bytes[compressed_bytes.len() - 1] as char;

        if (last_char == ' ' || last_char == '\n') && token.len() >= 3 && is_valid_capitalization(token) && word_to_index.contains_key(&token.to_lowercase()){

            // bytes that store compressed word
            let mut word_bytes = vec![0, 0, 0];

            // encode and store word
                let word_index = word_to_index[&token.to_lowercase()];

                word_bytes[2] = word_index as u8;
                word_bytes[1] = (word_index >> 8) as u8;
                word_bytes[0] = (word_index >> 16) as u8 & 0b00000111;

            // store previous char
            // (0 = ' ', 1 = '\n')
                if last_char == '\n'{
                    word_bytes[0] |= 0b00010000;
                }
                // erase previous char from encoding
                compressed_bytes.pop();

            // store first char case
            // (0 = lower, 1 = upper)
                if token[0..1] == token[0..1].to_uppercase(){
                    word_bytes[0] |= 0b00001000;
                }

            // signals the start of a 3 byte compressed word
                word_bytes[0] |= 0b11100000;

                Some(word_bytes)
        }else{
            None
        }
    }

    pub fn compress(text: &str, index_pairs: &[(HashMap<String, u32>, HashMap<u32, String>)]) -> Option<Vec<u8>>{
        // crash if non ascii(< 127) character
            for char in text.chars() {
                if char as u32 > 127 {
                    return None
                }
            }

        // split strings to tokens (seperator is any character that's not alphanumeric, or '\'')
            let mut result = Vec::new();
            let mut last = 0;
            for (index, matched) in text.match_indices(|c: char| !(c.is_alphanumeric() || c == '\'')) {
                if last != index {
                    result.push(&text[last..index]);
                }
                result.push(matched);
                last = index + matched.len();
            }
            if last < text.len() {
                result.push(&text[last..]);
            }

            let tokens = result;

        // compress
            let mut compressed_bytes: Vec<u8> = Vec::new();
            let mut last_was_plaintext = false;
            for token in tokens {

                // compress token if possible
                    let mut word_bytes = compress_1(token, &index_pairs[1].0, &mut compressed_bytes, last_was_plaintext);
                    if word_bytes == None {
                        word_bytes = compress_2(token, &index_pairs[2].0, &mut compressed_bytes, last_was_plaintext);
                    }
                    if word_bytes == None {
                        word_bytes = compress_3(token, &index_pairs[3].0, &mut compressed_bytes, last_was_plaintext);
                    }
                    if word_bytes == None {
                        word_bytes = compress_beginning(token, &index_pairs[1].0, &mut compressed_bytes, last_was_plaintext);
                    }               

                match word_bytes {
                    Some(word_bytes) => {
                        // append compressed token to compressed_bytes
                        for byte in word_bytes {
                            compressed_bytes.push(byte);
                        }
                        last_was_plaintext = false;
                    }
                    None => {
                        // append token to file as plaintext
                        for byte in token.bytes(){
                            compressed_bytes.push(byte);
                        }
                        last_was_plaintext = true;
                    }
                }
            }
        Some(compressed_bytes)
    }
}