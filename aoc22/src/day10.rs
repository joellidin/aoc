const FILLED_CHARACTER: &str = "██";
const EMPTY_CHARACTER: &str = "░░";
pub fn part_1(input: &str) -> u32 {
    let mut register_x = 1;
    let mut cycle = 0;
    input.trim().lines().fold(0, |acc, line| {
        let signal_strength;
        let add_cycles;
        let val;
        if line == "noop" {
            add_cycles = 1;
            val = 0;
            signal_strength = if (cycle + 1 - 20) % 40 == 0 || (cycle + 1) == 20 {
                (register_x) * (cycle + 1)
            } else {
                0
            };
        } else {
            add_cycles = 2;
            val = line
                .split_once(' ')
                .map(|(_, x)| x.parse::<isize>().unwrap())
                .unwrap();
            signal_strength = if (cycle + 2 - 20) % 40 == 0 || (cycle + 2) == 20 {
                (register_x) * (cycle + 2)
            } else if (cycle + 1 - 20) % 40 == 0 || (cycle + 2) == 20 {
                (register_x) * (cycle + 1)
            } else {
                0
            };
        }

        register_x += val;
        cycle += add_cycles;
        acc + signal_strength
    }) as u32
}

pub fn part_2(input: &str) -> String {
    let mut image: Vec<Vec<&str>> = vec![Vec::new(); 6];
    let mut register_x = 1;
    let mut cycle = 0;
    input.trim().lines().for_each(|line| {
        let add_cycles;
        let val;
        if line == "noop" {
            add_cycles = 1;
            val = 0;
            let char = if (register_x - 1..=register_x + 1).contains(&(cycle % 40)) {
                FILLED_CHARACTER
            } else {
                EMPTY_CHARACTER
            };
            image[(cycle / 40) as usize].push(char);
        } else {
            add_cycles = 2;
            val = line
                .split_once(' ')
                .map(|(_, x)| x.parse::<isize>().unwrap())
                .unwrap();
            let char1 = if (register_x - 1..=register_x + 1).contains(&(cycle % 40)) {
                FILLED_CHARACTER
            } else {
                EMPTY_CHARACTER
            };
            let char2 = if (register_x - 1..=register_x + 1).contains(&((cycle + 1) % 40)) {
                FILLED_CHARACTER
            } else {
                EMPTY_CHARACTER
            };
            image[(cycle / 40) as usize].push(char1);
            image[((cycle + 1) / 40) as usize].push(char2);
        }

        register_x += val;
        cycle += add_cycles;
    });

    image
        .iter()
        .map(|row| row.concat() + "\n")
        .collect::<String>()
}
