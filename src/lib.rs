#[cfg(test)]
mod tests {
    use crate::text_compressor;

    #[test]
    fn integration_test(){

        let text = "I am already far north of London, and as I walk in the streets of
        Petersburgh, I feel a cold northern breeze play upon my cheeks, which
        braces my nerves and fills me with delight.  Do you understand this
        feeling?  This breeze, which has travelled from the regions towards
        which I am advancing, gives me a foretaste of those icy climes.
        Inspirited by this wind of promise, my daydreams become more fervent
        and vivid.  I try in vain to be persuaded that the pole is the seat of
        frost and desolation; it ever presents itself to my imagination as the
        region of beauty and delight.  There, Margaret, the sun is forever
        visible, its broad disk just skirting the horizon and diffusing a
        perpetual splendour.  There--for with your leave, my sister, I will put
        some trust in preceding navigators--there snow and frost are banished;
        and, sailing over a calm sea, we may be wafted to a land surpassing in
        wonders and in beauty every region hitherto discovered on the habitable
        globe.  Its productions and features may be without example, as the
        phenomena of the heavenly bodies undoubtedly are in those undiscovered
        solitudes.  What may not be expected in a country of eternal light?  I
        may there discover the wondrous power which attracts the needle and may
        regulate a thousand celestial observations that require only this
        voyage to render their seeming eccentricities consistent forever.  I
        shall satiate my ardent curiosity with the sight of a part of the world
        never before visited, and may tread a land never before imprinted by
        the foot of man. These are my enticements, and they are sufficient to
        conquer all fear of danger or death and to induce me to commence this
        laborious voyage with the joy a child feels when he embarks in a little
        boat, with his holiday mates, on an expedition of discovery up his
        native river. But supposing all these conjectures to be false, you
        cannot contest the inestimable benefit which I shall confer on all
        mankind, to the last generation, by discovering a passage near the pole
        to those countries, to reach which at present so many months are
        requisite; or by ascertaining the secret of the magnet, which, if at
        all possible, can only be effected by an undertaking such as mine.
        
        These reflections have dispelled the agitation with which I began my
        letter, and I feel my heart glow with an enthusiasm which elevates me
        to heaven, for nothing contributes so much to tranquillize the mind as
        a steady purpose--a point on which the soul may fix its intellectual
        eye.  This expedition has been the favourite dream of my early years. I
        have read with ardour the accounts of the various voyages which have
        been made in the prospect of arriving at the North Pacific Ocean
        through the seas which surround the pole.  You may remember that a
        history of all the voyages made for purposes of discovery composed the
        whole of our good Uncle Thomas' library.  My education was neglected,
        yet I was passionately fond of reading.  These volumes were my study
        day and night, and my familiarity with them increased that regret which
        I had felt, as a child, on learning that my father's dying injunction
        had forbidden my uncle to allow me to embark in a seafaring life.
        
        These visions faded when I perused, for the first time, those poets
        whose effusions entranced my soul and lifted it to heaven.  I also
        became a poet and for one year lived in a paradise of my own creation;
        I imagined that I also might obtain a niche in the temple where the
        names of Homer and Shakespeare are consecrated.  You are well
        acquainted with my failure and how heavily I bore the disappointment.
        But just at that time I inherited the fortune of my cousin, and my
        thoughts were turned into the channel of their earlier bent.";

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

    // #[test]
    // fn catch_malformed_compressed(){
    //     // generate english tables
    //         let index_pairs = text_compressor::generate_english_tables();
    //     // decompress compressed message
    //         let decompressed = text_compressor::decompress(&[228, 207, 101], &index_pairs[3].1);
    //     assert_eq!(decompressed,None);
    // }

    #[test]
    fn compress_3_byte(){
        let text = " Frankenstein";
        

        // generate english tables
        let index_pairs = text_compressor::generate_english_tables();
        // compess tokens into bytes
            let compressed_bytes = text_compressor::compress(&text, &index_pairs);
        // ensure compression/decompression was lossless

            // 3 bytes compressed word with capital first lette and space =
            // 0b011110

            // Frankenstein = 
            // 146326
            // 010 00111011 10010110

            let bytes: Option<Vec<u8>> = Some(vec![0b11110010, 0b00111011, 0b10010110]);
            assert_eq!(compressed_bytes, bytes);
    }

    #[test]
    fn compress_1_byte(){
        let text = " of";
        

        // generate english tables
            let index_pairs = text_compressor::generate_english_tables();
        // compess tokens into bytes
            let compressed_bytes = text_compressor::compress(&text, &index_pairs);
        // ensure compression/decompression was lossless
            let bytes: Option<Vec<u8>> = Some(vec![0b10100011]);
            assert_eq!(compressed_bytes, bytes);

        // decompress compressed message
            let decompressed = text_compressor::decompress(&bytes.unwrap(), &index_pairs).unwrap();
        // ensure compression/decompression was lossless
            assert_eq!(text,decompressed);
    }
}



// Frankenstein

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

