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

    Ok(number_of_lines != 0)
}

fn main() {
    let arguments: Vec<String> = std::env::args().skip(1).collect();

    if arguments.len() < 2 {
        eprintln!("Usage : mygrep <pattern> <file-path>");
        process::exit(2);
    }

    let pattern: &String = &arguments[0];
    let path: &String = &arguments[1];

    match start_fetch(pattern, path) {
        Ok(true) => process::exit(0),
        Ok(false) => {
            process::exit(1);
        }
        Err(e) => {
            eprintln!("mygrep: {}: {}", path, e);
            process::exit(2);
        }
    }
}
