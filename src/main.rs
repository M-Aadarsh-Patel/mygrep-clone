use mygrep::{Config, start_search};
use std::process;

fn main() {
    let arguments: Vec<String> = std::env::args().skip(1).collect();

    let config = match Config::new(arguments) {
        Ok(value) => value,
        Err(e) => {
            eprintln!("{}", e);
            process::exit(2);
        }
    };

    let pattern = &config.pattern;
    let path = &config.path;

    match start_search(pattern, path) {
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
