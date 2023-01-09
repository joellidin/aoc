use std::fmt::Debug;

const MAX_ROWS: usize = 32;

struct Grid {
    width: usize,
    height: usize,
    walls: [u128; MAX_ROWS],
    start: (usize, u128),
    goal: (usize, u128),
    bliz_north: [u128; MAX_ROWS],
    bliz_south: [u128; MAX_ROWS],
    bliz_west: [u128; MAX_ROWS],
    bliz_east: [u128; MAX_ROWS],
    presence: [u128; MAX_ROWS],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    x: isize,
    y: isize,
}

impl Debug for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;
        for y in 0..self.height {
            for x in 0..self.width {
                // Count number of blizzards at this position
                let mut blizzards = 0;
                if self.bliz_north[y] & (1 << x) != 0 {
                    blizzards += 1;
                }
                if self.bliz_south[y] & (1 << x) != 0 {
                    blizzards += 1;
                }
                if self.bliz_west[y] & (1 << x) != 0 {
                    blizzards += 1;
                }
                if self.bliz_east[y] & (1 << x) != 0 {
                    blizzards += 1;
                }
                if blizzards > 1 {
                    write!(f, "{blizzards}")?;
                } else if self.walls[y] & (1 << x) != 0 {
                    write!(f, "#")?;
                } else if self.bliz_north[y] & (1 << x) != 0 {
                    write!(f, "^")?;
                } else if self.bliz_south[y] & (1 << x) != 0 {
                    write!(f, "v")?;
                } else if self.bliz_west[y] & (1 << x) != 0 {
                    write!(f, "<")?;
                } else if self.bliz_east[y] & (1 << x) != 0 {
                    write!(f, ">")?;
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
        let width = input.lines().next().unwrap().len();
        let height = input.lines().count();
        let mut start: (usize, u128) = (0, 0);
        let mut goal: (usize, u128) = (height - 1, 0);
        let mut presence = [0u128; MAX_ROWS];
        let mut walls = [0u128; MAX_ROWS];
        let mut bliz_north = [0u128; MAX_ROWS];
        let mut bliz_south = [0u128; MAX_ROWS];
        let mut bliz_east = [0u128; MAX_ROWS];
        let mut bliz_west = [0u128; MAX_ROWS];

        input.lines().enumerate().for_each(|(y, line)| {
            line.chars().enumerate().for_each(|(x, c)| {
                match c {
                    '^' => bliz_north[y] |= 1 << x,
                    'v' => bliz_south[y] |= 1 << x,
                    '<' => bliz_west[y] |= 1 << x,
                    '>' => bliz_east[y] |= 1 << x,
                    '#' => walls[y] |= 1 << x,
                    _ => (),
                };
                if y == 0 && c == '.' {
                    start.1 |= 1 << x;
                }
                if y == height - 1 && c == '.' {
                    goal.1 |= 1 << x;
                }
            })
        });
        presence[start.0] = start.1;
        Grid {
            width,
            height,
            walls,
            start,
            goal,
            bliz_north,
            bliz_south,
            bliz_west,
            bliz_east,
            presence,
        }
    }

    fn update_blizzards(&mut self) {
        // Simulate blizzards for this turn
        self.bliz_north[1..(self.height - 1)].rotate_left(1);
        self.bliz_south[1..(self.height - 1)].rotate_right(1);
        for r in 1..(self.height - 1) {
            self.bliz_west[r] = self.blow_west(self.bliz_west[r], self.walls[r]);
            self.bliz_east[r] = self.blow_east(self.bliz_east[r], self.walls[r]);
        }
    }

    fn blow_west(&self, before: u128, walls: u128) -> u128 {
        let wind = before >> 1;
        if wind & walls != 0 {
            wind | (walls >> 1)
        } else {
            wind
        }
    }

    fn blow_east(&self, row: u128, walls: u128) -> u128 {
        let wind = row << 1;
        if wind & walls != 0 {
            wind | (walls << 1)
        } else {
            wind
        }
    }

    fn step(&mut self) {
        self.update_blizzards();
        let mut above = 0;
        for row in 0..self.height {
            let current = self.presence[row];
            self.presence[row] |= above | (current << 1) | (current >> 1);
            if row + 1 < self.height {
                self.presence[row] |= self.presence[row + 1];
            }
            above = current;
            let obstacle = self.walls[row]
                | self.bliz_north[row]
                | self.bliz_south[row]
                | self.bliz_west[row]
                | self.bliz_east[row];
            self.presence[row] &= !obstacle;
        }
    }

    fn is_goal(&self) -> bool {
        self.presence[self.goal.0] & self.goal.1 != 0
    }

    fn steps_to_goal(&mut self) -> u16 {
        let mut steps = 0;
        while !self.is_goal() {
            self.step();
            steps += 1;
        }
        steps
    }

    fn reset_and_swap_goal(&mut self) {
        std::mem::swap(&mut self.start, &mut self.goal);
        self.presence.fill(0u128);
        self.presence[self.start.0] = self.start.1;
    }
}

pub fn part_1(input: &str) -> u16 {
    let mut grid = Grid::new(input);
    grid.steps_to_goal()
}

pub fn part_2(input: &str) -> u16 {
    let mut grid = Grid::new(input);
    let first = grid.steps_to_goal();
    grid.reset_and_swap_goal();
    let second = grid.steps_to_goal();
    grid.reset_and_swap_goal();
    let third = grid.steps_to_goal();
    first + second + third
}
