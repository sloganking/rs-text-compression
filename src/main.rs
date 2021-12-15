// initialization
    // import dependancies
        use std::fs::File;
        use std::io::prelude::*;
        // use std::time::{SystemTime};
        use serde_json::json;

// function declarations
    fn encode_word(word: &str){

    }

    fn is_valid_capitalization(word: &str) -> bool{
        if word == word.to_uppercase(){
            return true
        }
        if word == word.to_lowercase(){
            return true
        }
        // println!("word: {}",word);
        // println!("len: {}",word.chars().count());
        if word.chars().count() < 2 {
            return true
        }
        // if first char is uppercase, and rest are lowercase
        if &word[0..1] == word[0..1].to_uppercase() && &word[1..] == &word[1..].to_lowercase(){
            return true
        }
        return false
    }


    fn decompress(compressed_bytes: Vec<u8>) -> String{

        return "Hi".to_string();
    }

fn main() {

    // prepair word_table
        // retrieve words from json
            let mut file = File::open("./english-words/words.txt").expect("Failed to open file");
            let mut contents = String::new();
            file.read_to_string(&mut contents).expect("Failed to read to string");

        // divide file by lines
            let lines: Vec<&str> = contents.split("\n").collect();

        // remove return character from strings
            let mut fixed_lines: Vec<String> = Vec::new();
            let carrage_return = 13 as char;
            for line in lines{
                fixed_lines.push(line.replace(carrage_return, ""));
            }

        // ensure lines are all lower case
            for line in &mut fixed_lines{
                line.make_ascii_lowercase();
            }

        // put words into word_table (for faster searching)
            let mut word_table: serde_json::Value = serde_json::from_str("{}").expect("JSON was not well-formatted");
            let word_table = word_table.as_object_mut().unwrap();
            for (i, line) in fixed_lines.iter().enumerate() {
                word_table.insert(line.to_string(), json!(i));
            }
            let word_table = word_table;

        println!("done pre-processing");

    // compress

        // retrieve string from file
            let mut file = File::open("./input.txt").expect("Failed to open file");
            let mut contents = String::new();
            file.read_to_string(&mut contents).expect("Failed to read to string");

        // crash if non ascii(< 127) character
            for char in contents.chars() {
                if char as u32 > 127 {
                    panic!("Can't compress non ASCII character.")
                }
            }

        // split strings to tokens (seperator is any character that's not alphanumeric, or '\'')
            let mut result = Vec::new();
            let mut last = 0;
            for (index, matched) in contents.match_indices(|c: char| !(c.is_alphanumeric() || c == '\'')) {
                if last != index {
                    result.push(&contents[last..index]);
                }
                result.push(matched);
                last = index + matched.len();
            }
            if last < contents.len() {
                result.push(&contents[last..]);
            }

            let tokens = result;

        // turn string into tokens
            // seperate by " "
                // let current_tokens: Vec<&str> = contents.split([' ', '\n']).collect();
            // re-add whitespace
                // let mut next_tokens: Vec<&str> = Vec::new();
                // for token in current_tokens{
                //     if token.len() > 0 {
                //         next_tokens.push(token);
                //     }
                //     next_tokens.push(" ");
                // }
                // next_tokens.pop();

                // let tokens = next_tokens;


        // test print tokens
            // for x in 0..30{
            //     println!("{}",tokens[x]);   
            // }
            
                
        // compess tokens into bytes
            let mut intermediate_compressed_bytes: Vec<Vec<u8>> = Vec::new();
            let mut compressed_bytes: Vec<u8> = Vec::new();
            for token in tokens {
                if token.len() > 2 && is_valid_capitalization(token) && word_table.contains_key(&token.to_lowercase()){
                    println!("compressing: {}",token);

                    // encode and store word
                        // bytes that store compressed word
                        // first bit is 1 to signify that this is a compressed word
                        let mut word_bytes: [u8; 3] = [0, 0, 0];
                        let word_index = word_table[&token.to_lowercase()].as_u64().unwrap();

                        println!("word_index: {}", word_index);

                        word_bytes[2] = word_index as u8;
                        word_bytes[1] = (word_index >> 8) as u8;
                        word_bytes[0] = (word_index >> 16) as u8 & 0b00000111;

                    // store case of word
                        if token == token.to_lowercase(){
                            // bits are 00
                        } else if token == token.to_uppercase(){
                            word_bytes[0] = word_bytes[0] | 0b00100000
                        } else if token[0..1] == token[0..1].to_uppercase() && token[1..] == token[1..].to_lowercase(){
                            word_bytes[0] = word_bytes[0] | 0b01000000
                        } else {
                            panic!("token \"{}\" slipped past is_valid_capitalization()", token);
                        }

                    // make first bit a 1
                    // signals the start of a compressed word
                        word_bytes[0] = word_bytes[0] | 0b10000000;

                    println!("{:?}",word_bytes);

                    for byte in word_bytes {
                        compressed_bytes.push(byte);
                    }
                    intermediate_compressed_bytes.push(vec![word_bytes[0].clone(),word_bytes[1].clone(),word_bytes[2].clone()]);
                } else {
                    // append token to file as plaintext
                    for byte in token.bytes(){
                        compressed_bytes.push(byte);
                        intermediate_compressed_bytes.push(vec![byte])
                    }
                }
            }

        // print compressed bytes

            println!("{:?}",intermediate_compressed_bytes)
            

}