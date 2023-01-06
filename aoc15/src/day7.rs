use std::{collections::HashMap, str::FromStr};

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
enum LogicGate {
    Number(u16),
    Operation(String),
}

impl FromStr for LogicGate {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(num) = s.parse::<u16>() {
            Ok(LogicGate::Number(num))
        } else {
            Ok(LogicGate::Operation(s.to_owned()))
        }
    }
}

fn get_wire_value(wire: String, logic_gates: &mut HashMap<String, LogicGate>) -> u16 {
    if let Ok(num) = wire.parse::<u16>() {
        return num;
    }
    if let Some(LogicGate::Number(num)) = logic_gates.get(&wire) {
        return *num;
    }
    let logic_gate = logic_gates.get(&wire).unwrap().clone();
    let res = match logic_gate {
        LogicGate::Number(n) => n,
        LogicGate::Operation(s) => {
            let parts = s.split(' ').collect::<Vec<_>>();
            if parts.len() == 3 {
                match parts[1] {
                    "AND" => {
                        get_wire_value(parts[0].to_owned(), logic_gates)
                            & get_wire_value(parts[2].to_owned(), logic_gates)
                    }
                    "OR" => {
                        get_wire_value(parts[0].to_owned(), logic_gates)
                            | get_wire_value(parts[2].to_owned(), logic_gates)
                    }
                    "LSHIFT" => {
                        get_wire_value(parts[0].to_owned(), logic_gates)
                            << parts[2].parse::<u16>().unwrap()
                    }
                    "RSHIFT" => {
                        get_wire_value(parts[0].to_owned(), logic_gates)
                            >> parts[2].parse::<u16>().unwrap()
                    }
                    _ => panic!("Unknown operation"),
                }
            } else {
                match parts[0] {
                    "NOT" => !get_wire_value(parts[1].to_owned(), logic_gates),
                    _ => get_wire_value(parts[0].to_owned(), logic_gates),
                }
            }
        }
    };
    logic_gates.insert(wire, LogicGate::Number(res));
    res
}

fn parse_input(input: &str) -> HashMap<String, LogicGate> {
    input
        .lines()
        .map(|line| {
            let parts = line.split(" -> ").collect::<Vec<_>>();
            (parts[1].to_owned(), parts[0].parse::<LogicGate>().unwrap())
        })
        .collect()
}

pub fn solution() {
    let mut gates = parse_input(include_str!("../data/day7.txt"));
    let mut gates2 = gates.clone();

    let part1 = get_wire_value("a".to_owned(), &mut gates);
    println!("The signal on wire a for part 1 is {part1}");

    gates2.insert("b".to_owned(), LogicGate::Number(part1));
    let part2 = get_wire_value("a".to_owned(), &mut gates2);
    println!("The signal on wire a for part 2 is {part2}");
}
