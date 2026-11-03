fn start_fetch(pattern: &str, path: &str) -> Result<bool, std::io::Error> {
    let contents = std::fs::read_to_string(path)?;

    let mut number_of_lines: u32 = 0;

    for i in contents.lines() {
        if i.contains(pattern) {
            println!("{}", i);
            number_of_lines += 1;
        }
    }

    println!("{}", number_of_lines);
    Ok(true)
}

fn main() {
    let arguments: Vec<String> = std::env::args().skip(1).collect();

    if arguments.len() < 2 {
        eprintln!("Usage : mygrep <pattern> <file-path>");
        std::process::exit(2);
    }

    let pattern: &String = &arguments[0];
    let path: &String = &arguments[1];

    if let Err(e) = start_fetch(pattern, path) {
        eprintln!("{}", e);
        std::process::exit(2);
    }
}
