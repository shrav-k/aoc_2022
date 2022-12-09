use std::collections::HashSet;

pub fn read_rope(path: &str) {
    let lines = crate::utils::read::lines_as_strings(path);

    let mut head = (0, 0);
    let mut tail = (0, 0);
    let mut visited = HashSet::new();

    for line in lines {
        let tokens: Vec<&str> = line.split_whitespace().collect();
        let steps = tokens[1].parse().unwrap(); 
        match tokens[0] {
            "U" => {
                for _ in 0..steps {
                    head = (head.0 + 1, head.1);
                    tail = tail_move(head, tail);
                    visited.insert(tail);
                }
            }
            "D" => {
                for _ in 0..steps {
                    head = (head.0 - 1, head.1);
                    tail = tail_move(head, tail);
                    visited.insert(tail);
                }
            }
            "L" => {
                for _ in 0..steps {
                    head = (head.0, head.1 -1);
                    tail = tail_move(head, tail);
                    visited.insert(tail);
                }
            }
            "R" => {
                for _ in 0..steps {
                    head = (head.0, head.1 + 1);
                    tail = tail_move(head, tail);
                    visited.insert(tail);
                }
            }
            _ => {}
        }
    }

    // println!("{:?}", visited);
    println!("total: {}", visited.len());
}

pub fn read_rope_2(path: &str) {
    let lines = crate::utils::read::lines_as_strings(path);

    let mut knots = vec![(0,0); 10];
    let mut visited = HashSet::new();

    for line in lines {
        let tokens: Vec<&str> = line.split_whitespace().collect();
        let steps = tokens[1].parse().unwrap(); 
        match tokens[0] {
            "U" => {
                for _ in 0..steps {
                    knots[0] = (knots[0].0 + 1, knots[0].1);
                    for i in 1..10 {
                        knots[i] = tail_move(knots[i-1], knots[i]);
                    }
                    visited.insert(knots[9]);
                }
            }
            "D" => {
                for _ in 0..steps {
                    knots[0] = (knots[0].0 - 1, knots[0].1);
                    for i in 1..10 {
                        knots[i] = tail_move(knots[i-1], knots[i]);
                    }
                    visited.insert(knots[9]);
                }
            }
            "L" => {
                for _ in 0..steps {
                    knots[0] = (knots[0].0, knots[0].1 - 1);
                    for i in 1..10 {
                        knots[i] = tail_move(knots[i-1], knots[i]);
                    }
                    visited.insert(knots[9]);
                }
            }
            "R" => {
                for _ in 0..steps {
                    knots[0] = (knots[0].0, knots[0].1 + 1);
                    for i in 1..10 {
                        knots[i] = tail_move(knots[i-1], knots[i]);
                    }
                    visited.insert(knots[9]);
                }
            }
            _ => {}
        }
    }

    // println!("{:?}", visited);
    println!("total: {}", visited.len());
}

fn tail_move(head: (i32, i32), tail: (i32, i32)) -> (i32, i32) {
    let vec = ((head.0 - tail.0), (head.1 - tail.1));
    // println!("{:?} {:?} {:?}", head, tail, vec);
    if vec.0.abs() <= 1 && vec.1.abs() <= 1 {
        tail
    } else {
        let vec = match vec {
            (0, 0) => (0,0),
            (0, _) => (0, vec.1 / vec.1.abs()),
            (_, 0) => (vec.0 / vec.0.abs(), 0),
            (_, _) => (vec.0 / vec.0.abs(), vec.1 / vec.1.abs())
        };
        // println!("{:?} {:?} {:?}", head, tail, vec);
        (tail.0 + vec.0, tail.1 + vec.1)
    }
}