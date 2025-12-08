use std::collections::{HashMap, HashSet};

pub enum Tile {
    Empty,
    Splitter,
    Start(usize, usize),
}

pub fn generator(input: &str) -> Vec<Vec<Tile>> {
    input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(|(j, ch)| match ch {
                    'S' => Tile::Start(i, j),
                    '.' => Tile::Empty,
                    '^' => Tile::Splitter,
                    _ => panic!("Invalid input"),
                })
                .collect()
        })
        .collect()
}

pub fn part_1(input: &[Vec<Tile>]) -> u32 {
    let Some(Tile::Start(i, j)) = input
        .iter()
        .flatten()
        .find(|tile| matches!(tile, Tile::Start(..)))
    else {
        panic!("No start tile found");
    };

    let mut splits = 0;
    let mut visited = HashSet::new();
    let mut queue = vec![(*i, *j)];

    while let Some((i, j)) = queue.pop() {
        let new_i = i + 1;
        if new_i < input.len() {
            match input[new_i][j] {
                Tile::Empty | Tile::Start(_, _) => {
                    if visited.insert((new_i, j)) {
                        queue.push((new_i, j));
                    }
                }
                Tile::Splitter => {
                    if visited.insert((new_i, j)) {
                        splits += 1;
                        for dj in [-1_isize, 1] {
                            let new_j = j as isize + dj;
                            if new_j >= 0 && (new_j as usize) < input[0].len() {
                                queue.push((new_i, new_j as usize));
                            }
                        }
                    }
                }
            }
        }
    }
    splits
}

pub fn part_2(input: &[Vec<Tile>]) -> u64 {
    let Some(Tile::Start(start_i, start_j)) = input
        .iter()
        .flatten()
        .find(|tile| matches!(tile, Tile::Start(..)))
    else {
        panic!("No start tile found");
    };

    let width = input[0].len();
    let height = input.len();

    // column -> number of timelines at that column
    let mut timelines: HashMap<usize, u64> = HashMap::new();
    timelines.insert(*start_j, 1);

    let mut terminated = 0u64;

    for row in *start_i..height - 1 {
        let mut next_timelines: HashMap<usize, u64> = HashMap::new();

        for (col, count) in timelines.drain() {
            match input[row + 1][col] {
                Tile::Empty | Tile::Start(_, _) => {
                    *next_timelines.entry(col).or_default() += count;
                }
                Tile::Splitter => {
                    for dj in [-1_isize, 1] {
                        let new_col = col as isize + dj;
                        if new_col >= 0 && (new_col as usize) < width {
                            *next_timelines.entry(new_col as usize).or_default() += count;
                        } else {
                            terminated += count;
                        }
                    }
                }
            }
        }
        timelines = next_timelines;
    }

    terminated + timelines.values().sum::<u64>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
"#;

    #[test]
    fn part_1_test() {
        let generator_output = generator(INPUT);
        let result = part_1(&generator_output);
        assert_eq!(result, 21);
    }

    #[test]
    fn part_2_test() {
        let generator_output = generator(INPUT);
        let result = part_2(&generator_output);
        assert_eq!(result, 40);
    }
}
