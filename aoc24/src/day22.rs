use std::collections::{HashMap, HashSet};

use aoc_utils::prelude::*;

pub fn generator(input: &str) -> Vec<u64> {
    input
        .lines()
        .map(|line| {
            let &[x, ..] = extract_integers::<u64>(line).as_slice() else {
                panic!("Could not parse integers")
            };
            x
        })
        .collect()
}

fn apply_secret(num: u64) -> u64 {
    let mut secret = num;

    // multiply
    let mul = secret << 6;

    // Mix
    secret ^= mul;

    // Prune
    secret %= 16777216;

    // Step 2
    let div = secret >> 5;

    // Mix
    secret ^= div;

    // Prune
    secret %= 16777216;

    // Step 2
    let mul = secret << 11;

    // Mix
    secret ^= mul;

    // Prune
    secret %= 16777216;

    secret
}

pub fn part_1(input: &[u64]) -> u64 {
    input.iter().fold(0, |acc, n| {
        let mut secret = *n;
        for _ in 0..2000 {
            secret = apply_secret(secret);
        }
        acc + secret
    })
}

pub fn part_2(input: &[u64]) -> u64 {
    let mut sequences = HashSet::new();
    let mut bananas = HashMap::new();
    input.iter().for_each(|n| {
        let mut one = apply_secret(*n);
        let mut two = apply_secret(one);
        let mut three = apply_secret(two);
        let mut four = apply_secret(three);
        for _ in 4..2000 {
            let old_one = one;
            one = two;
            two = three;
            three = four;
            four = apply_secret(four);
            let first_diff = (four % 10) as i8 - (three % 10) as i8;
            let second_diff = (three % 10) as i8 - (two % 10) as i8;
            let third_diff = (two % 10) as i8 - (one % 10) as i8;
            let fourth_diff = (one % 10) as i8 - (old_one % 10) as i8;

            let key = (n, first_diff, second_diff, third_diff, fourth_diff);
            bananas.entry(key).or_insert_with(|| four % 10);
            sequences.insert((first_diff, second_diff, third_diff, fourth_diff));
        }
    });
    sequences
        .iter()
        .map(|g| {
            input
                .iter()
                .filter_map(|n| bananas.get(&(n, g.0, g.1, g.2, g.3)))
                .sum::<u64>()
        })
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_1: &str = r#"1
10
100
2024
"#;

    const INPUT_2: &str = r#"1
2
3
2024
"#;
    #[test]
    fn part_1_test() {
        let generator_output = generator(INPUT_1);
        let result = part_1(&generator_output);
        assert_eq!(result, 37327623);
    }

    #[test]
    fn part_2_test() {
        let generator_output = generator(INPUT_2);
        let result = part_2(&generator_output);
        assert_eq!(result, 23);
    }
}
