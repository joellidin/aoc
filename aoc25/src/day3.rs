pub fn generator(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| char.to_digit(10).expect("Must be parsable to a number"))
                .collect()
        })
        .collect()
}

fn find_largest_n_digit(numbers: &[u32], n: usize) -> u64 {
    if n == 0 || numbers.len() < n {
        return 0;
    }
    if n == 1 {
        return *numbers.iter().max().unwrap() as u64;
    }

    // We need to leave (n - 1) digits after our choice
    let end = numbers.len() - (n - 1);

    // Find FIRST max in numbers[0..end] (first to leave more room for future choices)
    let max_val = *numbers[..end].iter().max().unwrap();
    let max_idx = numbers[..end].iter().position(|&n| n == max_val).unwrap();

    // Recurse on the remaining slice after max_idx
    max_val as u64 * 10u64.pow((n - 1) as u32)
        + find_largest_n_digit(&numbers[max_idx + 1..], n - 1)
}

pub fn part_1(input: &[Vec<u32>]) -> u64 {
    input
        .iter()
        .map(|numbers| find_largest_n_digit(numbers, 2))
        .sum()
}

pub fn part_2(input: &[Vec<u32>]) -> u64 {
    input
        .iter()
        .map(|numbers| find_largest_n_digit(numbers, 12))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"987654321111111
811111111111119
234234234234278
818181911112111
"#;

    #[test]
    fn part_1_test() {
        let generator_output = generator(INPUT);
        let result = part_1(&generator_output);
        assert_eq!(result, 357);
    }

    #[test]
    fn part_2_test() {
        let generator_output = generator(INPUT);
        let result = part_2(&generator_output);
        assert_eq!(result, 3121910778619);
    }
}
