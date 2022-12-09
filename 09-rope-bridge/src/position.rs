use std::str::FromStr;

#[derive(Debug)]
pub enum Motion {
    Right(i32),
    Left(i32),
    Up(i32),
    Down(i32),
}

impl FromStr for Motion {
    type Err = ();

    fn from_str(input: &str) -> Result<Motion, Self::Err> {
        let (direction, steps) = input.split_once(' ').unwrap();
        match (direction.chars().next().unwrap(), steps.parse().unwrap()) {
            ('R', steps) => Ok(Motion::Right(steps)),
            ('L', steps) => Ok(Motion::Left(steps)),
            ('U', steps) => Ok(Motion::Up(steps)),
            ('D', steps) => Ok(Motion::Down(steps)),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new() -> Self {
        Self { x: 0, y: 0 }
    }

    pub fn motion(&mut self, motion: &Motion) {
        match motion {
            Motion::Right(steps) => self.y += steps,
            Motion::Left(steps) => self.y -= steps,
            Motion::Up(steps) => self.x += steps,
            Motion::Down(steps) => self.x -= steps,
        }
    }

    pub fn is_adjacent(&self, other: &Position) -> bool {
        self.x.abs_diff(other.x) <= 1 && self.y.abs_diff(other.y) <= 1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn adjacent_same_position() {
        assert!(Position { x: 0, y: 0 }.is_adjacent(&Position { x: 0, y: 0 }))
    }

    #[test]
    fn adjacent_top() {
        assert!(Position { x: 1, y: 1 }.is_adjacent(&Position { x: 1, y: 2 }))
    }

    #[test]
    fn adjacent_left() {
        assert!(Position { x: 1, y: 1 }.is_adjacent(&Position { x: 1, y: 2 }))
    }

    #[test]
    fn adjacent_diagonal() {
        assert!(Position { x: 1, y: 1 }.is_adjacent(&Position { x: 0, y: 0 }))
    }

    #[test]
    fn not_adjacent() {
        assert!(!Position { x: 0, y: 0 }.is_adjacent(&Position { x: 2, y: 0 }))
    }
}
