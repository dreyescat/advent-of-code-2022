// Make it generic Stacks<T>
#[derive(Debug)]
pub struct Stacks {
    stacks: Vec<Vec<char>>,
}

impl Stacks {
    pub fn new(n: usize) -> Self {
        Self {
            stacks: vec![vec![]; n],
        }
    }

    pub fn push(&mut self, n: usize, e: char) {
        self.stacks[n].push(e);
    }

    pub fn pop(&mut self, n: usize) -> Option<char> {
        self.stacks[n].pop()
    }

    pub fn arrange_in_order(&mut self, n: usize, from: usize, to: usize) {
        let mut f = vec![];
        for _i in 0..n {
            f.push(self.pop(from).unwrap())
        }
        f.reverse();
        self.stacks[to].append(&mut f);
    }

    pub fn arrange(&mut self, n: usize, from: usize, to: usize) {
        // Using a temporary vector because I have not been able to solve the
        // problem with borrowing two mutable:
        // self.push(to, self.pop(from).unwrap())
        //  ^             ^  both self are trying to borrow mutable.
        let mut f = vec![];
        for _i in 0..n {
            f.push(self.pop(from).unwrap())
        }
        self.stacks[to].append(&mut f);
    }

    // What about an iterator...
    pub fn tops(&self) -> Vec<&char> {
        self.stacks
            .iter()
            .map(|stack| stack.last().unwrap())
            .collect()
    }
}
