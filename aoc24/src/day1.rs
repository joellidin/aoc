use std::collections::HashMap;

type NumberVectors = (Vec<u32>, Vec<u32>);
type ParseResult = Result<NumberVectors, Box<dyn std::error::Error>>;

pub fn generator(input: &str) -> ParseResult {
    let mut left_numbers = Vec::new();
    let mut right_numbers = Vec::new();

    for line in input.lines() {
        let mut numbers = line.split_whitespace();
        let left_str = numbers.next().ok_or("Missing left number")?;
        let right_str = numbers.next().ok_or("Missing right number")?;
        let left_num = left_str.parse()?;
        let right_num = right_str.parse()?;
        left_numbers.push(left_num);
        right_numbers.push(right_num);
    }

    Ok((left_numbers, right_numbers))
}

pub fn part_1(input: &ParseResult) -> u32 {
    let (ref mut left, ref mut right) = input
        .as_ref()
        .expect("Input must be parsable to two number vectors")
        .clone();
    left.sort_unstable();
    right.sort_unstable();

    left.iter()
        .zip(right)
        .map(|(l, r)| (*l as i32 - *r as i32).unsigned_abs())
        .sum()
}

pub fn part_2(input: &ParseResult) -> u32 {
    let (left, right) = input.as_ref().expect("Invalid input data");
    let mut count_map = HashMap::new();

    for &num in right {
        *count_map.entry(num).or_insert(0) += 1;
    }

    left.iter()
        .filter_map(|&num| count_map.get(&num).map(|&count| num * count))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"3   4
4   3
2   5
1   3
3   9
3   3
"#;

    #[test]
    fn part_1_test() {
        let generator_output = generator(INPUT);
        let result = part_1(&generator_output);
        assert_eq!(result, 11);
    }

    #[test]
    fn part_2_test() {
        let generator_output = generator(INPUT);
        let result = part_2(&generator_output);
        assert_eq!(result, 31);
    }
}
