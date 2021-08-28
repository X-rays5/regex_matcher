use regex::Regex;
use std::io::{Write, BufRead};

fn read_line(text: &str) -> String {
    print!("{}", text);
    std::io::stdout().flush()
        .expect("Failed to flush stdout");

    let mut line = String::new();
    std::io::stdin().read_line(&mut line)
        .expect("Failed to read line");
    let input = line.trim();

    input.parse().unwrap()
}

fn read_file(filename: String) -> Vec<String> {
    let file = std::fs::File::open(filename).expect("no such file");
    let buf = std::io::BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn do_line(pattern: Regex, line: &String) {
    if pattern.is_match(&*line) {
        println!("\nHas match: {}", pattern.is_match(&*line));
        for captures in pattern.captures_iter(&*line) {
            for capture in captures.iter() {
                println!("{}", capture.unwrap().as_str().trim());
            }
        }
    }
}

fn main() {
    println!("Regex matcher");

    let file = read_line("File to match: ");
    if std::path::Path::new(&*file).exists() {
        let file = read_file(file);
        let pattern = Regex::new(&*read_line("Regex: ")).unwrap();

        for line in file.iter() {
            do_line(pattern.clone(), line);
        }
    } else {
        eprintln!("File not found");
    }
}
