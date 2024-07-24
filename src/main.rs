use pdf_extract;
use regex::Regex;
use std::collections::HashMap;
use std::env;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 5 {
        println!(
            "Usage: <flag> <file-path> <word-to-replace> <new-word>\n\t1-->Text files\n\t2-->PDFs"
        );
        return;
    }

    let flag = args[1].as_str();
    let input_file = &args[2];
    let old_word = &args[3];
    let new_word = &args[4];

    match flag {
        "1" => read_text_file(input_file, old_word, new_word),
        "2" => read_pdf_file(input_file, old_word, new_word),
        _ => println!("INVALID FLAG, use 1 for TEXT files and 2 for PDFs"),
    }
}

fn read_text_file(input_file: &String, old_word: &String, new_word: &String) {
    let mut content = String::new();
    let mut file = File::open(input_file).expect("Invalid File Path");
    file.read_to_string(&mut content)
        .expect("Failed Reading File");

    let modified_text = replace_text(&content, old_word, new_word);

    let mut file_name_split = input_file.split('.');
    let new_file_name = file_name_split.next().unwrap();

    let mut output_file = OpenOptions::new()
        .create(true)
        .write(true)
        .open(format!("{}_new.txt", new_file_name))
        .expect("Failed to create file");
    let _ = output_file.write(modified_text.as_bytes());

    println!(
        "\nModified the {}, and saved the contents to {}",
        input_file,
        format!("{}_new.txt", new_file_name)
    );
}

fn read_pdf_file(input_file: &str, old_word: &String, new_word: &String) {
    match pdf_extract::extract_text(input_file) {
        Ok(content) => {
            let cleaned_content = clean_text(&content);
            // println!("{}", cleaned_content);

            let modified_text = replace_text(&cleaned_content, old_word, new_word);

            let mut file_name_split = input_file.split('.');
            let new_file_name = file_name_split.next().unwrap();
            // println!("{:?},{}",file_name_split,new_file_name);

            let mut output_file = OpenOptions::new()
                .create(true)
                .write(true)
                .open(format!("{}_new.txt", new_file_name))
                .expect("Failed to create file");
            let _ = output_file.write(modified_text.as_bytes());

            println!(
                "\nModified the {}, and saved the contents to {}",
                input_file,
                format!("{}_new.txt", new_file_name)
            );
        }
        Err(error) => println!("Error extracting text from PDF: {:?}", error),
    }
}

fn clean_text(content: &str) -> String {
    let re = Regex::new(r"[.,!'?]").unwrap();
    let clean_text = re.replace_all(content, "").to_lowercase();
    clean_text.to_string()
}

fn replace_text(content: &String, old_word: &String, new_word: &String) -> String {
    let mut replacement = HashMap::new();
    replacement.insert(old_word.to_string(), new_word.to_string());

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
    modified_text
}