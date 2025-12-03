pub fn generator(input: &str) -> Vec<(u64, u64)> {
    input
        .split(',')
        .map(|nums| {
            let (low, high) = nums.split_once('-').expect("Wrong format");
            (
                low.parse().expect("Must be parsable to a number"),
                high.parse().expect("Must be parsable to a number"),
            )
        })
        .collect()
}

pub fn part_1(input: &[(u64, u64)]) -> u64 {
    let mut res = 0;
    input.iter().for_each(|&(low, high)| {
        for num in low..=high {
            let num_str = num.to_string();
            if num_str[..num_str.len() / 2] == num_str[num_str.len() / 2..] {
                res += num;
            }
        }
    });
    res
}

pub fn part_2(input: &[(u64, u64)]) -> u64 {
    let mut res = 0;
    input.iter().for_each(|&(low, high)| {
        for num in low..=high {
            let num_str = num.to_string();
            for chunk_size in 1..=num_str.len() / 2 {
                if num_str.len() % chunk_size == 0 {
                    let parts: Vec<&str> = (0..num_str.len())
                        .step_by(chunk_size)
                        .map(|i| &num_str[i..i + chunk_size])
                        .collect();
                    if parts.iter().all(|num| *num == parts[0]) {
                        res += num;
                        break;
                    }
                }
            }
        }
    });
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"#;

    #[test]
    fn part_1_test() {
        let generator_output = generator(INPUT);
        let result = part_1(&generator_output);
        assert_eq!(result, 1227775554);
    }

    #[test]
    fn part_2_test() {
        let generator_output = generator(INPUT);
        let result = part_2(&generator_output);
        assert_eq!(result, 4174379265);
    }
}
