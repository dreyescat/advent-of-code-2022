use std::fmt;

pub struct Monkey {
    pub items: Vec<u64>,
    pub operation: Box<dyn Fn(u64) -> u64>,
    pub divisor: u64,
    pub right_monkey: usize,
    pub left_monkey: usize,
}

// Implement Debug trait because Functions does not implement the default Debug.
impl fmt::Debug for Monkey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Monkey")
            .field("items", &self.items)
            .finish()
    }
}

impl Monkey {
    pub fn from_vec(attributes: &[String]) -> Self {
        Self {
            items: attributes[1]
                .strip_prefix("  Starting items: ")
                .unwrap()
                .split(", ")
                .map(|item| item.parse().unwrap())
                .collect(),
            // This function might be a Monkey method...
            operation: {
                let (operator, value) = attributes[2]
                    .strip_prefix("  Operation: new = old ")
                    .unwrap()
                    .split_once(' ')
                    .unwrap();
                match (operator, value) {
                    ("+", "old") => Box::new(|x| x + x),
                    ("*", "old") => Box::new(|x| x * x),
                    ("+", val) => {
                        let val: u64 = val.parse().unwrap();
                        Box::new(move |x| x + val)
                    }
                    ("*", val) => {
                        let val: u64 = val.parse().unwrap();
                        Box::new(move |x| x * val)
                    }
                    _ => panic!("Unsupported operation {} {}", operator, value),
                }
            },
            divisor: attributes[3]
                .strip_prefix("  Test: divisible by ")
                .unwrap()
                .parse()
                .unwrap(),
            right_monkey: attributes[4]
                .strip_prefix("    If true: throw to monkey ")
                .unwrap()
                .parse()
                .unwrap(),
            left_monkey: attributes[5]
                .strip_prefix("    If false: throw to monkey ")
                .unwrap()
                .parse()
                .unwrap(),
        }
    }

    pub fn test(&self, item: u64) -> usize {
        if item % self.divisor == 0 {
            self.right_monkey
        } else {
            self.left_monkey
        }
    }
}
