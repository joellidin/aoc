pub fn generator(input: &str) -> Vec<&str> {
    input.lines().collect()
}

pub fn part_1(input: &[&str]) -> u32 {
    let ones = input.iter().fold(vec![0; input[0].len()], |mut acc, num| {
        num.chars().enumerate().for_each(|(i, d)| {
            if d == '1' {
                acc[i] += 1
            }
        });
        acc
    });
    let n_lines = input.len();
    let gamma_rate = ones
        .iter()
        .rev()
        .enumerate()
        .fold(0u32, |acc, (i, num_ones)| {
            if *num_ones as usize > n_lines / 2 {
                acc + 2u32.pow(i as u32)
            } else {
                acc
            }
        });
    let num_bits = 32 - gamma_rate.leading_zeros();
    let mask = (1 << num_bits) - 1;
    let epsilon_rate = !gamma_rate & mask;
    gamma_rate * epsilon_rate
}

pub fn part_2(input: &[&str]) -> u32 {
    let mut n_ones = 0;
    let mut one_indices = Vec::with_capacity(input.len());
    let mut zero_indices = Vec::with_capacity(input.len());
    let mut valid_indices = (0..input.len()).collect::<Vec<_>>();
    let oxygen_generator_rating = 'outer: {
        for j in 0..input[0].len() {
            for i in 0..input.len() {
                if valid_indices.len() == 1 {
                    break 'outer u32::from_str_radix(input[valid_indices[0]], 2)
                        .expect("The number must be in binary format.");
                }
                if valid_indices.contains(&i) && input[i].get(j..j + 1) == Some("1") {
                    n_ones += 1;
                    one_indices.push(i);
                } else if valid_indices.contains(&i) {
                    zero_indices.push(i);
                }
            }
            if n_ones as f32 >= valid_indices.len() as f32 / 2.0 {
                valid_indices.clone_from(&one_indices);
            } else {
                valid_indices.clone_from(&zero_indices);
            }
            one_indices.clear();
            zero_indices.clear();
            n_ones = 0;
        }
        u32::from_str_radix(input[valid_indices[0]], 2)
            .expect("The number must be in binary format.")
    };

    let mut valid_indices = (0..input.len()).collect::<Vec<_>>();
    let co2_scrubber_rating = 'outer: {
        for j in 0..input[0].len() {
            for i in 0..input.len() {
                if valid_indices.len() == 1 {
                    break 'outer u32::from_str_radix(input[valid_indices[0]], 2)
                        .expect("The number must be in binary format.");
                } else if !valid_indices.contains(&i) {
                    continue;
                } else if input[i].get(j..j + 1) == Some("1") {
                    n_ones += 1;
                    one_indices.push(i);
                } else {
                    zero_indices.push(i);
                }
            }
            if (n_ones as f32) < valid_indices.len() as f32 / 2.0 {
                valid_indices.clone_from(&one_indices);
            } else {
                valid_indices.clone_from(&zero_indices);
            }
            one_indices.clear();
            zero_indices.clear();
            n_ones = 0;
        }
        u32::from_str_radix(input[valid_indices[0]], 2)
            .expect("The number must be in binary format.")
    };
    co2_scrubber_rating * oxygen_generator_rating
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010
"#;

    #[test]
    fn part_1_test() {
        let generator_output = generator(INPUT);
        let result = part_1(&generator_output);
        assert_eq!(result, 198);
    }

    #[test]
    fn part_2_test() {
        let generator_output = generator(INPUT);
        let result = part_2(&generator_output);
        assert_eq!(result, 230);
    }
}
