use std::str::FromStr;

#[derive(Debug)]
struct Stacks {
    stacks: Vec<Vec<String>>,
}

impl FromStr for Stacks {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = s.split('\n').collect();
        let size = lines.len();
        let n_stacks = (lines[size - 1].len() as f32 / 4.0 + 1.0) as usize;
        let mut stacks: Vec<Vec<String>> = Vec::new();
        for i in (0..size - 1).rev() {
            for j in 1..n_stacks + 1 {
                match stacks.get(j - 1) {
                    Some(_) => (),
                    None => stacks.push(Vec::new()),
                }
                match lines[i].get(j * 4 - 3..j * 4 - 2) {
                    Some(c) => {
                        if c != " " {
                            stacks[j - 1].push(c.to_owned())
                        }
                    }
                    None => (),
                }
            }
        }
        Ok(Stacks { stacks })
    }
}

impl Stacks {
    fn move_crates(&mut self, n_crates: usize, from_stack: usize, to_stack: usize) {
        for _ in 0..n_crates {
            let item = self.stacks[from_stack - 1].pop().unwrap();
            self.stacks[to_stack - 1].push(item);
        }
    }

    fn move_crates2(&mut self, n_crates: usize, from_stack: usize, to_stack: usize) {
        let end = self.stacks[from_stack - 1].len();
        let items: Vec<_> = self.stacks[from_stack - 1].drain(end-n_crates..end).collect();
        for item in items {
            self.stacks[to_stack - 1].push(item.to_owned());
        }
    }
}

pub fn solution() {
    let input = std::fs::read_to_string("data/day5.txt").unwrap();
    let (stacks, commands): (&str, &str) = input.split_once("\n\n").unwrap();
    let mut stacks: Stacks = stacks.parse().unwrap();
    for command in commands.trim().split("\n") {
        let (_, split) = command.split_once("move ").unwrap();
        let (n_crates, split) = split.split_once(" ").unwrap();
        let n_crates: usize = n_crates.parse().unwrap();
        let (_, split) = split.split_once("from ").unwrap();
        let (from_stack, split) = split.split_once(" ").unwrap();
        let from_stack: usize = from_stack.parse().unwrap();
        let (_, to_stack) = split.split_once("to ").unwrap();
        let to_stack: usize = to_stack.parse().unwrap();
        stacks.move_crates(n_crates, from_stack, to_stack);
    }
    let mut result = "".to_owned();
    for i in 0..stacks.stacks.len() {
        result = result + &stacks.stacks[i].pop().unwrap();
    }
    println!("The top crates are (part 1): {}", result);

    let (stacks, commands): (&str, &str) = input.split_once("\n\n").unwrap();
    let mut stacks: Stacks = stacks.parse().unwrap();
    for command in commands.trim().split("\n") {
        let (_, split) = command.split_once("move ").unwrap();
        let (n_crates, split) = split.split_once(" ").unwrap();
        let n_crates: usize = n_crates.parse().unwrap();
        let (_, split) = split.split_once("from ").unwrap();
        let (from_stack, split) = split.split_once(" ").unwrap();
        let from_stack: usize = from_stack.parse().unwrap();
        let (_, to_stack) = split.split_once("to ").unwrap();
        let to_stack: usize = to_stack.parse().unwrap();
        stacks.move_crates2(n_crates, from_stack, to_stack);
    }
    let mut result = "".to_owned();
    for i in 0..stacks.stacks.len() {
        result = result + &stacks.stacks[i].pop().unwrap();
    }
    println!("The top crates are (part 2): {}", result);
}
