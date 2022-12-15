use monkey_in_the_middle::monkey::Monkey;
use std::io;

fn main() {
    let lines: Vec<String> = io::stdin()
        .lines()
        .map(|line| line.unwrap())
        .filter(|line| !line.is_empty())
        .collect();

    println!("{}", level(&lines, 10000));
}

pub fn level(lines: &[String], rounds: u32) -> u64 {
    let mut monkeys: Vec<Monkey> = lines.chunks(6).map(Monkey::from_vec).collect();
    let worry_reduction = monkeys
        .iter()
        .fold(1, |acc, monkey| lcm(acc, monkey.divisor));

    let mut inspections: Vec<u64> = vec![0; monkeys.len()];
    for _round in 1..=rounds {
        for i in 0..monkeys.len() {
            inspections[i] += monkeys[i].items.len() as u64;
            for j in 0..monkeys[i].items.len() {
                let item = (monkeys[i].operation)(monkeys[i].items[j]) % worry_reduction;
                let to_monkey = monkeys[i].test(item);
                monkeys[to_monkey as usize].items.push(item);
            }
            monkeys[i].items.clear();
        }
    }
    inspections.sort_by(|a, b| b.cmp(a));
    inspections[0] * inspections[1]
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}
