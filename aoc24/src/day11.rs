use std::collections::HashMap;

pub fn generator(input: &str) -> Vec<u64> {
    input
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect()
}

fn number_length(n: &mut u64) -> u64 {
    if *n == 0 {
        return 1;
    }
    let mut count = 0;
    while *n > 0 {
        *n /= 10;
        count += 1;
    }
    count
}

fn split_number(n: &u64, n_len: &u64) -> (u64, u64) {
    let base = 10_i64.pow(*n_len as u32 / 2);
    let left = *n as i64 / base;
    let right = *n as i64 % base;
    (left as u64, right as u64)
}

pub fn blink(stones: &mut HashMap<u64, u64>) {
    let mut new_stones = HashMap::new();
    stones.iter().for_each(|(&stone, val)| {
        match stone {
            0 => {
                *new_stones.entry(1).or_default() += val;
            }
            _ => {
                let stone_len = number_length(&mut stone.clone());
                if stone_len % 2 == 0 {
                    let (left, right) = split_number(&stone, &stone_len);
                    *new_stones.entry(left).or_default() += val;
                    *new_stones.entry(right).or_default() += val;
                } else {
                    *new_stones.entry(2024 * stone).or_default() += val;
                }
            }
        };
    });
    *stones = new_stones;
}

pub fn part_1(input: &[u64]) -> u64 {
    let mut stones = input.iter().map(|&n| (n, 1)).collect::<HashMap<u64, u64>>();
    for _ in 0..25 {
        blink(&mut stones);
    }
    stones.values().sum()
}

pub fn part_2(input: &[u64]) -> u64 {
    let mut stones = input.iter().map(|&n| (n, 1)).collect::<HashMap<u64, u64>>();
    for _ in 0..75 {
        blink(&mut stones);
    }
    stones.values().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"125 17
"#;

    #[test]
    fn part_1_test() {
        let generator_output = generator(INPUT);
        let result = part_1(&generator_output);
        assert_eq!(result, 55312);
    }

    #[test]
    fn part_2_test() {
        let generator_output = generator(INPUT);
        let result = part_2(&generator_output);
        assert_eq!(result, 65601038650482);
    }
}
