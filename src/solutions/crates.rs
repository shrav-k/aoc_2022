use crate::utils::read::lines_as_strings;

pub fn read_crates(path: &str) {
    let lines = lines_as_strings(path);
    let mut line_index = 0;

    let mut crates_lines: Vec<Vec<char>> = Vec::new();
    loop {
        if lines[line_index].contains('[') {
            let chars = lines[line_index].chars().collect();
            crates_lines.push(chars);
            line_index += 1;
        } else {
            break
        }
    }

    let index_string = &lines[line_index][..];
    line_index += 1;

    let indices: Vec<usize> = index_string
    .split_whitespace()
    .map(|index| index_map(index, index_string))
    .collect();

    let mut crates: Vec<Vec<char>> = Vec::new();
    for _ in 0..indices.len() {
        crates.push(Vec::new())
    }
    for i in (0..crates_lines.len()).rev() {
        for j in 0..indices.len() {
            let c = crates_lines[i][indices[j]];
            if c >= 'A' && c <= 'Z' {
                crates[j].push(c);
            }
        }
    }

    println!("{:?}", crates);

    let mut move_lines: Vec<Vec<usize>> = Vec::new();
    while line_index < lines.len() {
        let split: Vec<&str> = lines[line_index].split_whitespace().collect();
        let mut crate_indices = Vec::new();
        for s in split {
            match s.parse::<usize>() {
                Ok(n) => crate_indices.push(n),
                Err(_) => {}
            }
        }
        if crate_indices.len() > 0 {
            move_lines.push(crate_indices);
        }
        line_index += 1;
    }

    println!("{:?}", move_lines);

    // for to_move in move_lines {
    //     for _ in 0..to_move[0] {
    //         let c = crates[to_move[1]-1].pop().unwrap();
    //         crates[to_move[2]-1].push(c);
    //     }
    // }
    // println!("{:?}", crates);

    for to_move in move_lines {
        let mut picked_up = Vec::new();
        for _ in 0..to_move[0] {
            let c = crates[to_move[1]-1].pop().unwrap();
            picked_up.push(c);
        }
        for _ in 0..picked_up.len() {
            let c = picked_up.pop().unwrap();
            crates[to_move[2]-1].push(c);
        }
    }
    println!("{:?}", crates);

    let mut result = String::new();
    for mut cr in crates {
        result.push(cr.pop().unwrap());
    }
    println!("{:?}", result);
}

fn index_map(index: &str, original: &str) -> usize {
    (addr_of(index) - addr_of(original)).into()
}

fn addr_of(s: &str) -> usize {
    s.as_ptr() as usize
}