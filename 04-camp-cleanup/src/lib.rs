#[derive(Debug)]
pub struct Range {
    from: u32,
    to: u32,
}

impl Range {
    pub fn new(from: u32, to: u32) -> Self {
        Self { from, to }
    }

    pub fn contains(&self, other: &Range) -> bool {
        self.from <= other.from && other.to <= self.to
    }

    pub fn overlaps(&self, other: &Range) -> bool {
        self.contains(other) ||
        other.from <= self.from && self.from <= other.to ||
            other.from <= self.to && self.to <= other.to
    }
}

pub fn assignment_pair(assignment1: &str, assignment2: &str) -> (Range, Range) {
    let (from1, to1) = assignment1.split_once('-').unwrap();
    let (from2, to2) = assignment2.split_once('-').unwrap();

    (
        Range::new(from1.parse().unwrap(), to1.parse().unwrap()),
        Range::new(from2.parse().unwrap(), to2.parse().unwrap()),
    )
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn no_overlaps_1() {
        assert!(!Range::new(2, 4).overlaps(&Range::new(6, 8)))
    }

    #[test]
    fn no_overlaps_2() {
        assert!(!Range::new(2, 3).overlaps(&Range::new(4, 5)))
    }

    #[test]
    fn overlaps_1() {
        assert!(Range::new(5, 7).overlaps(&Range::new(7, 9)))
    }

    #[test]
    fn overlaps_2() {
        assert!(Range::new(2, 8).overlaps(&Range::new(3, 7)))
    }

    #[test]
    fn overlaps_3() {
        assert!(Range::new(6, 6).overlaps(&Range::new(4, 6)))
    }

    #[test]
    fn overlaps_4() {
        assert!(Range::new(2, 6).overlaps(&Range::new(4, 8)))
    }
}
