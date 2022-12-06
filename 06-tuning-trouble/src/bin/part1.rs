use std::io;
use tuning_trouble::packet_start;

fn main() {
    let count: u32 = io::stdin()
        .lines()
        .map(|line| packet_start(line.unwrap().as_str()))
        .sum();

    println!("{}", count);
}
