use std::{
    cmp,
    collections::{BinaryHeap, HashSet},
};

use aoc_utils::prelude::*;

pub const WIDTH: i32 = if cfg!(test) { 6 } else { 70 };
pub const HEIGHT: i32 = if cfg!(test) { 6 } else { 70 };
pub const PART_1_MEMORY_SPACE: usize = if cfg!(test) { 12 } else { 1024 };

pub fn generator(input: &str) -> Vec<Vec2<i32>> {
    input
        .lines()
        .map(|line| {
            let &[x, y, ..] = extract_integers::<i32>(line).as_slice() else {
                panic!("Could not parse integers")
            };
            (x, y).into()
        })
        .collect()
}

fn dijkstra(map: &HashSet<Vec2<i32>>, end: &Vec2<i32>) -> Option<u32> {
    let mut heap = BinaryHeap::new();
    let mut visited = HashSet::new();
    let start: Vec2<i32> = (0, 0).into();

    heap.push(cmp::Reverse((0, start)));

    while let Some(cmp::Reverse((score, pos))) = heap.pop() {
        if visited.contains(&pos) {
            continue;
        }

        if pos == *end {
            return Some(score);
        }

        visited.insert(pos);

        for new_pos in get_neighbours(map, pos) {
            heap.push(cmp::Reverse((score + 1, new_pos)));
        }
    }

    None
}

fn get_neighbours(
    map: &HashSet<Vec2<i32>>,
    pos: Vec2<i32>,
) -> impl Iterator<Item = Vec2<i32>> + use<'_> {
    [(-1, 0), (1, 0), (0, -1), (0, 1)]
        .into_iter()
        .filter_map(move |(di, dj)| {
            let new_pos = pos + (dj, di);
            if new_pos.x > WIDTH
                || new_pos.x < 0
                || new_pos.y > HEIGHT
                || new_pos.y < 0
                || map.contains(&new_pos)
            {
                return None;
            }
            Some(new_pos)
        })
}

pub fn part_1(input: &[Vec2<i32>]) -> u32 {
    let corrupted_bytes: HashSet<_> = input.iter().take(PART_1_MEMORY_SPACE).copied().collect();

    let end: Vec2<i32> = (HEIGHT, WIDTH).into();
    dijkstra(&corrupted_bytes, &end).unwrap()
}

pub fn part_2(input: &[Vec2<i32>]) -> String {
    let mut corrupted_bytes: HashSet<_> = input.iter().take(PART_1_MEMORY_SPACE).cloned().collect();
    let end: Vec2<i32> = (HEIGHT, WIDTH).into();
    if let Some(Vec2 { x, y }) = input.iter().skip(PART_1_MEMORY_SPACE).find(|&&byte| {
        corrupted_bytes.insert(byte);
        let res = dijkstra(&corrupted_bytes, &end);
        res.is_none()
    }) {
        format!("{x},{y}")
    } else {
        panic!("Could not find a solution")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0
"#;

    #[test]
    fn part_1_test() {
        let generator_output = generator(INPUT);
        let result = part_1(&generator_output);
        assert_eq!(result, 22);
    }

    #[test]
    fn part_2_test() {
        let generator_output = generator(INPUT);
        let result = part_2(&generator_output);
        assert_eq!(result, "6,1");
    }
}
