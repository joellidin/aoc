use aoc_utils::prelude::*;

pub fn generator(input: &str) -> (Vec<Vec<u64>>, Vec<&str>) {
    let operators = input
        .lines()
        .last()
        .map(|line| line.split_whitespace().collect())
        .unwrap_or_default();

    let numbers = input
        .lines()
        .rev()
        .skip(1)
        .map(extract_integers::<u64>)
        .collect();

    (numbers, operators)
}

pub fn part_1(input: &str) -> u64 {
    let (numbers, operators) = generator(input);
    let mut results = vec![0; operators.len()];
    operators.iter().enumerate().for_each(|(i, operator)| {
        if *operator == "*" {
            results[i] = 1
        }
    });

    numbers.iter().for_each(|nums| {
        for i in 0..operators.len() {
            match operators[i] {
                "+" => results[i] += nums[i],
                "*" => results[i] *= nums[i],
                _ => panic!("Invalid operator"),
            }
        }
    });
    results.into_iter().sum()
}

pub fn part_2(input: &str) -> u64 {
    let matrix: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let Some(first_row) = matrix.first() else {
        return 0;
    };

    let mut results: Vec<u64> = Vec::new();
    let mut column_nums = Vec::new();

    for j in (0..first_row.len()).rev() {
        let mut num = 0;
        let mut is_number_column = true;

        for row in &matrix {
            match row[j] {
                c @ '0'..='9' => {
                    num = num * 10 + c.to_digit(10).expect("Not parsable to digit") as u64;
                }
                op @ ('+' | '*') => {
                    column_nums.push(num);
                    let result = if op == '+' {
                        column_nums.iter().sum()
                    } else {
                        column_nums.iter().product()
                    };
                    results.push(result);
                    column_nums.clear();
                    is_number_column = false;
                }
                _ => {}
            }
        }

        if is_number_column && num > 0 {
            column_nums.push(num);
        }
    }

    results.into_iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  
"#;

    #[test]
    fn part_1_test() {
        let result = part_1(INPUT);
        assert_eq!(result, 4277556);
    }

    #[test]
    fn part_2_test() {
        let result = part_2(INPUT);
        assert_eq!(result, 3263827);
    }
}