        // ensure lines are all lower case
            for line in &mut fixed_lines{
                line.make_ascii_lowercase();
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
        // currently a dupe of 1
            index_pairs.push(gen_index_tables_from(contents));
        
        // create index for 3 byte encoding
            // retrieve words from file
                let bytes = include_bytes!("../english-words/words.txt");

                let contents = match std::str::from_utf8(bytes) {
                    Ok(v) => v,
                    Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
                };

            index_pairs.push(gen_index_tables_from(contents));

        index_pairs
    }



    fn is_valid_capitalization(word: &str) -> bool{
        if word == word.to_lowercase(){
            return true
        }

        // should this be here?
            // if word.chars().count() < 2 {
            //     return true
            // }

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

    fn decompress_3(bytes: &[u8], index_to_word: &HashMap<u32, String>) -> Option<String> {
        // get word index
            // convert bytes to u32 number
            let mut word_index: u32 = ((bytes[0] as u32) << 16) | ((bytes[1] as u32) << 8) | bytes[2] as u32;
            // limit to 19 bits
            word_index &= 0b000001111111111111111111;
        // get word
            let word = &index_to_word[&word_index];

        // get case/prevchar of word
            let case_prevchar = (bytes[0] & 0b00011000) >> 3;

        // apply case/prevchar to word
            let decompressed = match case_prevchar{
                0 => Some(' '.to_string() + &word.to_lowercase()),
                1 => Some('\n'.to_string() + &word.to_lowercase()),
                2 => Some(' '.to_string() + &word[0..1].to_uppercase() + &word[1..].to_lowercase()),
                3 => Some('\n'.to_string() + &word[0..1].to_uppercase() + &word[1..].to_lowercase()),
                _ => None,
            };

        decompressed
    }

    pub fn decompress(compressed_bytes: &[u8], index_pairs: &Vec<(HashMap<String, u32>, HashMap<u32, String>)>) -> Option<String>{

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
                        3 => {
                            let decompressed = decompress_3(&compressed_bytes[i..i+3], &index_pairs[3].1);
                            i+=3;
                            decompressed
                        },
                        _ => None,
                    };

                // store decompressed text
                    match decoded{
                        Some(x) => decompressed_text = decompressed_text + &x,
                        _ => return None,
                    }



                // // get spaces
                //     let left_space = compressed_bytes[i] & 16 != 0;
                //     let right_space = compressed_bytes[i] & 8 != 0;

                // // get capitalization
                //     // 00 = allLower
                //     // 01 = allUpper
                //     // 10 = firstCharUpper restLower
                //     let capstate = (compressed_bytes[i] >> 5) & 3;
                // // get word index
                //     // convert bytes to u32 number
                //     let mut word_index: u32 = ((compressed_bytes[i] as u32) << 16) | ((compressed_bytes[i + 1] as u32) << 8) | compressed_bytes[i + 2] as u32;
                //     // limit to 19 bits
                //     word_index &= 0b000001111111111111111111;

                // // build text
                //     let mut word = index_pairs[3].1[&word_index].clone();
                //     // capitalize word
                //         if capstate == 0{
                //             // is this necessary?
                //             word = word.to_lowercase();
                //         }else if capstate == 1{
                //             word = word.to_uppercase();  
                //         }else if capstate == 2{
                //             // make first char uppercase
                //             let ch = word.chars().next().unwrap();
                //             let mut first_char = String::new();
                //             first_char.push(ch);

                //             word = first_char.to_uppercase() + &word[1..].to_lowercase();
                //         }else{
                //             return None
                //         }
                //     // append any spacing
                //         let mut text = word;
                //         if left_space {
                //             text = String::from(" ") + &text;
                //         }
                //         if right_space{
                //             text =  text.to_string() + " ";
                //         }
                //     // store decompressed text
                //         decompressed_text = decompressed_text + &text;
                    
                //     // update current byte
                //         i += 3;
            }else{
                decompressed_text.push(compressed_bytes[i] as char);
                i += 1;
            }
        }
        Some(decompressed_text)
    }

    fn compress_1(token: &str, word_to_index: &HashMap<String, u32>, compressed_bytes: &mut Vec<u8>, last_was_plaintext: bool) -> Option<Vec<u8>>{

        let mut compressed_byte: u8 = 0;

        if last_was_plaintext && compressed_bytes[compressed_bytes.len() - 1] as char == ' ' && word_to_index.contains_key(&token.to_string()){
            // remove previously encoded space
            // it will be encoded in the compressed word
                compressed_bytes.pop();

            compressed_byte = (word_to_index[token] as u8) & 0b00011111;
            compressed_byte |= 0b10100000;

            Some(vec![compressed_byte])
        }else{
            None
        }
    }

    fn compress_2(token: &str, word_to_index: &HashMap<String, u32>, compressed_bytes: &mut Vec<u8>, last_was_plaintext: bool) -> Option<Vec<u8>>{

        if last_was_plaintext == false {
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

            // store first char case
            // (0 = lower, 1 = upper)
                if token[0..1] == token[0..1].to_uppercase(){
                    word_bytes[0] |= 0b00001000;
                }

            // signals the start of a 2 byte compressed word
            word_bytes[0] |= 0b11000000;

            return Some(word_bytes);

        }else{
            None
        }
    }

    fn compress_3(token: &str, word_to_index: &HashMap<String, u32>, compressed_bytes: &mut Vec<u8>, last_was_plaintext: bool) -> Option<Vec<u8>>{

        if last_was_plaintext == false {
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

            // store case/prevchar of word

                let space_in_front = last_char == ' ';
                let return_in_front = last_char == '\n';
                let undercase = token == token.to_lowercase();
                let first_upper = token[0..1] == token[0..1].to_uppercase() && token[1..] == token[1..].to_lowercase();

                let case: u8 = if undercase && space_in_front{
                    0b00
                } else if undercase && return_in_front{
                    0b01
                } else if first_upper && space_in_front{
                    0b10
                } else if first_upper && return_in_front{
                    0b11
                }else{
                    panic!("invalid case/prevchar");
                };

                compressed_bytes.pop();

                word_bytes[0] |= case << 3;

            // signals the start of a 3 byte compressed word
                word_bytes[0] |= 0b11100000;

                return Some(word_bytes);
        }else{
            None
        }
    }

    pub fn compress(text: &str, index_pairs: &Vec<(HashMap<String, u32>, HashMap<u32, String>)>) -> Option<Vec<u8>>{
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
                        word_bytes = compress_3(token, &index_pairs[3].0, &mut compressed_bytes, last_was_plaintext);
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