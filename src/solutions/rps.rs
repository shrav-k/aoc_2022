use crate::utils::read::read_lines;

pub fn read_rps(path: &str) {
    let mut score = 0;
    if let Ok(lines) = read_lines(path) {
        for line in lines {
            if let Ok(code) = line {
                match code.as_str() {
                    "A X" => score += 4,
                    "A Y" => score += 8,
                    "A Z" => score += 9,
                    "B X" => score += 1,
                    "B Y" => score += 5,
                    "B Z" => score += 9,
                    "C X" => score += 1,
                    "C Y" => score += 2,
                    "C Z" => score += 6,
                    _ => {}
                }
            }
        }
    }
    println!("{}", score);
}