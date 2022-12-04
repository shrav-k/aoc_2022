use crate::utils::read::read_lines;

pub fn read_rucksack_2(path: &str) {
    let mut badge_sum: u64 = 0;
    if let Ok(lines) = read_lines(path) {
        let mut rucksacks = Vec::new();
        for line in lines {
            if let Ok(items) = line {
                rucksacks.push(items);
                if rucksacks.len() >= 3 {
                    let to_add: u64 = get_badge_value(&rucksacks).into();
                    badge_sum += to_add;
                    rucksacks.clear();
                }
            }
        }
    }
    println!("{}", badge_sum);
}

fn get_badge_value(rucksacks: &Vec<String>) -> u8 {
    let first = rucksacks[0].as_str();
    let second = rucksacks[1].as_str();
    let third = rucksacks[2].as_str();

    let shared = get_badge(first, second, third);
    let priority = get_priority(shared);

    println!("shared: {}", shared);
    println!("priority: {}", priority);
    priority
}

fn get_badge(first: &str, second: &str, third: &str) -> char {
    for c in first.chars() {
        if second.contains(c) && third.contains(c) {
            return c
        }
    }
    return '0'
}

pub fn read_rucksack(path: &str) {
    let mut priority_sum: u64 = 0;
    if let Ok(lines) = read_lines(path) {
        for line in lines {
            if let Ok(items) = line {
                let to_add: u64 = get_shared_priority(items).into();
                priority_sum += to_add;
            }
        }
    }
    println!("{}", priority_sum);
}

fn get_shared_priority(items: String) -> u8 {
    let length = items.len();
    let first = &items[0..length/2];
    let second = &items[(length/2)..length];

    let shared = get_shared_char(first, second);
    let priority = get_priority(shared);

    priority
}

fn get_shared_char(first: &str, second: &str) -> char {
    for c in first.chars() {
        if second.contains(c) {
            return c
        }
    }
    return '0'
}

fn get_priority(c: char) -> u8 {
    let code = c as u8;
    if code >= 65 && code <= 90 {
        code - 65 + 27
    } else if code >= 97 && code <= 122 {
        code - 97 + 1
    } else {
        0
    }
}