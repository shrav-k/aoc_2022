mod expedition;
mod utils;

fn main() {
    let exp = utils::read_expedition("./puzzles/1/input.txt");
    println!("Most calories: {}", exp.most_calories());
}
