pub enum Operator {
    Mul(u32, u32),
    Do,
    DoNot,
}

pub fn generator(input: &str) -> Vec<Operator> {
    let mut chars = input.chars().peekable();
    let mut operators = Vec::new();

    while let Some(c) = chars.next() {
        if c == 'm' {
            let mut lookahead = chars.clone();
            if lookahead.next() == Some('u')
                && lookahead.next() == Some('l')
                && lookahead.next() == Some('(')
            {
                // Consume the 'mul(' part
                chars.nth(2);

                // Parse the first number
                let mut n = String::new();
                while let Some(&next) = chars.peek() {
                    if next.is_ascii_digit() {
                        n.push(next);
                        chars.next();
                    } else {
                        break;
                    }
                }

                // Check for the comma
                if chars.next() == Some(',') {
                    // Parse the second number
                    let mut m = String::new();
                    while let Some(&next) = chars.peek() {
                        if next.is_ascii_digit() {
                            m.push(next);
                            chars.next();
                        } else {
                            break;
                        }
                    }

                    // Check for the closing parenthesis
                    if chars.next() == Some(')') {
                        if let (Ok(n_number), Ok(m_number)) = (n.parse::<u32>(), m.parse::<u32>()) {
                            operators.push(Operator::Mul(n_number, m_number))
                        }
                    }
                }
            }
        }
        if c == 'd' {
            let mut lookahead = chars.clone();
            let next_char = lookahead.next();
            let next_char_1 = lookahead.next();
            let next_char_2 = lookahead.next();
            if next_char == Some('o') {
                // Check for `do()`
                if next_char_1 == Some('(') && next_char_2 == Some(')') {
                    operators.push(Operator::Do);
                    chars.nth(2); // Consume `o()`
                }
                // Check for `don't()`
                else if next_char_1 == Some('n')
                    && next_char_2 == Some('\'')
                    && lookahead.next() == Some('t')
                    && lookahead.next() == Some('(')
                    && lookahead.next() == Some(')')
                {
                    operators.push(Operator::DoNot);
                    chars.nth(5);
                }
            }
        }
    }
    operators
}

pub fn part_1(input: &[Operator]) -> u32 {
    input
        .iter()
        .map(|op| match op {
            Operator::Mul(n, m) => n * m,
            _ => 0,
        })
        .sum()
}

pub fn part_2(input: &[Operator]) -> u32 {
    let mut do_mul = true;
    input
        .iter()
        .map(|op| match op {
            Operator::Mul(n, m) if do_mul => n * m,
            Operator::Do => {
                do_mul = true;
                0
            }
            Operator::DoNot => {
                do_mul = false;
                0
            }
            _ => 0,
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str =
        r#"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"#;
    const INPUT2: &str =
        r#"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"#;

    #[test]
    fn part_1_test() {
        let generator_output = generator(INPUT);
        let result = part_1(&generator_output);
        assert_eq!(result, 161);
    }

    #[test]
    fn part_2_test() {
        let generator_output = generator(INPUT2);
        let result = part_2(&generator_output);
        assert_eq!(result, 48);
    }
}
