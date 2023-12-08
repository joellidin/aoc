use std::collections::HashMap;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Directions {
    Left,
    Right,
}

impl Directions {
    fn from_char(c: char) -> Self {
        match c {
            'L' => Directions::Left,
            'R' => Directions::Right,
            _ => panic!("Invalid card character"),
        }
    }
}

pub fn generator(input: &str) -> (Vec<Directions>, HashMap<String, (String, String)>) {
    if let Some((instructions_str, map_str)) = input.split_once("\n\n") {
        let instructions = instructions_str
            .chars()
            .map(Directions::from_char)
            .collect();
        let mut map = HashMap::new();

        map_str.lines().for_each(|l| {
            let (key, destinations) = l.split_once(" = ").unwrap();
            let destinations = destinations.replace(['(', ')'], "");
            let (left_destination, right_destination) = destinations.split_once(", ").unwrap();
            map.insert(
                key.to_string(),
                (left_destination.to_string(), right_destination.to_string()),
            );
        });

        return (instructions, map);
    }
    panic!("Error when parsing input.");
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    a / gcd(a, b) * b
}

fn lcm_of_vec(numbers: &[u64]) -> u64 {
    numbers.iter().cloned().reduce(lcm).unwrap_or(1)
}

pub fn part_1((instructions, map): &(Vec<Directions>, HashMap<String, (String, String)>)) -> u32 {
    let mut instructions_cycle = instructions.iter().cycle();
    let mut current = "AAA".to_string();
    let mut steps = 0;

    while current != *"ZZZ" {
        let new_nodes = map.get(&current).unwrap();
        steps += 1;
        current = match instructions_cycle.next() {
            Some(Directions::Left) => new_nodes.0.to_owned(),
            Some(Directions::Right) => new_nodes.1.to_owned(),
            None => panic!("Something wrong with the instructions cycle."),
        };
    }
    steps
}

pub fn part_2((instructions, map): &(Vec<Directions>, HashMap<String, (String, String)>)) -> u64 {
    let mut instructions_cycle = instructions.iter().cycle();
    let mut currents = map.keys().filter(|s| s.ends_with('A')).collect::<Vec<_>>();
    let mut steps = vec![];
    let mut current_steps = 0;

    while !currents.is_empty() {
        let next_instruction = instructions_cycle.next();
        let mut new_currents = vec![];
        current_steps += 1;
        for current in currents {
            let new_nodes = map.get(current).unwrap();
            let new_current = match next_instruction {
                Some(Directions::Left) => &new_nodes.0,
                Some(Directions::Right) => &new_nodes.1,
                None => panic!("Something wrong with the instructions cycle."),
            };
            if new_current.ends_with('Z') {
                steps.push(current_steps);
            } else {
                new_currents.push(new_current);
            }
        }
        currents = new_currents;
    }
    lcm_of_vec(&steps)
}
