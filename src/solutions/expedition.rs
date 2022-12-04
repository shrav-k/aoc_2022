pub mod elf;
pub mod snack;

use elf::Elf;

use min_max_heap::MinMaxHeap;

use crate::utils::read::read_lines;

#[derive(Default)]
pub struct Expedition {
    elves: MinMaxHeap<Elf>
}

impl Expedition {
    pub fn add_elf(&mut self, elf: Elf) {
        self.elves.push(elf);
    }

    pub fn most_calories(&self) -> u64 {
        match self.elves.peek_max() {
            Some(elf) => elf.calories,
            None => 0
        }
    }

    pub fn top_k_calories(&mut self, k: u64) -> u64 {
        let mut total = 0;
        for _ in 0..k {
            if let Some(elf) = self.elves.pop_max() {
                println!("{}", elf.calories);
                total += elf.calories;
            }
        }
        total
    }
}

pub fn read_expedition(path: &str) {
    let mut exp = Expedition::default();
    if let Ok(lines) = read_lines(path) {
        let mut cal_vec = Vec::new();
        for line in lines {
            if let Ok(s) = line {
                match s.parse::<u64>() {
                    Ok(calories) => cal_vec.push(calories),
                    Err(_) => {
                        let mut elf = Elf::default();
                        elf.add_snacks(&cal_vec);
                        exp.add_elf(elf);
                        cal_vec.clear();
                    }
                }
            }
        }
        let mut elf = Elf::default();
        elf.add_snacks(&cal_vec);
        exp.add_elf(elf);

    }
    println!("Calories: {}", exp.top_k_calories(3));
}
