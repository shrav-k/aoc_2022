pub fn read_datastream(path: &str) {
    let lines = crate::utils::read::lines_as_strings(path);
    for line in lines {
        let res = non_repeating(line);
        println!("{}", res);
    }
}

fn non_repeating(line: String) -> usize {
    let chars: Vec<char> = line.chars().collect();
    let mut last = Vec::new();
    for i in 0..chars.len() {
        let c = chars[i];
        if last.len() == 14 {
            return i;
        } else if last.contains(&c) {
            let j = last.iter().position(|x| x == &c).unwrap();
            last = last[(j+1)..last.len()].to_vec();
        } 
        last.push(c);
    }
    0
}