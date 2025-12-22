use std::fs::{self, File};
use std::io::prelude::*;
use std::io::{BufRead, BufReader};

fn main() {
    let mut file = match File::create("output.txt") {
        Ok(v) => v,
        Err(_) => { 
            println!("Error occured while creating a file!"); 
            return
        }
    };

    match file.write_all(b"Hello from rust!!") {
        Ok(v) => v,
        Err(_) => {
            println!("Error while writing to file!");
            return
        }
    }

    let contents = fs::read_to_string("output.txt")
        .expect("Error");
    let word_count = contents.split_whitespace().count();
    let line_count = contents.lines().count();
    let char_count = contents.chars().count();
    
    // ^^^^^^^
    let mut doc_vec: Vec<String> = Vec::new();
    let doc = File::open("output.txt").unwrap_or_else(|err| { // closure (lambda)
        println!("Log not found ({}), creating a new one.", err);
        File::create("history.log").expect("Could not create a file!")
    }) ;

    let reader = BufReader::new(doc);

    for line in reader.lines() {
        let line_string = line.expect("Error reading this line");
        doc_vec.push(line_string);
    }

    println!("Contents: {contents}");
    println!("Chars: {}\nWords: {}\nLine: {}", char_count, word_count, line_count);
}