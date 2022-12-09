pub fn read_trees(path: &str) {
    let lines = crate::utils::read::lines_as_strings(path);
    let mut trees = Vec::new();
    for line in lines {
        let chars = line.chars();
        let mut row = Vec::new();
        for c in chars {
            match c.to_digit(10) {
                Some(n) => row.push(n),
                None => {}
            }
        }
        trees.push(row);
    }
    // println!("{:?}", trees);

    // println!("visible: {}", visible_trees(trees));
    println!("max scenic score: {}", max_scenic_score(trees));
}

fn visible_trees(trees: Vec<Vec<u32>>) -> u32 {
    let row_len = trees.len();
    let col_len = trees[0].len();

    let mut visible = Vec::new();
    for _ in 0..row_len {
        let mut row = Vec::new();
        for _ in 0..col_len {
            row.push(false);
        }
        visible.push(row)
    }

    for i in 0..row_len {
        for j in 0..col_len {
            let height = trees[i][j]; 
            
            let mut is_visible = true;
            for k in 0..i {
                if trees[k][j] >= height {
                    is_visible = false;
                    break
                }
            }
            if is_visible {
                visible[i][j] = true;
                continue
            }

            let mut is_visible = true;
            for k in (i+1)..row_len {
                if trees[k][j] >= height {
                    is_visible = false;
                    break
                }
            }
            if is_visible {
                visible[i][j] = true;
                continue
            }

            let mut is_visible = true;
            for k in 0..j {
                if trees[i][k] >= height {
                    is_visible = false;
                    break
                }
            }
            if is_visible {
                visible[i][j] = true;
                continue
            }
            let mut is_visible = true;
            for k in (j+1)..col_len {
                if trees[i][k] >= height {
                    is_visible = false;
                    break
                }
            }
            if is_visible {
                visible[i][j] = true;
                continue
            }
        }
    }
    // println!("{:?}", visible);

    let mut total = 0;
    for i in 0..row_len {
        for j in 0..col_len {
            if visible[i][j] {
                total += 1
            }
        }
    }
    total
}

fn max_scenic_score(trees: Vec<Vec<u32>>) -> u32 {
    let mut result = 0;
    let row_len = trees.len();
    let col_len = trees[0].len();

    for i in 0..row_len {
        for j in 0..col_len {
            let height = trees[i][j]; 
            let mut counts = Vec::new();
            let mut count = 0;
            for k in (0..i).rev() {
                count += 1;
                if trees[k][j] >= height {
                    break
                }
            }
            counts.push(count);

            let mut count = 0;
            for k in (i+1)..row_len {
                count += 1;
                if trees[k][j] >= height {
                    break
                }
            }
            counts.push(count);

            let mut count = 0;
            for k in (0..j).rev() {
                count += 1;
                if trees[i][k] >= height {
                    break
                }
            }
            counts.push(count);

            let mut count = 0;
            for k in (j+1)..col_len {
                count += 1;
                if trees[i][k] >= height {
                    break
                }
            }
            counts.push(count);

            let mut score = 1;
            for count in counts {
                score = score * count;
            }

            if score > result {
                result = score;
            }
        }
    }
    result
}