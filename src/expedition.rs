pub mod elf;
pub mod snack;

use elf::Elf;

use min_max_heap::MinMaxHeap;

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
}
