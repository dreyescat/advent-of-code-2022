use std::io;
use supply_stacks::read_stacks;

fn main() {
    let mut stacks = read_stacks(&mut io::stdin().lock());
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
