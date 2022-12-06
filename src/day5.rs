use std::str::FromStr;

#[derive(Debug, Clone)]
struct Dock {
    stacks: Vec<Vec<char>>,
}

impl FromStr for Dock {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s
            .split('\n')
            .collect::<Vec<_>>()
            .iter()
            .rev()
            .skip(1)
            .map(|line| line.chars().skip(1).step_by(4).collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let mut stacks = vec![Vec::<char>::new(); lines[0].len()];
        for line in lines.iter() {
            for (i, c) in line.iter().enumerate() {
                if *c != ' ' {
                    stacks[i].push(*c);
                }
            }
        }
        Ok(Dock { stacks })
    }
}

impl Dock {
    fn use_crate_mover_9000(&mut self, instructions: &str) {
        for instruction in instructions.trim().split("\n") {
            let (n_crates, from_stack, to_stack) = Dock::parse_instruction(instruction);

            for _ in 0..n_crates {
                let item = self.stacks[from_stack - 1].pop().unwrap();
                self.stacks[to_stack - 1].push(item);
            }
        }
    }

    fn use_crate_mover_9001(&mut self, instructions: &str) {
        for instruction in instructions.trim().split("\n") {
            let (n_crates, from_stack, to_stack) = Dock::parse_instruction(instruction);
            let end = self.stacks[from_stack - 1].len();
            let items: Vec<_> = self.stacks[from_stack - 1]
                .drain(end - n_crates..end)
                .collect();
            for item in items {
                self.stacks[to_stack - 1].push(item.to_owned());
            }
        }
    }

    fn parse_instruction(instruction: &str) -> (usize, usize, usize) {
        let (_, split) = instruction.split_once("move ").unwrap();
        let (n_crates, split) = split.split_once(" ").unwrap();
        let n_crates: usize = n_crates.parse().unwrap();
        let (_, split) = split.split_once("from ").unwrap();
        let (from_stack, split) = split.split_once(" ").unwrap();
        let from_stack: usize = from_stack.parse().unwrap();
        let (_, to_stack) = split.split_once("to ").unwrap();
        let to_stack: usize = to_stack.parse().unwrap();

        (n_crates, from_stack, to_stack)
    }
}

pub fn solution() {
    let input = std::fs::read_to_string("data/day5.txt").unwrap();
    let (stacks, instructions): (&str, &str) = input.split_once("\n\n").unwrap();
    let mut stacks: Dock = stacks.parse().unwrap();
    let mut stacks2 = stacks.clone();

    stacks.use_crate_mover_9000(instructions);
    let mut result = "".to_owned();
    for i in 0..stacks.stacks.len() {
        result = result + &stacks.stacks[i].pop().unwrap().to_string();
    }
    println!("The top crates are (part 1): {}", result);

    stacks2.use_crate_mover_9001(instructions);
    let mut result = "".to_owned();
    for i in 0..stacks2.stacks.len() {
        result = result + &stacks2.stacks[i].pop().unwrap().to_string();
    }
    println!("The top crates are (part 2): {}", result);
}
