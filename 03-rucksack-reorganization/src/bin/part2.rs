use std::io;
use rucksack_reorganization::priority_group;

fn main() {
    let mut score = 0;
    let mut lines = io::stdin().lines();
    // NOTE: How to iterate in chunks of three?
    while let (Some(first), Some(second), Some(third)) = (lines.next(), lines.next(), lines.next())
    {
        score += priority_group(&[
            first.unwrap().as_str(),
            second.unwrap().as_str(),
            third.unwrap().as_str(),
        ]);
    }
    println!("{}", score);
}
