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
    let mut row = String::from("");
    for instruction in instructions {
        match instruction.unwrap() {
            Operator::Noop => {
                process_cycle(x, &mut cycle, &mut row);
            }
            Operator::Addx(value) => {
                process_cycle(x, &mut cycle, &mut row);
                process_cycle(x, &mut cycle, &mut row);
                x += value;
            }
        }
    }
}

fn process_cycle(x: i32, cycle: &mut i32, row: &mut String) {
    if (x - 1 ..= x + 1).contains(&(*cycle % 40)) {
        row.push('#');
    } else {
        row.push('.');
    }
    *cycle += 1;
    if *cycle % 40 == 0 {
        println!("{}", row);
        row.clear();
    }
}
