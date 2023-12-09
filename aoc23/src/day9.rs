pub fn generator(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|l| l.split_whitespace().map(|n| n.parse().unwrap()).collect())
        .collect()
}

fn predict<F>(numbers: &[i32], reduce_logic: F) -> i32
where
    F: Fn(i32, &[i32]) -> i32,
{
    let mut pyramid = Vec::new();
    let mut current_row = numbers.to_vec();

    while !current_row.iter().all(|&x| x == 0) {
        pyramid.push(current_row.clone());
        current_row = current_row
            .windows(2)
            .map(|window| window[1] - window[0])
            .collect();
    }
    pyramid.iter().rev().fold(0, |acc, v| reduce_logic(acc, v))
}

pub fn part_1(input: &[Vec<i32>]) -> i32 {
    input
        .iter()
        .map(|v| predict(v, |acc, v| v.last().unwrap() + acc))
        .sum()
}

pub fn part_2(input: &[Vec<i32>]) -> i32 {
    input
        .iter()
        .map(|v| predict(v, |acc, v| v.first().unwrap() - acc))
        .sum()
}
