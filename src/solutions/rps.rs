use crate::utils::read::read_lines;

pub fn read_rps(path: &str) {
    let mut score = 0;
    if let Ok(lines) = read_lines(path) {
        for line in lines {
            if let Ok(code) = line {
                match code.as_str() {
                    "A X" => score += 4,
                    "A Y" => score += 8,
                    "A Z" => score += 3,
                    "B X" => score += 1,
                    "B Y" => score += 5,
                    "B Z" => score += 9,
                    "C X" => score += 7,
                    "C Y" => score += 2,
                    "C Z" => score += 6,
                    _ => {}
                }
            }
        }
    }
    println!("{}", score);
}

pub fn read_rps_2(path: &str) {
    let mut score = 0;
    if let Ok(lines) = read_lines(path) {
        for line in lines {
            if let Ok(code) = line {
                match code.as_str() {
                    "A X" => score += 3,
                    "A Y" => score += 4,
                    "A Z" => score += 8,
                    "B X" => score += 1,
                    "B Y" => score += 5,
                    "B Z" => score += 9,
                    "C X" => score += 2,
                    "C Y" => score += 6,
                    "C Z" => score += 7,
                    _ => {}
                }
            }
        }
    }
    println!("{}", score);
}