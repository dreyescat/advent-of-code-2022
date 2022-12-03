use rucksack_reorganization::priority_rucksack;
use std::io;

fn main() {
    let score: u32 = io::stdin()
        .lines()
        .map(|line| priority_rucksack(line.unwrap().as_str()))
        .sum();
    println!("{}", score);
}
