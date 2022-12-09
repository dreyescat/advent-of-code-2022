use rope_bridge::position::{Motion, Position};
use std::collections::HashSet;
use std::io;
use std::str::FromStr;

fn main() {
    let mut visited: HashSet<Position> = HashSet::new();
    let mut knots = [Position::new(); 10];
    visited.insert(knots[9]);
    io::stdin()
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let (direction, steps) = line.split_once(' ').unwrap();
            (Motion::from_str(direction).unwrap(), steps.parse().unwrap())
        })
        .for_each(|(motion, steps)| {
            for _i in 0..steps {
                knots[0].motion(&motion);
                for i in 1..=9 {
                    let head = knots[i - 1];
                    if !knots[i].is_adjacent(&head) {
                        move_knot(&head, &mut knots[i]);
                    }
                }
                visited.insert(knots[9]);
            }
        });
    println!("{}", visited.len());
}

fn move_knot(head: &Position, position: &mut Position) {
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
