use std::io;
use supply_stacks::{read_moves, read_stacks};

fn main() {
    let mut stacks = read_stacks(&mut io::stdin().lock());
    read_moves(&mut io::stdin().lock()).for_each(|m| {
        stacks.arrange(m[0], m[1] - 1, m[2] - 1);
    });

    println!("{:?}", stacks.tops().into_iter().collect::<String>());
}
