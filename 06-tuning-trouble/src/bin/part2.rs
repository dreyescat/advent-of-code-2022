use std::io;
use tuning_trouble::message_start;

fn main() {
    let count: u32 = io::stdin()
        .lines()
        .map(|line| message_start(line.unwrap().as_str()))
        .sum();

    println!("{}", count);
}
