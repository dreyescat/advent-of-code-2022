use camp_cleanup::assignment_pair;
use std::io;

fn main() {
    let score = io::stdin()
        .lines()
        .map(|line| {
            let assignment = line.unwrap();
            let (a1, a2) = assignment.split_once(',').unwrap();
            assignment_pair(a1, a2)
        })
        .filter(|(range1, range2)| range1.contains(range2) || range2.contains(range1))
        .count();

    println!("{:?}", score);
}
