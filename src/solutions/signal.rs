pub fn read_signal(path: &str) {
    let lines = crate::utils::read::lines_as_strings(path);
    let thresholds = vec![20, 60, 100, 140, 180, 220];

    let mut x = 1;
    let mut cycles = 0;
    let mut total = 0;
    for line in lines {
        let tokens: Vec<&str> = line.split_whitespace().collect();
        match tokens[0] {
            "noop" => {
                cycles += 1;
                for t in thresholds.iter() {
                    if cycles == *t {
                        total += t * x;
                        // println!("t: {}, x: {}", t, x);
                        // println!("total: {}", total);
                    }
                }
            }
            "addx" => {
                cycles += 2;
                for t in thresholds.iter() {
                    if cycles == *t || cycles == *t + 1 {
                        total += t * x;
                        // println!("t: {}, x: {}", t, x);
                        // println!("total: {}", total);
                    }
                }
                x += tokens[1].parse::<i32>().unwrap();
            }
            _ => {}
        }
    }
    println!("total: {}", total);
}

pub fn read_crt(path: &str) {
    let lines = crate::utils::read::lines_as_strings(path);
    let mut display = Vec::new();

    let mut x = 1;
    let mut cycles = 0;
    for line in lines {
        let tokens: Vec<&str> = line.split_whitespace().collect();
        println!("{} {}", cycles, x);
        match tokens[0] {
            "noop" => {
                cycles += 1;
                let pos: i32 = (cycles - 1) % 40;
                if (pos - x).abs() <= 1 {
                    display.push(true);
                } else {
                    display.push(false);
                }
            }
            "addx" => {
                cycles += 1;
                let pos: i32 = (cycles - 1) % 40;
                if (pos - x).abs() <= 1 {
                    display.push(true);
                } else {
                    display.push(false);
                }
                cycles += 1;
                let pos: i32 = (cycles - 1) % 40;
                if (pos - x).abs() <= 1 {
                    display.push(true);
                } else {
                    display.push(false);
                }
                x += tokens[1].parse::<i32>().unwrap();
            }
            _ => {}
        }
    }
    //print!("{:?}", display);
    for i in 0..240 {
        if i % 40 == 0 {
            println!("");
        }
        if display[i] {
            print!("#");
        } else {
            print!(".");
        }
    }
}