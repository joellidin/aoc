pub fn generator(input: &str) -> Vec<(String, i32)> {
    input
        .lines()
        .map(|line| match line.split_once(' ') {
            Some((dir, num)) => {
                let num = num.parse::<i32>().expect("Second part must be a number.");
                (dir.to_owned(), num)
            }
            None => panic!("We need to be able to split every line."),
        })
        .collect()
}

pub fn part_1(input: &[(String, i32)]) -> i32 {
    let total_travelled = input.iter().fold((0, 0), |(total_h, total_d), (dir, num)| {
        match dir.as_str() {
            "forward" => (*num + total_h, total_d),
            "down" => (total_h, *num + total_d),
            "up" => (total_h, total_d - *num),
            _ => panic!("Not a valid direction."),
        }
    });
    total_travelled.0 * total_travelled.1
}

pub fn part_2(input: &[(String, i32)]) -> i32 {
    let total_travelled = input.iter().fold(
        (0, 0, 0),
        |(total_h, total_d, total_aim), (dir, num)| match dir.as_str() {
            "forward" => (*num + total_h, total_aim * *num + total_d, total_aim),
            "down" => (total_h, total_d, total_aim + *num),
            "up" => (total_h, total_d, total_aim - *num),
            _ => panic!("Not a valid direction."),
        },
    );
    total_travelled.0 * total_travelled.1
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"forward 5
down 5
forward 8
up 3
down 8
forward 2
"#;

    #[test]
    fn part_1_test() {
        let generator_output = generator(INPUT);
        let result = part_1(&generator_output);
        assert_eq!(result, 150);
    }

    #[test]
    fn part_2_test() {
        let generator_output = generator(INPUT);
        let result = part_2(&generator_output);
        assert_eq!(result, 900);
    }
}
