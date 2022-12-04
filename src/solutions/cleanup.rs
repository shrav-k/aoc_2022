use crate::utils::read::read_lines;

pub fn read_cleanup(path: &str) {
    let mut pairs: u64 = 0;
    if let Ok(lines) = read_lines(path) {
        for line in lines {
            if let Ok(sections) = line {
                let ranges = get_ranges(sections);
                if contained(ranges) {
                    pairs += 1
                }
            }
        }
    }
    println!("{}", pairs);
}

pub fn read_cleanup_2(path: &str) {
    let mut pairs: u64 = 0;
    if let Ok(lines) = read_lines(path) {
        for line in lines {
            if let Ok(sections) = line {
                let ranges = get_ranges(sections);
                if overlap(ranges) {
                    pairs += 1
                }
            }
        }
    }
    println!("{}", pairs);
}

fn overlap(ranges: Vec<u64>) -> bool {
    (ranges[0] <= ranges[2] && ranges[1] >= ranges[2]) ||
    (ranges[0] <= ranges[2] && ranges[1] >= ranges[3]) ||
    (ranges[0] <= ranges[3] && ranges[1] >= ranges[3]) ||
    (ranges[0] >= ranges[2] && ranges[1] <= ranges[3])
}

fn contained(ranges: Vec<u64>) -> bool {
    (ranges[0] <= ranges[2] && ranges[1] >= ranges[3]) ||
    (ranges[0] >= ranges[2] && ranges[1] <= ranges[3])
}

fn get_ranges(sections: String) -> Vec<u64> {
    let comma: Vec<&str> = sections.split(',').collect();
    let first: Vec<&str> = comma[0].split('-').collect();
    let second: Vec<&str> = comma[1].split('-').collect();

    let mut ranges: Vec<u64> = Vec::new();
    for s in first {
        ranges.push(s.parse().unwrap());
    }
    for s in second {
        ranges.push(s.parse().unwrap());
    }
    ranges
}

