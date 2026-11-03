use std::{io, process};

fn start_fetch(pattern: &str, path: &str) -> Result<bool, io::Error> {
    let contents = std::fs::read_to_string(path)?;

    let mut number_of_lines: u32 = 0;

    for i in contents.lines() {
        if i.contains(pattern) {
            println!("{}", i);
            number_of_lines += 1;
        }
    }

    if number_of_lines == 0 {
        return Err(io::Error::other("no match found"));
    }
    Ok(true)
}

fn main() {
    let arguments: Vec<String> = std::env::args().skip(1).collect();

    if arguments.len() < 2 {
        eprintln!("Usage : mygrep <pattern> <file-path>");
        process::exit(2);
    }

    let pattern: &String = &arguments[0];
    let path: &String = &arguments[1];

    if let Err(e) = start_fetch(pattern, path) {
        if e.to_string().contains("no match found") {
            process::exit(1);
        }
    } else {
        process::exit(0);
    }
}
