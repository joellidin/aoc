pub fn generator(input: &str) -> Vec<(u64, Vec<u64>)> {
    input
        .lines()
        .map(|line| {
            let (val_str, operands_str) = line.split_once(": ").expect("Invalid input");
            let val = val_str
                .parse()
                .expect("Target must be parsable to a number");
            let operands = operands_str
                .split_whitespace()
                .map(|n| n.parse().expect("Operands must be parsable to a number"))
                .collect();
            (val, operands)
        })
        .collect()
}

fn digit_count(mut n: u64) -> u32 {
    if n == 0 {
        return 1;
    }
    let mut count = 0;
    while n > 0 {
        n /= 10;
        count += 1;
    }
    count
}

fn concat_numbers(a: u64, b: u64) -> Option<u64> {
    let digits = digit_count(b);
    a.checked_mul(10_u64.pow(digits))
        .and_then(|val| val.checked_add(b))
}

fn is_possible_calibration(
    numbers: &[u64],
    target: u64,
    index: usize,
    current_result: u64,
    do_concat: bool,
) -> bool {
    // Pruning
    if current_result > target {
        return false;
    }

    // If we've reached the end and current_result is the target, return true
    if index == numbers.len() {
        return current_result == target;
    }

    let next_num = numbers[index];

    // Try addition
    if is_possible_calibration(
        numbers,
        target,
        index + 1,
        current_result + next_num,
        do_concat,
    ) {
        return true;
    }

    // Try multiplication
    if is_possible_calibration(
        numbers,
        target,
        index + 1,
        current_result * next_num,
        do_concat,
    ) {
        return true;
    }

    // Try concatenation
    if do_concat {
        if let Some(concat_res) = concat_numbers(current_result, next_num) {
            if concat_res <= target
                && is_possible_calibration(numbers, target, index + 1, concat_res, do_concat)
            {
                return true;
            }
        }
    }

    // If none of the branches worked out, return false
    false
}

pub fn part_1(input: &[(u64, Vec<u64>)]) -> u64 {
    input
        .iter()
        .filter(|(val, nums)| is_possible_calibration(nums, *val, 1, nums[0], false))
        .map(|(val, _)| val)
        .sum()
}

pub fn part_2(input: &[(u64, Vec<u64>)]) -> u64 {
    input
        .iter()
        .filter(|(val, nums)| is_possible_calibration(nums, *val, 1, nums[0], true))
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
