use std::{
    collections::HashMap,
    fmt::{Debug, Formatter, Result},
};

const MAX_ROWS: usize = 102;

#[derive(Clone, Copy)]
pub struct Grid {
    round_rocks: [u128; MAX_ROWS],
    square_rocks: [u128; MAX_ROWS],
}

enum TiltDirection {
    West,
    East,
}

impl Debug for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let start_row = self
            .square_rocks
            .iter()
            .position(|n| n == &u128::MAX)
            .unwrap();
        writeln!(f)?;
        for i in start_row + 1..MAX_ROWS - 1 {
            for j in 1..(MAX_ROWS - start_row - 1) {
                let mask = 1 << j;
                if self.square_rocks[i] & mask != 0 {
                    write!(f, "#")?;
                } else if self.round_rocks[i] & mask != 0 {
                    write!(f, "O")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Grid {
    fn new(input: &str) -> Self {
        let mut round_rocks = [0; MAX_ROWS];
        let mut square_rocks = [0; MAX_ROWS];

        // Determine the length of the rows (plus walls on both sides)
        let row_length = input.lines().next().map_or(0, |s| s.len()) + 2;

        // Calculate the number of input rows
        let num_input_rows = input.lines().count();
        let top_wall_index = MAX_ROWS - num_input_rows - 2; // Account for the top wall

        // Add the top wall
        square_rocks[top_wall_index] = u128::MAX;

        for (i, line) in input.lines().enumerate() {
            let mut round_row = 0_u128;
            let mut square_row = 1 | (1 << (row_length - 1)); // Add walls on both sides
            let row_index = top_wall_index + 1 + i;

            for (j, ch) in line.chars().enumerate() {
                let pos = j + 1; // Offset by 1 due to the left wall
                if ch == 'O' {
                    round_row |= 1 << pos;
                } else if ch == '#' {
                    square_row |= 1 << pos;
                }
            }

            round_rocks[row_index] = round_row;
            square_rocks[row_index] = square_row;
        }

        // Add the bottom wall
        square_rocks[MAX_ROWS - 1] = u128::MAX;

        Grid {
            round_rocks,
            square_rocks,
        }
    }

    fn tilt_west(&mut self) {
        for i in 0..MAX_ROWS {
            self.round_rocks[i] = self.tilt_row(
                self.round_rocks[i],
                self.square_rocks[i],
                TiltDirection::West,
            );
        }
    }

    fn tilt_east(&mut self) {
        for i in 0..MAX_ROWS {
            self.round_rocks[i] = self.tilt_row(
                self.round_rocks[i],
                self.square_rocks[i],
                TiltDirection::East,
            );
        }
    }

    fn tilt_row(
        self,
        mut round_rocks: u128,
        square_rocks: u128,
        tilt_direction: TiltDirection,
    ) -> u128 {
        use TiltDirection::*;
        let mut any_moved;

        loop {
            any_moved = false;

            // Mask to identify positions that are free (no round or square rocks).
            let free_positions = match tilt_direction {
                West => !(round_rocks | square_rocks) << 1,
                East => !(round_rocks | square_rocks) >> 1,
            };
            // Identify movable rocks: round rocks that have a free position
            // left for the west direction and right for the east.
            let movable_rocks = round_rocks & free_positions;

            if movable_rocks != 0 {
                // Move the identified rocks based on tilt direction
                round_rocks ^= match tilt_direction {
                    West => movable_rocks | (movable_rocks >> 1),
                    East => movable_rocks | (movable_rocks << 1),
                };
                any_moved = true;
            }

            if !any_moved {
                break;
            }
        }
        round_rocks
    }

    fn tilt_north(&mut self) {
        for col in 1..MAX_ROWS - 1 {
            let col_mask = 1 << col;
            let mut can_move = true;

            while can_move {
                can_move = false;

                // Start from the second row (1) up to the second-to-last row (MAX_ROWS - 1)
                for row in 1..MAX_ROWS - 1 {
                    if (self.round_rocks[row] & col_mask) != 0  // Check if there's a round rock in the current position
                    && (self.round_rocks[row - 1] & col_mask) == 0  // Check if the position above is empty
                    && (self.square_rocks[row - 1] & col_mask) == 0
                    // Check if there's no square rock in the position above
                    {
                        // Move the rock up
                        self.round_rocks[row] &= !col_mask; // Remove the rock from the current position
                        self.round_rocks[row - 1] |= col_mask; // Place the rock in the position above
                        can_move = true;
                    }
                }
            }
        }
    }

    fn tilt_south(&mut self) {
        for col in 1..MAX_ROWS {
            let col_mask = 1 << col;
            let mut can_move = true;

            while can_move {
                can_move = false;

                // Start from the second-to-last row and go down to the second row
                for row in (1..MAX_ROWS - 2).rev() {
                    if (self.round_rocks[row] & col_mask) != 0  // Check if there's a round rock in the current position
                    && (self.round_rocks[row + 1] & col_mask) == 0  // Check if the position below is empty
                    && (self.square_rocks[row + 1] & col_mask) == 0
                    // Check if there's no square rock in the position below
                    {
                        // Move the rock down
                        self.round_rocks[row] &= !col_mask; // Remove the rock from the current position
                        self.round_rocks[row + 1] |= col_mask; // Place the rock in the position below
                        can_move = true;
                    }
                }
            }
        }
    }

    fn cycle(&mut self, n: usize) {
        let mut i = 0;
        let mut seen_maps = HashMap::new();

        while i < n {
            self.tilt_north();
            self.tilt_west();
            self.tilt_south();
            self.tilt_east();
            i += 1;

            if let Some(seen_index) = seen_maps.get(&self.round_rocks) {
                let cycle_length = i - seen_index;
                if cycle_length == 0 {
                    break;
                }
                // Skip the remaining cycles
                i += ((n - i) / cycle_length) * cycle_length;
                continue;
            } else {
                seen_maps.insert(self.round_rocks, i);
            }
        }
    }

    fn count_points(&self) -> usize {
        let mut score = 0;
        for (i, &row) in self.round_rocks.iter().enumerate().skip(1) {
            for j in 1..MAX_ROWS {
                if row & (1 << j) != 0 {
                    // Calculate the row number, with the bottom row being row 1
                    let row_number = MAX_ROWS - i - 1;
                    score += row_number;
                }
            }
        }
        score
    }
}

pub fn generator(input: &str) -> Grid {
    Grid::new(input)
}

pub fn part_1(input: &Grid) -> usize {
    let mut grid = *input;
    grid.tilt_north();
    grid.count_points()
}

pub fn part_2(input: &Grid) -> usize {
    let mut grid = *input;
    grid.cycle(1_000_000_000);
    grid.count_points()
}
