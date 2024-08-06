pub fn generator(input: &str) -> Vec<u8> {
    input.split(',').map(|n| n.parse::<u8>().unwrap()).collect()
}

fn simulate_steps(steps: u64, lanternfish: &[u8]) -> u64 {
    // Initialize an array to hold the count of lanternfish with timers from 0 to 8
    let mut timer_counts = [0u64; 9];

    // Populate the initial state based on the input array
    for &timer in lanternfish {
        timer_counts[timer as usize] += 1;
    }

    // Simulate each day
    for _ in 0..steps {
        // Number of fish that will create new fish
        let new_fish = timer_counts[0];

        // Shift all the fish timer counts down by one
        for i in 0..8 {
            timer_counts[i] = timer_counts[i + 1];
        }

        // Reset fish with timer 0 to timer 6
        timer_counts[6] += new_fish;

        // New fish go to timer 8
        timer_counts[8] = new_fish;
    }

    // Sum up all the fish to get the total count
    timer_counts.iter().sum()
}

pub fn part_1(input: &[u8]) -> u64 {
    let lanternfish = input.to_vec();
    simulate_steps(80, &lanternfish)
}

pub fn part_2(input: &[u8]) -> u64 {
    let lanternfish = input.to_vec();
    simulate_steps(256, &lanternfish)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"3,4,3,1,2"#;

    #[test]
    fn part_1_test() {
        let generator_output = generator(INPUT);
        let result = part_1(&generator_output);
        assert_eq!(result, 5934);
    }

    #[test]
    fn part_2_test() {
        let generator_output = generator(INPUT);
        let result = part_2(&generator_output);
        assert_eq!(result, 26984457539);
    }
}
