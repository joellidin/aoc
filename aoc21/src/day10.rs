pub fn generator(input: &str) -> Vec<&str> {
    input.lines().collect()
}

fn matching_closing_bracket(open: char) -> char {
    match open {
        '(' => ')',
        '{' => '}',
        '[' => ']',
        '<' => '>',
        _ => panic!("Invalid opening bracket"),
    }
}

fn syntax_error_score(c: char) -> u32 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0,
    }
}

fn completion_cost(c: char) -> u64 {
    match c {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => 0,
    }
}

pub fn part_1(input: &[&str]) -> u32 {
    input.iter().fold(0, |acc, &l| {
        // Find invalid subsystems and calculate syntax error score
        let mut stack = vec![];
        for c in l.chars() {
            match c {
                '(' | '{' | '[' | '<' => stack.push(c),
                _ => {
                    if let Some(open_char) = stack.pop() {
                        let expected_close = matching_closing_bracket(open_char);
                        if c != expected_close {
                            return acc + syntax_error_score(c);
                        }
                    }
                }
            }
        }
        acc
    })
}

pub fn part_2(input: &[&str]) -> u64 {
    let mut points = input
        .iter()
        // Find the valid stacks
        .filter_map(|l| {
            let mut stack = vec![];
            for c in l.chars() {
                match c {
                    '(' | '{' | '[' | '<' => stack.push(c),
                    _ => {
                        if let Some(open_char) = stack.pop() {
                            let expected_close = matching_closing_bracket(open_char);
                            if c != expected_close {
                                return None;
                            }
                        }
                    }
                }
            }
            Some(stack)
        })
        // Calculate completion cost for all navigation subsystems
        .map(|stack| {
            stack.iter().rev().fold(0, |acc, &c| {
                let close_char = matching_closing_bracket(c);
                acc * 5 + completion_cost(close_char)
            })
        })
        .collect::<Vec<_>>();
    // Find the middle score
    points.sort_unstable();
    *points
        .get(points.len() / 2)
        .expect("There must be a odd number of scores.")
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]
"#;

    #[test]
    fn part_1_test() {
        let generator_output = generator(INPUT);
        let result = part_1(&generator_output);
        assert_eq!(result, 26397);
    }

    #[test]
    fn part_2_test() {
        let generator_output = generator(INPUT);
        let result = part_2(&generator_output);
        assert_eq!(result, 288957);
    }
}
