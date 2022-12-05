use std::io::BufRead;

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

pub fn read_stacks(reader: &mut impl BufRead) -> Stacks {
    let crates = reader
        .lines()
        .map_while(|line| {
            let line = line.unwrap();
            if line.len() > 1 {
                Some(line.chars().skip(1).step_by(4).collect::<Vec<_>>())
            } else {
                None
            }
        })
        .collect::<Vec<Vec<_>>>();
    // Last line contains the stacks names/numbers.
    // Use this line to know how many stacks.
    // 1   2   3   4   5   6   7   8   9
    let stack_number = crates[crates.len() - 1].len() as usize;
    let mut stacks = Stacks::new(stack_number);
    // Skip the line with the stacks names/numbers.
    // Use into_iter to "move" elements instead of borrowing.
    for items in crates.into_iter().rev().skip(1) {
        for (i, item) in items
            .into_iter()
            .enumerate()
            .filter(|(_i, item)| *item != ' ')
        {
            stacks.push(i, item);
        }
    }

    stacks
}

pub fn read_moves(reader: &mut impl BufRead) -> impl Iterator<Item = Vec<usize>> + '_ {
    reader.lines().map(|line| {
        // "move n from i to j"
        line.unwrap()
            .split(' ')
            .skip(1)
            .step_by(2)
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<_>>()
    })
}
