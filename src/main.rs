use std::collections::HashMap;
use std::env;
use std::io::{Read, Write};
use std::fs::{File, OpenOptions};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        println!("Usage: <file-path> <word-to-replace> <new-word>");
        return;
    }

    let input_file = &args[1];
    let old_word = &args[2];
    let new_word = &args[3];

    let mut content = String::new();
    let mut file = File::open(input_file).expect("Invalid File Path");
    file.read_to_string(&mut content).expect("Failed Reading File");

    println!("{}", content);

    let mut replacement = HashMap::new();
    replacement.insert(old_word.to_string(), new_word.to_string());

    // println!("{:?}", replacement);

    let mut modified_text = String::new();
    for line in content.lines() {
        let words = line.split_whitespace();
        for word in words {
            if let Some(replacement_word) = replacement.get(word) {
                modified_text.push_str(replacement_word);
            } else {
                modified_text.push_str(word);
            }
            modified_text.push(' ');
        }
        modified_text.push('\n'); 
    }

    let mut file_name_split = input_file.split('.');
    let new_file_name = &file_name_split.next().unwrap();

    let mut output_file = OpenOptions::new()
        .create(true)
        .write(true)
        .open(format!("{}_new.txt", new_file_name))
        .expect("Failed to create file");
    let _ = output_file.write(modified_text.as_bytes());

    println!("Modified the {}, and saved the contents to {}", input_file, format!("{}_new.txt", new_file_name));
}