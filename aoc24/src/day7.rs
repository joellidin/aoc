use std::collections::HashSet;

pub fn generator(input: &str) -> Vec<(u64, Vec<u64>)> {
    input
        .lines()
        .map(|line| {
            let (val_str, operands_str) = line.split_once(": ").unwrap();
            let val = val_str.parse().unwrap();
            let operands = operands_str
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect();
            (val, operands)
        })
        .collect()
}

fn calculate_possible_values(numbers: &[u64], max_number: &u64, do_concat: bool) -> HashSet<u64> {
    fn backtrack(
        numbers: &[u64],
        max_number: &u64,
        index: usize,
        current_result: u64,
        results: &mut HashSet<u64>,
        do_concat: bool,
    ) {
        if &current_result > max_number {
            return;
        }
        if &current_result == max_number {
            results.insert(*max_number);
            return;
        }

        if index == numbers.len() {
            results.insert(current_result);
            return;
        }

        // Apply addition
        backtrack(
            numbers,
            max_number,
            index + 1,
            current_result + numbers[index],
            results,
            do_concat,
        );
        // Apply multiplication
        backtrack(
            numbers,
            max_number,
            index + 1,
            current_result * numbers[index],
            results,
            do_concat,
        );
        if do_concat {
            backtrack(
                numbers,
                max_number,
                index + 1,
                format!("{}{}", current_result, numbers[index])
                    .parse()
                    .unwrap(),
                results,
                do_concat,
            );
        }
    }

    let mut results = HashSet::new();

    if !numbers.is_empty() {
        backtrack(numbers, max_number, 1, numbers[0], &mut results, do_concat);
    }

    results
}
fn can_be_solved(val: &u64, operands: &[u64], do_concat: bool) -> bool {
    calculate_possible_values(operands, val, do_concat).contains(val)
}
pub fn part_1(input: &[(u64, Vec<u64>)]) -> u64 {
    input
        .iter()
        .filter(|(val, nums)| can_be_solved(val, nums, false))
        .map(|(val, _)| val)
        .sum()
}

pub fn part_2(input: &[(u64, Vec<u64>)]) -> u64 {
    input
        .iter()
        .filter(|(val, nums)| can_be_solved(val, nums, true))
        .map(|(val, _)| val)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
"#;

    #[test]
    fn part_1_test() {
        let generator_output = generator(INPUT);
        let result = part_1(&generator_output);
        assert_eq!(result, 3749);
    }

    #[test]
    fn part_2_test() {
        let generator_output = generator(INPUT);
        let result = part_2(&generator_output);
        assert_eq!(result, 11387);
    }
}
