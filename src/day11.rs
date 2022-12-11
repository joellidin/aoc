use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
struct Divisble(u128);
#[derive(Debug, Copy, Clone)]
struct MonkeyNumber(usize);
#[derive(Debug)]
struct Monkey {
    starting_items: Vec<u128>,
    operation: String,
    test: (Divisble, MonkeyNumber, MonkeyNumber),
}

impl Monkey {
    fn inspect_items(&self) -> Vec<u128> {
        self.starting_items
            .iter()
            .map(|item| {
                let new_val;
                if self.operation.starts_with('*') {
                    if self.operation.starts_with("* old") {
                        new_val = item * item;
                    } else {
                        new_val = item
                            * self
                                .operation
                                .strip_prefix("* ")
                                .unwrap()
                                .parse::<u128>()
                                .unwrap();
                    }
                } else {
                    if self.operation.starts_with("+ old") {
                        new_val = item + item;
                    } else {
                        new_val = item
                            + self
                                .operation
                                .strip_prefix("+ ")
                                .unwrap()
                                .parse::<u128>()
                                .unwrap();
                    }
                }
                new_val
            })
            .collect::<Vec<u128>>()
    }

    fn test_worry_level(&self, worry_level: &u128) -> MonkeyNumber {
        if worry_level % self.test.0.0 == 0 {
            return self.test.1;
        } else {
            return self.test.2;
        }
    }
}

impl FromStr for Monkey {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.split('\n').map(|line| line.trim()).skip(1);
        let starting_items = lines
            .next()
            .unwrap()
            .strip_prefix("Starting items: ")
            .unwrap()
            .split(", ")
            .map(|n| n.parse::<u128>().unwrap())
            .collect::<Vec<u128>>();
        let operation = lines
            .next()
            .unwrap()
            .strip_prefix("Operation: new = old ")
            .unwrap()
            .to_owned();
        let divisible = Divisble(
            lines
                .next()
                .unwrap()
                .strip_prefix("Test: divisible by ")
                .unwrap()
                .parse()
                .unwrap(),
        );
        let true_monkey = MonkeyNumber(
            lines
                .next()
                .unwrap()
                .strip_prefix("If true: throw to monkey ")
                .unwrap()
                .parse()
                .unwrap(),
        );
        let false_monkey = MonkeyNumber(
            lines
                .next()
                .unwrap()
                .strip_prefix("If false: throw to monkey ")
                .unwrap()
                .parse()
                .unwrap(),
        );
        let test = (divisible, true_monkey, false_monkey);
        Ok(Monkey {
            starting_items,
            operation,
            test,
        })
    }
}
pub fn solution() {
    let mut monkeys = std::fs::read_to_string("data/day11.txt")
        .unwrap()
        .split("\n\n")
        .map(|monkey| monkey.parse::<Monkey>().unwrap())
        .collect::<Vec<_>>();
    let mut processed_items = vec![0; monkeys.len()];
    for _ in 0..1000 {
        for i in 0..monkeys.len() {
            for item in monkeys[i].inspect_items().iter() {
                let to_monkey = monkeys[i].test_worry_level(item);
                monkeys[to_monkey.0].starting_items.push(*item);
                monkeys[i].starting_items.pop();
                processed_items[i] += 1;
            }
        }
    }
    println!("{:?}", monkeys);
    println!("{:?}", processed_items);
}
