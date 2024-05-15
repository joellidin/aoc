pub fn generator(input: &str) -> Vec<u32> {
    input
        .lines()
        .map(|l| l.parse::<u32>().expect("Line must be parsable to int."))
        .collect::<Vec<u32>>()
}

pub fn part_1(input: &[u32]) -> u32 {
    input.windows(2).filter(|v| v[0] < v[1]).count() as u32
}

pub fn part_2(input: &[u32]) -> u32 {
    input.windows(4).filter(|v| v[0] < v[3]).count() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"199
200
208
210
200
207
240
269
260
263
"#;

    #[test]
    fn part_1_test() {
        let generator_output = generator(INPUT);
        let result = part_1(&generator_output);
        assert_eq!(result, 7);
    }

    #[test]
    fn part_2_test() {
        let generator_output = generator(INPUT);
        let result = part_2(&generator_output);
        assert_eq!(result, 5);
    }
}
