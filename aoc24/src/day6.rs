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
        let mut pos = self.guard_pos;
        let mut dir = self.guard_dir;
        while matches!(
            self.step(&mut pos, &mut dir),
            StepResult::Step | StepResult::Turn
        ) {
            self.visited[pos.1][pos.0] = true;
        }
    }

    fn is_loop(&self) -> bool {
        let mut tortoise_pos = self.guard_pos;
        let mut tortoise_dir = self.guard_dir;
        let mut hare_pos = self.guard_pos;
        let mut hare_dir = self.guard_dir;

        loop {
            // Move tortoise one step
            if matches!(
                self.step(&mut tortoise_pos, &mut tortoise_dir),
                StepResult::OutOfBounds
            ) {
                return false;
            }

            // Move hare two steps
            for _ in 0..2 {
                if matches!(
                    self.step(&mut hare_pos, &mut hare_dir),
                    StepResult::OutOfBounds
                ) {
                    return false;
                }
            }

            // Check if tortoise and hare meet
            if tortoise_pos == hare_pos && tortoise_dir == hare_dir {
                return true; // Loop detected
            }
        }
    }

    fn step(&self, pos: &mut (usize, usize), dir: &mut Direction) -> StepResult {
        let (width, height) = (self.grid[0].len(), self.grid.len());

        // Compute next position
        let (new_x, new_y) = match dir {
            Direction::North => {
                if pos.1 == 0 {
                    return StepResult::OutOfBounds;
                }
                (pos.0, pos.1 - 1)
            }
            Direction::South => {
                if pos.1 + 1 >= height {
                    return StepResult::OutOfBounds;
                }
                (pos.0, pos.1 + 1)
            }
            Direction::East => {
                if pos.0 + 1 >= width {
                    return StepResult::OutOfBounds;
                }
                (pos.0 + 1, pos.1)
            }
            Direction::West => {
                if pos.0 == 0 {
                    return StepResult::OutOfBounds;
                }
                (pos.0 - 1, pos.1)
            }
        };

        // Check grid cell
        match self.grid[new_y][new_x] {
            b'.' => {
                *pos = (new_x, new_y);
                StepResult::Step
            }
            b'#' => {
                // Rotate right
                *dir = match dir {
                    Direction::North => Direction::East,
                    Direction::East => Direction::South,
                    Direction::South => Direction::West,
                    Direction::West => Direction::North,
                };
                StepResult::Turn
            }
            _ => panic!("Invalid character in grid"),
        }
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
