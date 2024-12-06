use std::collections::HashSet;

#[derive(Clone)]
pub struct Map {
    grid: Vec<Vec<u8>>,
    guard_pos: (usize, usize),
    guard_dir: Direction,
    visited: HashSet<(usize, usize)>,
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
}

pub fn generator(input: &str) -> Map {
    let mut guard_dir = Direction::North;
    let mut guard_pos = (0, 0);
    let mut visited = HashSet::new();
    let grid = input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            line.bytes()
                .enumerate()
                .map(|(j, c)| match c {
                    b'^' => {
                        guard_dir = Direction::North;
                        guard_pos = (j, i);
                        b'.'
                    }
                    b'v' => {
                        guard_dir = Direction::South;
                        guard_pos = (j, i);
                        b'.'
                    }
                    b'>' => {
                        guard_dir = Direction::East;
                        guard_pos = (j, i);
                        b'.'
                    }
                    b'<' => {
                        guard_dir = Direction::West;
                        guard_pos = (j, i);
                        b'.'
                    }
                    _ => c,
                })
                .collect()
        })
        .collect();
    visited.insert(guard_pos);
    Map {
        grid,
        guard_dir,
        guard_pos,
        visited,
    }
}

impl Map {
    fn walk_until_stop(&mut self) {
        while self.step() {}
    }

    fn step(&mut self) -> bool {
        let (new_x, new_y) = match self.guard_dir {
            Direction::North => (self.guard_pos.0, self.guard_pos.1.saturating_sub(1)),
            Direction::South => (self.guard_pos.0, self.guard_pos.1.saturating_add(1)),
            Direction::East => (self.guard_pos.0.saturating_add(1), self.guard_pos.1),
            Direction::West => (self.guard_pos.0.saturating_sub(1), self.guard_pos.1),
        };
        if new_x > self.grid[0].len() - 1
            || new_y > self.grid.len() - 1
            || self.guard_pos == (new_x, new_y)
        {
            return false;
        }

        match self.grid[new_y][new_x] {
            b'.' => {
                self.visited.insert((new_x, new_y));
            }
            b'#' => {
                self.rotate_right();
                return true;
            }
            _ => panic!("Found invalid character in grid"),
        }
        self.guard_pos = (new_x, new_y);
        true
    }

    fn is_loop(&mut self) -> bool {
        let mut visited_with_dir = HashSet::new();
        while !visited_with_dir.contains(&(self.guard_pos, self.guard_dir)) {
            visited_with_dir.insert((self.guard_pos, self.guard_dir));
            if !self.step() {
                return false;
            };
        }
        true
    }

    fn rotate_right(&mut self) {
        match self.guard_dir {
            Direction::North => self.guard_dir = Direction::East,
            Direction::South => self.guard_dir = Direction::West,
            Direction::East => self.guard_dir = Direction::South,
            Direction::West => self.guard_dir = Direction::North,
        };
    }
}

pub fn part_1(input: &Map) -> u32 {
    let mut map = input.clone();
    map.walk_until_stop();
    map.visited.len() as u32
}

pub fn part_2(input: &Map) -> u32 {
    let mut map = input.clone();
    let original_pos = map.guard_pos;
    let original_dir = map.guard_dir;
    let mut n_loops = 0;
    for i in 0..input.grid.len() {
        for j in 0..input.grid[0].len() {
            match map.grid[i][j] {
                b'.' => map.grid[i][j] = b'#',
                _ => continue,
            }
            if map.is_loop() {
                n_loops += 1;
            };
            map.grid[i][j] = b'.';
            map.guard_pos = original_pos;
            map.guard_dir = original_dir;
        }
    }
    n_loops
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
"#;

    #[test]
    fn part_1_test() {
        let generator_output = generator(INPUT);
        let result = part_1(&generator_output);
        assert_eq!(result, 41);
    }

    #[test]
    fn part_2_test() {
        let generator_output = generator(INPUT);
        let result = part_2(&generator_output);
        assert_eq!(result, 6);
    }
}
