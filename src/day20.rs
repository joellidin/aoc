use std::collections::VecDeque;

fn mix(data: &[isize], positions: &mut VecDeque<usize>) {
    for (idx, val) in data.iter().enumerate() {
        if *val == 0 {
            continue;
        }
        let pos = positions.iter().position(|x| *x == idx).unwrap();
        positions.remove(pos).unwrap();
        if *val < 0 {
            let val = val.unsigned_abs() % (data.len() - 1);
            positions.rotate_right(val);
            positions.insert(pos, idx);
        } else {
            let val = val.unsigned_abs() % (data.len() - 1);
            positions.rotate_left(val);
            positions.insert(pos, idx);
        }
    }
}

fn calculate_coordinates(data: &[isize], positions: &VecDeque<usize>) -> isize {
    let pos = positions
        .iter()
        .position(|i| *i == data.iter().position(|x| *x == 0).unwrap())
        .unwrap();
    (1..4).fold(0, |acc, x| {
        acc + data[positions[(pos + x * 1000) % data.len()]]
    })
}

pub fn solution() {
    let mut data = std::fs::read_to_string("data/day20.txt")
        .unwrap()
        .trim()
        .split('\n')
        .map(|num| num.parse::<isize>().unwrap())
        .collect::<Vec<_>>();
    let mut positions = (0..data.len()).collect::<VecDeque<_>>();
    mix(&data, &mut positions);
    println!(
        "Sum of the three coordinates: {}",
        calculate_coordinates(&data, &positions)
    );

    data = data.iter().map(|x| 811589153 * x).collect();
    let mut positions = (0..data.len()).collect::<VecDeque<_>>();
    (0..10).for_each(|_| mix(&data, &mut positions));
    println!(
        "Sum of the three coordinates with encryption key: {}",
        calculate_coordinates(&data, &positions)
    );
}
