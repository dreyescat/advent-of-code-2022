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
}
