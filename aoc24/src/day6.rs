#[derive(Clone)]
pub struct Map {
    grid: Vec<Vec<u8>>,
    guard_pos: (usize, usize),
    guard_dir: Direction,
    visited: Vec<Vec<bool>>,
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
}

enum StepResult {
    OutOfBounds,
    Turn,
    Step,
}

pub fn generator(input: &str) -> Map {
    let mut guard_dir = Direction::North;
    let mut guard_pos = (0, 0);
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
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
    visited[guard_pos.1][guard_pos.0] = true;
    Map {
        grid,
        guard_dir,
        guard_pos,
        visited,
    }
}

impl Map {
    fn walk_until_stop(&mut self) {
        while matches!(self.step(), StepResult::Step | StepResult::Turn) {}
    }

    fn step(&mut self) -> StepResult {
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
            return StepResult::OutOfBounds;
        }

        match self.grid[new_y][new_x] {
            b'.' => {
                self.visited[new_y][new_x] = true;
            }
            b'#' => {
                self.rotate_right();
                return StepResult::Turn;
            }
            _ => panic!("Found invalid character in grid"),
        }
        self.guard_pos = (new_x, new_y);
        StepResult::Step
    }

    fn is_loop(&mut self) -> bool {
        let mut tortoise = self.clone();
        let mut hare = self.clone();

        loop {
            // Tortoise takes one step
            if let StepResult::OutOfBounds = tortoise.step() {
                return false;
            }

            // Hare takes two steps
            for _ in 0..2 {
                if let StepResult::OutOfBounds = hare.step() {
                    return false;
                }
            }

            // Check if the tortoise and hare are in the same state
            if tortoise.guard_pos == hare.guard_pos && tortoise.guard_dir == hare.guard_dir {
                return true; // Loop detected
            }
        }
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
    map.visited
        .iter()
        .flat_map(|rows| rows.iter().filter(|&&v| v))
        .count() as u32
}

pub fn part_2(input: &Map) -> u32 {
    let mut map = input.clone();
    let original_pos = map.guard_pos;
    let original_dir = map.guard_dir;
    map.walk_until_stop();
    map.guard_pos = original_pos;
    map.guard_dir = original_dir;

    let mut n_loops = 0;
    for (i, j) in map
        .visited
        .clone()
        .iter()
        .enumerate()
        .flat_map(|(i, rows)| rows.iter().enumerate().map(move |(j, _)| (i, j)))
    {
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
