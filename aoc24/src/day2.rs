fn is_safe(level: &[u32]) -> bool {
    let is_increasing = level.windows(2).all(|w| w[0] < w[1]);
    let is_decreasing = level.windows(2).all(|w| w[0] > w[1]);

    // Check if differences between adjacent numbers are between 1 and 3
    let valid = level.windows(2).all(|w| {
        let diff = w[0].abs_diff(w[1]);
        (1..=3).contains(&diff)
    });
    valid && (is_increasing || is_decreasing)
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
    input.iter().filter(|level| is_safe(level)).count() as u32
}

pub fn part_2(input: &[Vec<u32>]) -> u32 {
    input
        .iter()
        .filter(|level| {
            if is_safe(level) {
                true
            } else {
                // Try removing each level
                (0..level.len()).any(|i| {
                    let mut new_line = level.to_owned().clone();
                    new_line.remove(i);
                    is_safe(&new_line)
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
