use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
struct Divisble(usize);
#[derive(Debug, Copy, Clone)]
struct MonkeyNumber(usize);
#[derive(Debug)]
struct Monkey {
    starting_items: Vec<usize>,
    operation: String,
    test: (Divisble, MonkeyNumber, MonkeyNumber),
}

impl Monkey {
    fn inspect_items(&self) -> Vec<usize> {
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
                                .parse::<usize>()
                                .unwrap();
                    }
                } else if self.operation.starts_with("+ old") {
                    new_val = item + item;
                } else {
                    new_val = item
                        + self
                            .operation
                            .strip_prefix("+ ")
                            .unwrap()
                            .parse::<usize>()
                            .unwrap();
                }
                new_val
            })
            .collect::<Vec<usize>>()
    }

    fn test_worry_level(&self, worry_level: &usize) -> MonkeyNumber {
        if worry_level % self.test.0 .0 == 0 {
            self.test.1
        } else {
            self.test.2
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
            .map(|n| n.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
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

fn get_monkeys(input: &str) -> Vec<Monkey> {
    input
        .split("\n\n")
        .map(|monkey| monkey.parse::<Monkey>().unwrap())
        .collect()
}

pub fn part_1(input: &str) -> usize {
    let mut monkeys = get_monkeys(input);
    let mut processed_items = vec![0; monkeys.len()];
    for _ in 0..20 {
        for i in 0..monkeys.len() {
            for item in monkeys[i].inspect_items().iter() {
                let to_monkey = monkeys[i].test_worry_level(&(item / 3));
                let new_val = item / 3;
                monkeys[to_monkey.0].starting_items.push(new_val);
                monkeys[i].starting_items.pop();
                processed_items[i] += 1;
            }
        }
    }
    processed_items.sort_by(|a, b| b.cmp(a));
    processed_items[0] * processed_items[1]
}

pub fn part_2(input: &str) -> u64 {
    let mut monkeys = get_monkeys(input);
    let mut processed_items = vec![0; monkeys.len()];
    let mod_val = monkeys.iter().fold(1, |acc, monkey| acc * monkey.test.0 .0);
    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            for item in monkeys[i].inspect_items().iter() {
                let to_monkey = monkeys[i].test_worry_level(item);
                let new_val = item % mod_val;
                monkeys[to_monkey.0].starting_items.push(new_val);
                monkeys[i].starting_items.pop();
                processed_items[i] += 1;
            }
        }
    }
    processed_items.sort_by(|a, b| b.cmp(a));
    processed_items[0] as u64 * processed_items[1] as u64
}
