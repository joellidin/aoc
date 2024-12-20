use std::collections::VecDeque;

use aoc_utils::prelude::*;

pub const PART_1_SAVING: u32 = if cfg!(test) { 1 } else { 100 };
pub const PART_2_SAVING: u32 = if cfg!(test) { 50 } else { 100 };

type Map = Vec<Vec<char>>;
pub fn generator(input: &str) -> (Map, Vec2<i32>, Vec2<i32>) {
    let mut start = (0, 0).into();
    let mut end = (0, 0).into();
    let map = input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(|(j, c)| match c {
                    'S' => {
                        start = (j, i).into();
                        c
                    }
                    'E' => {
                        end = (j, i).into();
                        c
                    }
                    _ => c,
                })
                .collect()
        })
        .collect();
    (map, start, end)
}

fn flood_fill(map: &[Vec<char>], start: &Vec2<i32>, end: &Vec2<i32>) -> Option<Vec<Vec2<i32>>> {
    let mut q = VecDeque::new();
    q.push_back((0, *start));
    let mut lengths = Vec::new();
    while let Some((score, pos)) = q.pop_front() {
        if lengths.contains(&pos) {
            continue;
        }

        lengths.push(pos);

        if pos == *end {
            return Some(lengths);
        }

        for (di, dj) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let new_pos: Vec2<i32> = pos + (dj, di);
            if map[new_pos.i()][new_pos.j()] != '#' {
                q.push_back((score + 1, new_pos));
            }
        }
    }
    None
}

fn find_n_cheats(lengths: &[Vec2<i32>], cheat: u32, saving: u32) -> u32 {
    let mut n_cheats_tot = 0;
    for i in 0..lengths.len() {
        for j in 0..lengths.len() {
            if i == j {
                continue;
            }
            let start = lengths[i];
            let end = lengths[j];
            let manhattan_distance = start.x.abs_diff(end.x) + start.y.abs_diff(end.y);
            if manhattan_distance <= cheat
                && i.saturating_sub(j)
                    .saturating_sub(manhattan_distance as usize)
                    >= saving as usize
            {
                n_cheats_tot += 1;
            }
        }
    }
    n_cheats_tot
}

pub fn part_1((map, start, end): &(Map, Vec2<i32>, Vec2<i32>)) -> u32 {
    let lengths = flood_fill(map, start, end).expect("No valid paths");
    find_n_cheats(&lengths, 2, PART_1_SAVING)
}

pub fn part_2((map, start, end): &(Map, Vec2<i32>, Vec2<i32>)) -> u32 {
    let lengths = flood_fill(map, start, end).expect("No valid paths");
    find_n_cheats(&lengths, 20, PART_2_SAVING)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############
"#;

    #[test]
    fn part_1_test() {
        let generator_output = generator(INPUT);
        let result = part_1(&generator_output);
        assert_eq!(result, 44);
    }

    #[test]
    fn part_2_test() {
        let generator_output = generator(INPUT);
        let result = part_2(&generator_output);
        assert_eq!(result, 285);
    }
}
