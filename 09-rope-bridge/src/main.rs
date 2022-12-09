pub mod position;

use crate::position::{Motion, Position};
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
        .map(|line| Motion::from_str(&line.unwrap()).unwrap())
        .for_each(|motion| {
            head.motion(&motion);
            if !tail.is_adjacent(&head) {
                move_tail(&mut tail, &head, &motion, &mut visited);
            }
        });
    println!("{}", visited.len());
}

fn move_tail(
    tail: &mut Position,
    head: &Position,
    motion: &Motion,
    visited: &mut HashSet<Position>,
) {
    match motion {
        Motion::Right(_steps) => {
            if tail.x > head.x {
                tail.motion(&Motion::Down(1));
            }
            if tail.x < head.x {
                tail.motion(&Motion::Up(1));
            }
            assert!(tail.x == head.x);
            for _i in tail.y + 1..head.y {
                tail.motion(&Motion::Right(1));
                visited.insert(*tail);
            }
        }
        Motion::Left(_steps) => {
            if tail.x > head.x {
                tail.motion(&Motion::Down(1));
            }
            if tail.x < head.x {
                tail.motion(&Motion::Up(1));
            }
            assert!(tail.x == head.x);
            for _i in head.y + 1..tail.y {
                tail.motion(&Motion::Left(1));
                visited.insert(*tail);
            }
        }
        Motion::Up(_steps) => {
            if tail.y > head.y {
                tail.motion(&Motion::Left(1));
            }
            if tail.y < head.y {
                tail.motion(&Motion::Right(1));
            }
            assert!(tail.y == head.y);
            for _i in tail.x + 1..head.x {
                tail.motion(&Motion::Up(1));
                visited.insert(*tail);
            }
        }
        Motion::Down(_steps) => {
            if tail.y > head.y {
                tail.motion(&Motion::Left(1));
            }
            if tail.y < head.y {
                tail.motion(&Motion::Right(1));
            }
            assert!(tail.y == head.y);
            for _i in head.x + 1..tail.x {
                tail.motion(&Motion::Down(1));
                visited.insert(*tail);
            }
        }
    }
}
