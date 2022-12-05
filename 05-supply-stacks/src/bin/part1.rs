use std::io;
use supply_stacks::Stacks;

fn main() {
    let crates = io::stdin()
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

    io::stdin()
        .lines()
        .map(|line| {
            // "move n from i to j"
            line.unwrap()
                .split(' ')
                .skip(1)
                .step_by(2)
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .for_each(|m| {
            stacks.arrange(m[0], m[1] - 1, m[2] - 1);
        });

    println!("{:?}", stacks.tops().into_iter().collect::<String>());
}
