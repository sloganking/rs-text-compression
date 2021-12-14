// initialization
    // import dependancies
        use std::fs::File;
        use std::io::prelude::*;
        // use std::time::{SystemTime};
        use serde_json::json;

// function declarations

fn main() {

    // prepair word table
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

        // put words into table (for faster searching)
            let mut table: serde_json::Value = serde_json::from_str("{}").expect("JSON was not well-formatted");
            let table = table.as_object_mut().unwrap();
            for (i, line) in fixed_lines.iter().enumerate() {
                table.insert(line.to_string(), json!(i));
            }

        // test if key in table
            let key = "squid";
            println!("key table: {}", table[key]);

        println!("done pre-processing");
        




    // compress

        // retrieve string from file
            let mut file = File::open("./input.txt").expect("Failed to open file");
            let mut contents = String::new();
            file.read_to_string(&mut contents).expect("Failed to read to string");

        let to_compress = contents;

        // process input text
            // turn string into tokens

            let to_compress = to_compress.split(" ");

            println!("{:?}",to_compress)

            


}
