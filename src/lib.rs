use std::io;
pub struct Config {
    pub path: String,
    pub pattern: String,
}

impl Config {
    pub fn new(arguments: Vec<String>) -> Result<Config, String> {
        if arguments.len() == 2 {
            Ok(Config {
                path: arguments[1],
                pattern: arguments[0],
            })
        } else {
            Err(String::from("Usage: mygrep <pattern> <path>"))
        }
    }
}

pub fn start_search(pattern: &str, path: &str) -> Result<bool, io::Error> {
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
