use std::io;
use std::str::FromStr;

#[derive(Debug)]
enum Operator {
    Noop,
    Addx(i32),
}

impl FromStr for Operator {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let instruction: Vec<&str> = input.split_ascii_whitespace().collect();
        match &instruction[..] {
            ["noop"] => Ok(Operator::Noop),
            ["addx", value] => Ok(Operator::Addx(value.parse().unwrap())),
            _ => Err(()),
        }
    }
}

fn main() {
    let instructions = io::stdin().lines().map(|line| {
        let line = line.unwrap();
        Operator::from_str(&line)
    });

    let mut x = 1;
    let mut cycle = 0;
    let mut signal_strength = 0;
    for instruction in instructions {
        match instruction.unwrap() {
            Operator::Noop => {
                if signal_strength_cycle(cycle + 1) {
                    signal_strength += (cycle + 1) * x;
                }
                cycle += 1;
            }
            Operator::Addx(value) => {
                if signal_strength_cycle(cycle + 1) {
                    signal_strength += (cycle + 1) * x;
                }
                if signal_strength_cycle(cycle + 2) {
                    signal_strength += (cycle + 2) * x;
                }
                cycle += 2;
                x += value;
            }
        }
    }

    println!("{}", signal_strength);
}

fn signal_strength_cycle(cycle: i32) -> bool {
    cycle % 40 - 20 == 0
}
