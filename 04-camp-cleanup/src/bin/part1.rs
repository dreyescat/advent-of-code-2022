use camp_cleanup::Range;
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

fn assignment_pair(assignment1: &str, assignment2: &str) -> (Range, Range) {
    let (from1, to1) = assignment1.split_once('-').unwrap();
    let (from2, to2) = assignment2.split_once('-').unwrap();

    (
        Range::new(from1.parse().unwrap(), to1.parse().unwrap()),
        Range::new(from2.parse().unwrap(), to2.parse().unwrap()),
    )
}
