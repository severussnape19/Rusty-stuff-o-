use std::env;
use std::fs;

fn analyze(contents: &str) {
    let line_count = contents.lines().count(); // iterator over the lines
    let word_count = contents.split_whitespace().count(); // iterator of words
    let char_count = contents.len();

    println!("
    content: {}
    line count: {}
    word count: {}
    char count: {}",
    contents, line_count, word_count, char_count);
}

fn main() {
    let args: Vec<String> = env::args().collect(); // iterator over the arguments
    
    if args.len() < 2 {
        println!("Usage: program <filename>");
        return;
    }

    let filename = &args[1];
    let opt: i8;
    opt = 2;
    match opt {
        1 => match fs::read_to_string(filename) {
            Ok(contents) => analyze(&contents),
            Err(e) => eprintln!("Error reading file: {e}")
        }

        2 => if let Ok(contents) = fs::read_to_string(filename) {
            analyze(&contents);
        } else {
            eprintln!("Error opening the file!");
        }

        _ => println!("Choose a valid option"),
    }
}

// .len() counts bytes
// .char() counts the unicode scalar values