use std::collections::{BinaryHeap, HashMap, HashSet};

use aoc_utils::prelude::*;

pub fn generator(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Eq, PartialEq, Clone)]
struct State {
    score: u32,
    pos: Vec2<i32>,
    dir: Direction,
    path: HashSet<Vec2<i32>>,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.score.cmp(&self.score)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn get_neighbours(
    map: &[Vec<char>],
    position: Vec2<i32>,
    direction: Direction,
) -> Vec<(Vec2<i32>, Direction)> {
    let mut neighbours = Vec::new();
    let (di, dj) = match direction {
        Direction::North => (-1, 0),
        Direction::East => (0, 1),
        Direction::South => (1, 0),
        Direction::West => (0, -1),
    };
    let new_pos = position + (dj, di);
    if map[new_pos.i()][new_pos.j()] != '#' {
        neighbours.push((new_pos, direction));
    }
    match direction {
        Direction::North => {
            neighbours.push((position, Direction::East));
            neighbours.push((position, Direction::West));
        }
        Direction::East => {
            neighbours.push((position, Direction::South));
            neighbours.push((position, Direction::North));
        }
        Direction::South => {
            neighbours.push((position, Direction::West));
            neighbours.push((position, Direction::East));
        }
        Direction::West => {
            neighbours.push((position, Direction::North));
            neighbours.push((position, Direction::South));
        }
    }
    neighbours
}

fn dijkstra(map: &[Vec<char>], start: Vec2<i32>) -> Option<(u32, HashSet<Vec2<i32>>)> {
    let mut heap = BinaryHeap::new();
    let mut best_positions = HashSet::new();
    let mut best_score = u32::MAX;

    // Map from (pos, dir) to the best known score to reach that state.
    let mut scores: HashMap<(Vec2<i32>, Direction), u32> = HashMap::new();

    let mut initial_path = HashSet::new();
    initial_path.insert(start);

    heap.push(State {
        score: 0,
        pos: start,
        dir: Direction::East,
        path: initial_path,
    });

    while let Some(State {
        score,
        pos,
        dir,
        path,
    }) = heap.pop()
    {
        if score > best_score {
            break;
        }

        let tile = map[pos.i()][pos.j()];

        if tile == 'E' {
            best_score = score;
            best_positions.extend(&path);
            continue;
        }

        scores.insert((pos, dir), score);

        for (new_pos, new_dir) in get_neighbours(map, pos, dir) {
            let cost = if new_dir != dir { 1000 } else { 1 };
            let new_score = score + cost;

            if let Some(&known_score) = scores.get(&(new_pos, new_dir)) {
                if known_score == new_score {
                    best_positions.extend(&path);
                }
                continue;
            }

            let mut new_path = path.clone();
            new_path.insert(new_pos);

            heap.push(State {
                score: new_score,
                pos: new_pos,
                dir: new_dir,
                path: new_path,
            });
        }
    }

    if best_score < u32::MAX {
        Some((best_score, best_positions))
    } else {
        None
    }
}

pub fn part_1(input: &[Vec<char>]) -> u32 {
    let start: Vec2<i32> = input
        .iter()
        .enumerate()
        .find_map(|(i, row)| row.iter().position(|&c| c == 'S').map(|j| (j, i)))
        .unwrap()
        .into();
    dijkstra(input, start).unwrap().0
}

pub fn part_2(input: &[Vec<char>]) -> u32 {
    let start: Vec2<i32> = input
        .iter()
        .enumerate()
        .find_map(|(i, row)| row.iter().position(|&c| c == 'S').map(|j| (j, i)))
        .unwrap()
        .into();
    dijkstra(input, start).unwrap().1.len() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_1: &str = r#"###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############
"#;

    const INPUT_2: &str = r#"#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################
"#;

    #[test]
    fn part_1_test() {
        let generator_output = generator(INPUT_1);
        let result = part_1(&generator_output);
        assert_eq!(result, 7036);

        let generator_output = generator(INPUT_2);
        let result = part_1(&generator_output);
        assert_eq!(result, 11048);
    }

    #[test]
    fn part_2_test() {
        let generator_output = generator(INPUT_1);
        let result = part_2(&generator_output);
        assert_eq!(result, 45);

        let generator_output = generator(INPUT_2);
        let result = part_2(&generator_output);
        assert_eq!(result, 64);
    }
}
