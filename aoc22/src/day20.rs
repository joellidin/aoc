use std::collections::VecDeque;

fn mix(data: &[i64], positions: &mut VecDeque<usize>) {
    for (idx, val) in data.iter().enumerate() {
        if *val == 0 {
            continue;
        }
        let pos = positions.iter().position(|x| *x == idx).unwrap();
        positions.remove(pos).unwrap();
        if *val < 0 {
            let val = val.unsigned_abs() as usize % (data.len() - 1);
            positions.rotate_right(val);
            positions.insert(pos, idx);
        } else {
            let val = val.unsigned_abs() as usize % (data.len() - 1);
            positions.rotate_left(val);
            positions.insert(pos, idx);
        }
    }
}

fn calculate_coordinates(data: &[i64], positions: &VecDeque<usize>) -> i64 {
    let pos = positions
        .iter()
        .position(|i| *i == data.iter().position(|x| *x == 0).unwrap())
        .unwrap();
    (1..4).fold(0, |acc, x| {
        acc + data[positions[(pos + x * 1000) % data.len()]]
    })
}

pub fn generator(input: &str) -> Vec<i64> {
    input
        .trim()
        .split('\n')
        .map(|num| num.parse::<i64>().unwrap())
        .collect()
}

pub fn part_1(input: &[i64]) -> i64 {
    let mut positions = (0..input.len()).collect();
    mix(input, &mut positions);
    calculate_coordinates(input, &positions)
}

pub fn part_2(input: &[i64]) -> i64 {
    let data = input.iter().map(|x| 811589153 * x).collect::<Vec<i64>>();
    let mut positions = (0..data.len()).collect();
    (0..10).for_each(|_| mix(&data, &mut positions));
    calculate_coordinates(&data, &positions)
}
