use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn lines_as_strings(path: &str) -> Vec<String> {
    let mut strings = Vec::new();
    if let Ok(lines) = read_lines(path) {
        for line in lines {
            if let Ok(s) = line {
                strings.push(s);
            }
        }
    }
    strings
}

pub fn read_lines<P>(path: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(path)?;
    Ok(io::BufReader::new(file).lines())
}