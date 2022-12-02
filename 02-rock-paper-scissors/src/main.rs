use std::collections::HashMap;
use std::io;

fn main() {
    // X = 1, Y = 2, Z = 3
    // Win = 6, Draw = 3, Loss = 0
    let scores = HashMap::from([
        ("A X", 1 + 3),
        ("A Y", 2 + 6),
        ("A Z", 3 + 0),
        ("B X", 1 + 0),
        ("B Y", 2 + 3),
        ("B Z", 3 + 6),
        ("C X", 1 + 6),
        ("C Y", 2 + 0),
        ("C Z", 3 + 3),
    ]);
    let score = io::stdin()
        .lines()
        .into_iter()
        .map(|line| scores.get(line.unwrap().as_str()).unwrap())
        .sum::<u32>();
    println!("{}", score);
}
