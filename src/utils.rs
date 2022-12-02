mod read;

use crate::expedition::{
    Expedition,
    elf::Elf
};

pub fn test() {
    let mut expedition = Expedition::default();

    let mut elf1 = Elf::default();
    elf1.add_snack(1000);
    elf1.add_snack(2000);
    elf1.add_snack(3000);
    expedition.add_elf(elf1);

    let mut elf2 = Elf::default();
    elf2.add_snack(4000);
    expedition.add_elf(elf2);

    let mut elf3 = Elf::default();
    elf3.add_snack(5000);
    elf3.add_snack(6000);
    expedition.add_elf(elf3);

    let mut elf4 = Elf::default();
    elf4.add_snack(7000);
    elf4.add_snack(8000);
    elf4.add_snack(9000);
    expedition.add_elf(elf4);

    let mut elf5 = Elf::default();
    elf5.add_snack(10000);
    expedition.add_elf(elf5);

    println!("{}", expedition.most_calories());
}


