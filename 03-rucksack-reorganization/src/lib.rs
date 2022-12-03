use std::collections::HashSet;

pub fn priority_group(&[first, second, third]: &[&str; 3]) -> u32 {
    let (a, b, c) = (
        first.chars().collect::<HashSet<char>>(),
        second.chars().collect::<HashSet<char>>(),
        third.chars().collect::<HashSet<char>>(),
    );
    // NOTE: How to intersect multiple HashSets?
    // NOTE: Is cloned really necessary?
    a.intersection(&b).cloned().collect::<HashSet<_>>()
        .intersection(&c).map(priority_item).sum()
}

pub fn priority_rucksack(rucksack: &str) -> u32 {
    let (first, second) = rucksack.split_at(rucksack.len() / 2);
    let (first, second) = (
        first.chars().collect::<HashSet<char>>(),
        second.chars().collect::<HashSet<char>>(),
    );
    first.intersection(&second).map(priority_item).sum()
}

pub fn priority_item(item: &char) -> u32 {
    match *item {
        'A'..='Z' => (26 - ('Z' as u32 - *item as u32)) + 26,
        'a'..='z' => 26 - ('z' as u32 - *item as u32),
        _ => panic!("{} item out of bounds", item),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn rucksack_1() {
        assert_eq!(priority_rucksack("vJrwpWtwJgWrhcsFMMfFFhFp"), 16)
    }

    #[test]
    fn rucksack_2() {
        assert_eq!(priority_rucksack("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"), 38)
    }

    #[test]
    fn rucksack_3() {
        assert_eq!(priority_rucksack("PmmdzqPrVvPwwTWBwg"), 42)
    }

    #[test]
    fn rucksack_4() {
        assert_eq!(priority_rucksack("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn"), 22)
    }

    #[test]
    fn rucksack_5() {
        assert_eq!(priority_rucksack("ttgJtRGJQctTZtZT"), 20)
    }

    #[test]
    fn rucksack_6() {
        assert_eq!(priority_rucksack("CrZsJsPPZsGzwwsLwLmpwMDw"), 19)
    }

    #[test]
    fn group_1() {
        assert_eq!(
            priority_group(&[
                "vJrwpWtwJgWrhcsFMMfFFhFp",
                "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
                "PmmdzqPrVvPwwTWBwg"
            ]),
            18
        );
    }

    #[test]
    fn group_2() {
        assert_eq!(
            priority_group(&[
                "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
                "ttgJtRGJQctTZtZT",
                "CrZsJsPPZsGzwwsLwLmpwMDw"
            ]),
            52
        );
    }
}
