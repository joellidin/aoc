use std::{collections::HashMap, str::FromStr};

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub enum MonkeyYell {
    Number(u64),
    Operation(String),
}

impl FromStr for MonkeyYell {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v = s.trim().split(' ').collect::<Vec<_>>();
        if v.len() == 1 {
            Ok(MonkeyYell::Number(v[0].parse().unwrap()))
        } else {
            Ok(MonkeyYell::Operation(s.to_owned()))
        }
    }
}

fn contain_humn(monkey: &str, monkeys: &HashMap<&str, MonkeyYell>) -> bool {
    match monkeys.get(monkey).unwrap() {
        MonkeyYell::Number(_) => false,
        MonkeyYell::Operation(s) => {
            if s.contains("humn") {
                true
            } else {
                let parts = s.split(' ').collect::<Vec<_>>();
                contain_humn(parts[0], monkeys) || contain_humn(parts[2], monkeys)
            }
        }
    }
}

fn get_monkey_number(monkey: &str, monkeys: &HashMap<&str, MonkeyYell>) -> u64 {
    match monkeys.get(monkey).unwrap() {
        MonkeyYell::Number(n) => *n,
        MonkeyYell::Operation(s) => {
            let parts = s.split(' ').collect::<Vec<_>>();
            match parts[1] {
                "+" => get_monkey_number(parts[0], monkeys) + get_monkey_number(parts[2], monkeys),
                "-" => get_monkey_number(parts[0], monkeys) - get_monkey_number(parts[2], monkeys),
                "*" => get_monkey_number(parts[0], monkeys) * get_monkey_number(parts[2], monkeys),
                "/" => get_monkey_number(parts[0], monkeys) / get_monkey_number(parts[2], monkeys),
                _ => panic!("Unknown operation"),
            }
        }
    }
}

fn test_equality(monkey: &str, monkeys: &HashMap<&str, MonkeyYell>) -> u64 {
    let mut res = 0;
    if let MonkeyYell::Operation(s) = monkeys.get(monkey).unwrap() {
        let mut v = s.split(' ').collect::<Vec<_>>();
        let (mut left, mut right) = (v[0], v[2]);
        v[1] = "-";
        loop {
            let (calc_side, next_branch) = if contain_humn(left, monkeys) || left == "humn" {
                (right, left)
            } else {
                (left, right)
            };

            match (v[1], calc_side == right) {
                ("+", _) => res -= get_monkey_number(calc_side, monkeys),
                ("-", true) => res += get_monkey_number(calc_side, monkeys),
                ("-", false) => res = get_monkey_number(calc_side, monkeys) - res,
                ("*", _) => res /= get_monkey_number(calc_side, monkeys),
                ("/", true) => res *= get_monkey_number(calc_side, monkeys),
                ("/", false) => res = get_monkey_number(calc_side, monkeys) / res,
                _ => panic!("Unknown operation"),
            }
            if [left, right].contains(&"humn") {
                break;
            }
            if let MonkeyYell::Operation(s) = monkeys.get(next_branch).unwrap() {
                v = s.split(' ').collect::<Vec<_>>();
                (left, right) = (v[0], v[2]);
            }
        }
    }
    res
}

pub fn generator(input: &str) -> HashMap<&str, MonkeyYell> {
    let mut monkeys = HashMap::new();
    input
        .trim()
        .split('\n')
        .map(|line| {
            let (name, monkey_yell) = line.split_once(':').unwrap();
            monkeys.insert(name, monkey_yell.trim().parse::<MonkeyYell>().unwrap());
        })
        .for_each(drop);
    monkeys

}

pub fn part_1(input: &HashMap<&str, MonkeyYell>) -> u64 {
    get_monkey_number("root", input)
}

pub fn part_2(input: &HashMap<&str, MonkeyYell>) -> u64 {
    test_equality("root", input)
}
