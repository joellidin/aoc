use std::{
    collections::{HashMap, HashSet},
    fmt::{Debug, Formatter, Result},
    hash::{Hash, Hasher},
};

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct Map {
    map: Vec<Vec<Tile>>,
}

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub enum Tile {
    RoundedRock,
    SquaredRock,
    Empty,
}

pub fn generator(input: &str) -> Map {
    use Tile::*;
    Map {
        map: input
            .lines()
            .map(|l| {
                l.chars()
                    .map(|c| match c {
                        'O' => RoundedRock,
                        '#' => SquaredRock,
                        '.' => Empty,
                        _ => panic!("Invalid character"),
                    })
                    .collect()
            })
            .collect(),
    }
}

impl Debug for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Tile::RoundedRock => Ok(write!(f, "O")?),
            Tile::SquaredRock => Ok(write!(f, "#")?),
            Tile::Empty => Ok(write!(f, ".")?),
        }
    }
}
impl Debug for Map {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        writeln!(f, "Map:")?;
        for row in self.map.iter() {
            for tile in row.iter() {
                write!(f, "{tile:?}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Map {
    fn tilt_north(&mut self) {
        use Tile::*;
        let n_rows = self.map.len();
        let mut movements = Vec::new();
        let mut moved_rocks = HashSet::new();
        self.map.iter().enumerate().for_each(|(i, row)| {
            row.iter().enumerate().for_each(|(j, tile)| match tile {
                RoundedRock => {
                    if moved_rocks.get(&(i, j)).is_some() {
                        for k in i + 1..n_rows {
                            match self.map[k][j] {
                                RoundedRock => {
                                    if moved_rocks.get(&(k, j)).is_none() {
                                        movements.push((k, i, j));
                                        moved_rocks.insert((k, j));
                                        break;
                                    }
                                }
                                SquaredRock => break,
                                Empty => continue,
                            }
                        }
                    }
                }
                SquaredRock => {}
                Empty => {
                    for k in i + 1..n_rows {
                        match self.map[k][j] {
                            RoundedRock => {
                                if moved_rocks.get(&(k, j)).is_none() {
                                    movements.push((k, i, j));
                                    moved_rocks.insert((k, j));
                                    break;
                                }
                            }
                            SquaredRock => break,
                            Empty => continue,
                        }
                    }
                }
            });
        });

        // Apply movements
        for (from_i, to_i, j) in movements {
            self.map[to_i][j] = RoundedRock;
            self.map[from_i][j] = Empty;
        }
    }

    fn tilt_south(&mut self) {
        use Tile::*;
        let mut movements = Vec::new();
        let mut moved_rocks = HashSet::new();
        self.map.iter().enumerate().rev().for_each(|(i, row)| {
            row.iter().enumerate().for_each(|(j, tile)| match tile {
                RoundedRock => {
                    if moved_rocks.get(&(i, j)).is_some() {
                        for k in (0..i + 1).rev() {
                            match self.map[k][j] {
                                RoundedRock => {
                                    if moved_rocks.get(&(k, j)).is_none() {
                                        movements.push((k, i, j));
                                        moved_rocks.insert((k, j));
                                        break;
                                    }
                                }
                                SquaredRock => break,
                                Empty => continue,
                            }
                        }
                    }
                }
                SquaredRock => {}
                Empty => {
                    for k in (0..i + 1).rev() {
                        match self.map[k][j] {
                            RoundedRock => {
                                if moved_rocks.get(&(k, j)).is_none() {
                                    movements.push((k, i, j));
                                    moved_rocks.insert((k, j));
                                    break;
                                }
                            }
                            SquaredRock => break,
                            Empty => continue,
                        }
                    }
                }
            });
        });

        // Apply movements
        for (from_i, to_i, j) in movements {
            self.map[to_i][j] = RoundedRock;
            self.map[from_i][j] = Empty;
        }
    }

    fn tilt_west(&mut self) {
        use Tile::*;
        let n_rows = self.map.len();
        let n_cols = self.map[0].len();
        let mut movements = Vec::new();
        let mut moved_rocks = HashSet::new();

        // Iterate over each row
        for i in 0..n_rows {
            // Iterate over columns from right to left
            for j in 0..n_cols {
                match self.map[i][j] {
                    RoundedRock => {
                        // Check if the RoundedRock at the current position has already moved
                        if moved_rocks.get(&(i, j)).is_some() {
                            for k in j + 1..n_cols {
                                match self.map[i][k] {
                                    RoundedRock => {
                                        // Check if the RoundedRock at position k hasn't moved
                                        if moved_rocks.get(&(i, k)).is_none() {
                                            movements.push((i, k, j));
                                            moved_rocks.insert((i, k));
                                            break;
                                        }
                                    }
                                    SquaredRock => break,
                                    Empty => continue,
                                }
                            }
                        }
                    }
                    SquaredRock => {}
                    Empty => {
                        // Check for a RoundedRock to the left
                        for k in j + 1..n_cols {
                            match self.map[i][k] {
                                RoundedRock => {
                                    // Move RoundedRock if it hasn't moved
                                    if moved_rocks.get(&(i, k)).is_none() {
                                        movements.push((i, k, j));
                                        moved_rocks.insert((i, k));
                                        break;
                                    }
                                }
                                SquaredRock => break,
                                Empty => continue,
                            }
                        }
                    }
                }
            }
        }

        // Apply movements
        for (i, from_j, to_j) in movements {
            self.map[i][to_j] = RoundedRock;
            self.map[i][from_j] = Empty;
        }
    }

    fn tilt_east(&mut self) {
        use Tile::*;
        let n_rows = self.map.len();
        let n_cols = self.map[0].len();
        let mut movements = Vec::new();
        let mut moved_rocks = HashSet::new();

        // Iterate over each row
        for i in 0..n_rows {
            // Iterate over columns from right to left
            for j in (0..n_cols).rev() {
                match self.map[i][j] {
                    RoundedRock => {
                        // Check if the RoundedRock at the current position has already moved
                        if moved_rocks.get(&(i, j)).is_some() {
                            for k in (0..j).rev() {
                                match self.map[i][k] {
                                    RoundedRock => {
                                        // Check if the RoundedRock at position k hasn't moved
                                        if moved_rocks.get(&(i, k)).is_none() {
                                            movements.push((i, k, j));
                                            moved_rocks.insert((i, k));
                                            break;
                                        }
                                    }
                                    SquaredRock => break,
                                    Empty => continue,
                                }
                            }
                        }
                    }
                    SquaredRock => {}
                    Empty => {
                        // Check for a RoundedRock to the left
                        for k in (0..j).rev() {
                            match self.map[i][k] {
                                RoundedRock => {
                                    // Move RoundedRock if it hasn't moved
                                    if moved_rocks.get(&(i, k)).is_none() {
                                        movements.push((i, k, j));
                                        moved_rocks.insert((i, k));
                                        break;
                                    }
                                }
                                SquaredRock => break,
                                Empty => continue,
                            }
                        }
                    }
                }
            }
        }

        // Apply movements
        for (i, from_j, to_j) in movements {
            self.map[i][to_j] = RoundedRock;
            self.map[i][from_j] = Empty;
        }
    }

    fn cycle(&mut self, n: usize) {
        if n == 0 {
            return;
        }

        let mut i = 0;
        let cycle_length = {
            let mut seen_maps = HashMap::new();
            loop {
                self.tilt_north();
                self.tilt_west();
                self.tilt_south();
                self.tilt_east();
                i += 1;

                // Compute hash of self.map
                let mut hasher = std::collections::hash_map::DefaultHasher::new();
                self.map.hash(&mut hasher);
                let map_hash = hasher.finish();

                if let Some(seen_index) = seen_maps.get(&map_hash) {
                    break i - seen_index;
                } else {
                    seen_maps.insert(map_hash, i);
                }

                if i >= n {
                    return;
                }
            }
        };

        // Skip unnecessary iterations if n is much larger than the cycle length
        let remaining_steps = (n - i) % cycle_length;
        self.cycle(remaining_steps);
    }
    fn count_points(&self) -> usize {
        use Tile::*;
        self.map
            .iter()
            .rev()
            .enumerate()
            .map(|(i, row)| {
                row.iter()
                    .filter(|tile| matches!(tile, RoundedRock))
                    .count()
                    * (i + 1)
            })
            .sum()
    }
}

pub fn part_1(input: &Map) -> usize {
    let mut map = input.clone();
    map.tilt_north();
    map.count_points()
}

pub fn part_2(input: &Map) -> usize {
    let mut map = input.clone();
    map.cycle(1_000_000_000);
    map.count_points()
}
