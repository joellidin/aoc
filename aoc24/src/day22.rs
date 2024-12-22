use std::collections::{HashMap, HashSet};

pub fn generator(input: &str) -> Vec<u64> {
    input
        .lines()
        .map(|line| line.parse().expect("Input musy be parsable to a number"))
        .collect()
}

fn apply_secret(num: u64) -> u64 {
    let mut secret = num ^ ((num << 6) & 0xFFFFFF);
    secret ^= (secret >> 5) & 0xFFFFFF;
    secret ^= (secret << 11) & 0xFFFFFF;
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
    *input
        .iter()
        .fold(HashMap::<(i8, i8, i8, i8), u64>::new(), |mut acc, &n| {
            let mut bananas = vec![(n % 10) as i8];
            let mut secret = n;
            for _ in 0..2000 {
                secret = apply_secret(secret);
                bananas.push((secret % 10) as i8);
            }
            let mut seen = HashSet::new();
            bananas.windows(5).for_each(|s| {
                let d1 = s[4] - s[3];
                let d2 = s[3] - s[2];
                let d3 = s[2] - s[1];
                let d4 = s[1] - s[0];
                if seen.insert((d1, d2, d3, d4)) {
                    *acc.entry((d1, d2, d3, d4)).or_default() += s[4] as u64;
                }
            });
            acc
        })
        .values()
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
