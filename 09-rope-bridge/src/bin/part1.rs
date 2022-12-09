use rope_bridge::position::{Motion, Position};
use std::collections::HashSet;
use std::io;
use std::str::FromStr;

fn main() {
    let mut visited: HashSet<Position> = HashSet::new();
    let mut head = Position::new();
    let mut tail = Position::new();
    visited.insert(tail);
    io::stdin()
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let (direction, steps) = line.split_once(' ').unwrap();
            (Motion::from_str(direction).unwrap(), steps.parse().unwrap())
        })
        .for_each(|(motion, steps)| {
            for _i in 0..steps {
                head.motion(&motion);
                if !tail.is_adjacent(&head) {
                    move_tail(&head, &mut tail);
                    visited.insert(tail);
                }
            }
        });
    println!("{}", visited.len());
}

fn move_tail(head: &Position, position: &mut Position) {
    if position.x > head.x {
        position.motion(&Motion::Down);
    }
    if position.x < head.x {
        position.motion(&Motion::Up);
    }
    if position.y > head.y {
        position.motion(&Motion::Left);
    }
    if position.y < head.y {
        position.motion(&Motion::Right);
    }
}
