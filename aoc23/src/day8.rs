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

pub fn count_steps_to_end<'a, F, I>(
    start: String,
    mut instructions: I,
    map: &HashMap<String, (String, String)>,
    is_end: F,
) -> u32
where
    F: Fn(&str) -> bool,
    I: Iterator<Item = &'a Directions>,
{
    let mut current = start;
    let mut steps = 0;

    while !is_end(&current) {
        let new_nodes = map.get(&current).expect("Current node not found in map");
        steps += 1;
        current = match instructions.next() {
            Some(Directions::Left) => new_nodes.0.to_owned(),
            Some(Directions::Right) => new_nodes.1.to_owned(),
            None => panic!("Something wrong with the instructions cycle."),
        };
    }
    steps
}

pub fn part_1((instructions, map): &(Vec<Directions>, HashMap<String, (String, String)>)) -> u32 {
    count_steps_to_end(
        "AAA".to_string(),
        instructions.iter().cycle(),
        map,
        |node| node == "ZZZ",
    )
}

pub fn part_2((instructions, map): &(Vec<Directions>, HashMap<String, (String, String)>)) -> u64 {
    let instructions_cycle = instructions.iter().cycle();
    let currents = map.keys().filter(|s| s.ends_with('A'));
    let steps = currents
        .map(|start| {
            count_steps_to_end(start.to_string(), instructions_cycle.clone(), map, |node| {
                node.ends_with('Z')
            }) as u64
        })
        .collect::<Vec<_>>();
    lcm_of_vec(&steps)
}
