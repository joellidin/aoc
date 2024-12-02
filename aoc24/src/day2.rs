fn is_safe_iter<'a, I>(mut iter: I) -> bool
where
    I: Iterator<Item = &'a u32>,
{
    let first = match iter.next() {
        Some(val) => val,
        None => return false, // Empty sequence is not safe
    };
    let second = match iter.next() {
        Some(val) => val,
        None => return true, // Single-element sequence is safe
    };
    let diff = *second as i32 - *first as i32;

    // Check initial conditions
    if diff == 0 || diff.abs() > 3 {
        return false;
    }
    let is_increasing = diff > 0;

    let mut prev = second;
    for curr in iter {
        let diff = *curr as i32 - *prev as i32;
        if diff == 0 || diff.abs() > 3 || (diff > 0) != is_increasing {
            return false;
        }
        prev = curr;
    }
    true
}

pub fn generator(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|level| {
            level
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect()
        })
        .collect()
}

pub fn part_1(input: &[Vec<u32>]) -> u32 {
    input
        .iter()
        .filter(|level| is_safe_iter(level.iter()))
        .count() as u32
}

pub fn part_2(input: &[Vec<u32>]) -> u32 {
    input
        .iter()
        .filter(|level| {
            if is_safe_iter(level.iter()) {
                true
            } else {
                (0..level.len()).any(|i| {
                    let iter =
                        level
                            .iter()
                            .enumerate()
                            .filter_map(|(j, val)| if j != i { Some(val) } else { None });
                    is_safe_iter(iter)
                })
            }
        })
        .count() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
"#;

    #[test]
    fn part_1_test() {
        let generator_output = generator(INPUT);
        let result = part_1(&generator_output);
        assert_eq!(result, 2);
    }

    #[test]
    fn part_2_test() {
        let generator_output = generator(INPUT);
        let result = part_2(&generator_output);
        assert_eq!(result, 4);
    }
}
