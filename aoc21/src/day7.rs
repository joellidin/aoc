pub fn generator(input: &str) -> Vec<u32> {
    input.split(',').map(|n| n.parse().unwrap()).collect()
}

fn calculate_mean(positions: &[u32]) -> u32 {
    let sum: u32 = positions.iter().sum();
    let len = positions.len() as u32;
    sum / len
}

fn calculate_median(positions: &mut [u32]) -> u32 {
    positions.sort_unstable();
    let len = positions.len();

    if len == 0 {
        return 0; // Handle the case of an empty list
    }

    let median = if len % 2 == 1 {
        positions[len / 2] as f64
    } else {
        let mid1 = positions[len / 2 - 1] as f64;
        let mid2 = positions[len / 2] as f64;
        (mid1 + mid2) / 2.0
    };

    median.round() as u32
}

fn cost(positions: &[u32], pos: u32, increment: u32) -> u32 {
    positions
        .iter()
        .map(|&next_pos| {
            let diff = (pos as i32 - next_pos as i32).unsigned_abs();
            (0..diff).map(|i| i * increment + 1).sum::<u32>()
        })
        .sum()
}

pub fn part_1(input: &[u32]) -> u32 {
    let mut positions = input.to_vec();
    cost(input, calculate_median(&mut positions), 0)
}

pub fn part_2(input: &[u32]) -> u32 {
    let mean = calculate_mean(input);
    cost(input, mean, 1).min(cost(input, mean + 1, 1))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"16,1,2,0,4,2,7,1,2,14"#;

    #[test]
    fn part_1_test() {
        let generator_output = generator(INPUT);
        let result = part_1(&generator_output);
        assert_eq!(result, 37);
    }

    #[test]
    fn part_2_test() {
        let generator_output = generator(INPUT);
        let result = part_2(&generator_output);
        assert_eq!(result, 168);
    }
}
