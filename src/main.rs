use std::collections::HashMap;
// initialization
    // import dependancies
        use std::fs::File;
        use std::io::prelude::*;
        // use std::time::{SystemTime};
        use serde_json::json;

// function declarations
    // fn encode_word(word: &str){
    // }

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
        if word[0..1] == word[0..1].to_uppercase() && word[1..] == word[1..].to_lowercase(){
            return true
        }
        false
    }


    fn decompress(compressed_bytes: Vec<u8>, index_to_word: HashMap<u32, String>) -> String{

        let mut decompressed_text: String = String::new();
        let mut i = 0;
        while i < compressed_bytes.len(){
            // if next three bytes are a compressed word
            if compressed_bytes[i] & 128 != 0{
                // get spaces
                    let left_space = compressed_bytes[i] & 16 != 0;
                    let right_space = compressed_bytes[i] & 8 != 0;

                // get capitalization
                    // 00 = allLower
                    // 01 = allUpper
                    // 10 = firstCharUpper restLower
                    let capstate = (compressed_bytes[i] >> 5) & 3;
                // get word index
                    // convert bytes to u32 number
                    let mut word_index: u32 = ((compressed_bytes[i] as u32) << 16) | ((compressed_bytes[i + 1] as u32) << 8) | compressed_bytes[i + 2] as u32;
                    // limit to 19 bits
                    word_index = word_index & 0b000001111111111111111111;

                // build text
                    let mut word = index_to_word[&word_index].clone();
                    // capitalize word
                        if capstate == 0{
                            // is this necessary?
                            word = word.to_lowercase();
                        }else if capstate == 1{
                            word = word.to_uppercase();  
                        }else if capstate == 2{
                            // make first char uppercase
                            let ch = word.chars().nth(0).unwrap();
                            let mut first_char = String::new();
                            first_char.push(ch);

                            word = first_char + &word[1..].to_lowercase();
                        }else{
                            panic!("invalid capitalization state")
                        }
                    // append any spacing
                        let mut text = word;
                        if left_space {
                            text = String::from(" ") + &text;
                        }
                        if right_space{
                            text =  text.to_string() + " ";
                        }
                    // store decompressed text
                        decompressed_text = decompressed_text + &text;
                    
                    // update current byte
                        i += 3;
            }else{
                decompressed_text.push(compressed_bytes[i] as char);
                i += 1;
            }
        }
        decompressed_text
    }

fn main() {

    // prepair word_to_index
        // retrieve words from json
            let mut file = File::open("./english-words/words.txt").expect("Failed to open file");
            let mut contents = String::new();
            file.read_to_string(&mut contents).expect("Failed to read to string");

        // divide file by lines
            let lines: Vec<&str> = contents.split('\n').collect();

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

        // put words into word_to_index (for faster searching)
            // create word_to_index table
                let mut word_to_index: serde_json::Value = serde_json::from_str("{}").expect("JSON was not well-formatted");
                let word_to_index = word_to_index.as_object_mut().unwrap();
            // create index_to_word table
                let mut index_to_word: HashMap<u32, String> = HashMap::new();


            for (i, line) in fixed_lines.iter().enumerate() {
                word_to_index.insert(line.to_string(), json!(i));
                index_to_word.insert(i as u32, line.to_string());
            }

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
                if token.len() > 2 && is_valid_capitalization(token) && word_to_index.contains_key(&token.to_lowercase()){

                    // encode and store word
                        // bytes that store compressed word
                        // first bit is 1 to signify that this is a compressed word
                        let mut word_bytes: [u8; 3] = [0, 0, 0];
                        let word_index = word_to_index[&token.to_lowercase()].as_u64().unwrap();

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

                    for byte in word_bytes {
                        compressed_bytes.push(byte);
                    }
                    intermediate_compressed_bytes.push(vec![word_bytes[0],word_bytes[1],word_bytes[2]]);
                } else {
                    // append token to file as plaintext
                    for byte in token.bytes(){
                        compressed_bytes.push(byte);
                        intermediate_compressed_bytes.push(vec![byte])
                    }
                }
            }

        // print compressed bytes

            // println!("{:?}",intermediate_compressed_bytes);

            let decompressed = decompress(compressed_bytes, index_to_word);
            println!("{}",decompressed);
            

}