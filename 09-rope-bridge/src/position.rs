use std::str::FromStr;

#[derive(Debug)]
pub enum Motion {
    Right,
    Left,
    Up,
    Down,
}

impl FromStr for Motion {
    type Err = ();

    fn from_str(input: &str) -> Result<Motion, Self::Err> {
        match input.chars().next().unwrap() {
            'R' => Ok(Motion::Right),
            'L' => Ok(Motion::Left),
            'U' => Ok(Motion::Up),
            'D' => Ok(Motion::Down),
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
            Motion::Right => self.y += 1,
            Motion::Left => self.y -= 1,
            Motion::Up => self.x += 1,
            Motion::Down => self.x -= 1,
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
