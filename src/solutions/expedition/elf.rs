use min_max_heap::MinMaxHeap;
use std::cmp::Ordering;

use crate::expedition::snack::Snack;

#[derive(Default)]
pub struct Elf {
    snacks: MinMaxHeap<Snack>,
    pub calories: u64
}

impl Ord for Elf {
    fn cmp(&self, other: &Self) -> Ordering {
        self.calories.cmp(&other.calories)
    }
}

impl PartialOrd for Elf {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Elf {
    fn eq(&self, other: &Self) -> bool {
        self.calories == other.calories
    }
}

impl Eq for Elf {}

impl Elf {
    pub fn add_snack(&mut self, calories: u64) {
        let snack = Snack {
            calories
        };
        self.snacks.push(snack);
        self.calories += calories;
    }

    pub fn add_snacks(&mut self, cal_vec: &Vec<u64>) {
        for i in 0..cal_vec.len() {
            let calories = cal_vec[i];
            let snack = Snack {
                calories
            };
            self.snacks.push(snack);
            self.calories += calories;
        }
    }
}