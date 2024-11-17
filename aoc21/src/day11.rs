use std::{collections::VecDeque, fmt, str::FromStr};

#[derive(Clone)]
pub struct Grid {
    map: Vec<Vec<u8>>,
}

impl FromStr for Grid {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let map = s
            .lines()
            .map(|line| line.bytes().map(|b| (b - b'0')).collect::<Vec<u8>>())
            .collect();
        Ok(Grid { map })
    }
}

impl fmt::Debug for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f)?;
        for row in &self.map {
            for &n in row {
                if n == 0 {
                    // Write bold for zeros
                    write!(f, "\x1b[1m{}\x1b[0m ", n)?;
                } else {
                    // Write regular for other numbers
                    write!(f, "{} ", n)?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Grid {
    fn simulate_steps(&mut self, n: u32) -> u32 {
        let mut n_flashes = 0;
        for _ in 0..n {
            // Increment all energy levels by 1
            self.map.iter_mut().flatten().for_each(|n| *n += 1);
            n_flashes += self.flash_map();
        }
        n_flashes
    }

    fn flash_map(&mut self) -> u32 {
        let mut n_flashes = 0;
        let rows = self.map.len();
        let cols = self.map[0].len();

        // 2D array to keep track of flashed positions
        let mut flashed = vec![vec![false; cols]; rows];
        let mut to_flash = VecDeque::new();

        // Find initial flashes
        for row in 0..rows {
            for col in 0..cols {
                if self.map[row][col] > 9 {
                    to_flash.push_back((row, col));
                }
            }
        }

        // Process flashes
        while let Some((row, col)) = to_flash.pop_front() {
            if flashed[row][col] {
                continue;
            }
            flashed[row][col] = true;
            n_flashes += 1;

            // Increment neighbors' energy
            for delta_row in -1..=1 {
                for delta_col in -1..=1 {
                    if delta_row == 0 && delta_col == 0 {
                        continue;
                    }

                    let neighbor_row = row as isize + delta_row;
                    let neighbor_col = col as isize + delta_col;

                    // Check bounds
                    if neighbor_row >= 0
                        && neighbor_col >= 0
                        && neighbor_row < rows as isize
                        && neighbor_col < cols as isize
                    {
                        let neighbor_row = neighbor_row as usize;
                        let neighbor_col = neighbor_col as usize;
                        self.map[neighbor_row][neighbor_col] += 1;
                        if self.map[neighbor_row][neighbor_col] > 9
                            && !flashed[neighbor_row][neighbor_col]
                        {
                            to_flash.push_back((neighbor_row, neighbor_col));
                        }
                    }
                }
            }
        }

        // Reset energy levels of flashed octopuses
        (0..rows).for_each(|row| {
            for col in 0..cols {
                if flashed[row][col] {
                    self.map[row][col] = 0;
                }
            }
        });

        n_flashes
    }
}

pub fn generator(input: &str) -> Grid {
    input.parse().expect("Input must be a grid of numbers.")
}

pub fn part_1(input: &Grid) -> u32 {
    let mut grid = input.clone();
    grid.simulate_steps(100)
}

pub fn part_2(input: &Grid) -> u32 {
    let mut grid = input.clone();
    let mut steps = 0;
    let total_octopuses = (grid.map.len() * grid.map[0].len()) as u32;
    loop {
        steps += 1;
        let flashes = grid.simulate_steps(1);
        if flashes == total_octopuses {
            return steps;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526
"#;

    #[test]
    fn part_1_test() {
        let generator_output = generator(INPUT);
        let result = part_1(&generator_output);
        assert_eq!(result, 1656);
    }

    #[test]
    fn part_2_test() {
        let generator_output = generator(INPUT);
        let result = part_2(&generator_output);
        assert_eq!(result, 195);
    }
}
