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
    // X = Need to Lose, Y = Need to Draw, Z = Need to Win
    let scores_2 = HashMap::from([
        ("A X", *scores.get("A Z").unwrap()),
        ("A Y", *scores.get("A X").unwrap()),
        ("A Z", *scores.get("A Y").unwrap()),
        ("B X", *scores.get("B X").unwrap()),
        ("B Y", *scores.get("B Y").unwrap()),
        ("B Z", *scores.get("B Z").unwrap()),
        ("C X", *scores.get("C Y").unwrap()),
        ("C Y", *scores.get("C Z").unwrap()),
        ("C Z", *scores.get("C X").unwrap()),
    ]);
    let score = io::stdin()
        .lines()
        .into_iter()
        .map(|line| {
            let key = line.unwrap();
            (
                scores.get(key.as_str()).unwrap(),
                scores_2.get(key.as_str()).unwrap(),
            )
        })
        .fold((0, 0), |(acc1, acc2), (s1, s2)| (acc1 + s1, acc2 + s2));
    println!("{:?}", score);
}
