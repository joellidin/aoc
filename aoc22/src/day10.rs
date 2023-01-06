pub fn solution() {
    let mut image: Vec<Vec<char>> = vec![Vec::new(); 6];
    let res = include_str!("../data/day10.txt")
        .trim()
        .lines()
        .fold((1isize, 0, 0), |(register_x, signal_sum, cycle), line| {
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
                let char = if (register_x..=register_x + 2).contains(&((cycle + 1) % 40)) {
                    '#'
                } else {
                    '.'
                };
                image[(cycle / 40) as usize].push(char);
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
                let char1 = if (register_x..=register_x + 2).contains(&((cycle + 1) % 40)) {
                    '#'
                } else {
                    '.'
                };
                let char2 = if (register_x..=register_x + 2).contains(&((cycle + 2) % 40)) {
                    '#'
                } else {
                    '.'
                };
                image[(cycle / 40) as usize].push(char1);
                image[((cycle + 1) / 40) as usize].push(char2);
            }

            (
                register_x + val,
                signal_sum + signal_strength,
                cycle + add_cycles,
            )
        });

    println!("The sum of the signal strengths is {}", res.1);
    println!(
        "CRT image:\n{}",
        image
            .iter()
            .map(|row| row.iter().collect::<String>() + "\n")
            .collect::<String>()
    );
}
