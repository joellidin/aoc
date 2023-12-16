use std::{
    collections::HashMap,
    fmt::{self, Debug, Display},
};

#[derive(Clone)]
pub struct Grid {
    map: Vec<Vec<Tile>>,
    beams: Vec<bool>,
}

#[derive(Clone, PartialEq, Eq)]
enum Tile {
    VerticalSplitter,
    HorizontalSplitter,
    SWMirror,
    NWMirror,
    Empty,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    North,
    West,
    South,
    East,
}

macro_rules! get_idx {
    ($map:expr, $i:expr, $j:expr $(, $dir:expr)?) => {
        $i * $map.map[0].len() + $j $(+ $dir as usize * $map.map.len() * $map.map[0].len())?
    };
}

impl TryFrom<usize> for Direction {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Direction::North),
            1 => Ok(Direction::West),
            2 => Ok(Direction::South),
            3 => Ok(Direction::East),
            _ => Err(()), // Return an error for invalid values
        }
    }
}

impl Debug for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut directions_map = HashMap::new();
        let n_rows = self.map.len();
        let n_cols = self.map[0].len();

        // Convert seen_states to HashMap
        for i in 0..n_rows {
            for j in 0..n_cols {
                let mut dirs: Vec<Direction> = Vec::new();
                for dir in 0..4 {
                    if self.beams[get_idx!(self, i, j, dir)] {
                        // Assuming Direction can be constructed from dir (which is 0..4)
                        dirs.push(dir.try_into().unwrap());
                    }
                }
                if !dirs.is_empty() {
                    directions_map.insert((i as isize, j as isize), dirs);
                }
            }
        }

        // Printing logic
        writeln!(f)?;
        for (i, row) in self.map.iter().enumerate() {
            for (j, tile) in row.iter().enumerate() {
                match tile {
                    Tile::VerticalSplitter => write!(f, "|"),
                    Tile::HorizontalSplitter => write!(f, "-"),
                    Tile::SWMirror => write!(f, "/"),
                    Tile::NWMirror => write!(f, "\\"),
                    Tile::Empty => {
                        if let Some(dirs) = directions_map.get(&(i as isize, j as isize)) {
                            if dirs.len() == 1 {
                                write!(f, "{}", dirs[0])
                            } else {
                                write!(f, "{}", dirs.len())
                            }
                        } else {
                            write!(f, ".")
                        }
                    }
                }?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Tile::VerticalSplitter => write!(f, "|"),
            Tile::HorizontalSplitter => write!(f, "-"),
            Tile::SWMirror => write!(f, "/"),
            Tile::NWMirror => write!(f, "\\"),
            Tile::Empty => write!(f, "."),
        }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Direction::North => write!(f, "^"),
            Direction::South => write!(f, "v"),
            Direction::East => write!(f, ">"),
            Direction::West => write!(f, "<"),
        }
    }
}

impl Grid {
    fn populate_energized_tiles(&mut self, start_beam: (isize, isize, Direction)) -> usize {
        let n_rows = self.map.len();
        let n_cols = self.map[0].len();
        let mut seen_states = vec![false; n_rows * n_cols * 4];
        let mut energized_tiles = vec![false; n_rows * n_cols];
        let mut beams = self.walk(start_beam);
        while let Some((row, col, direction)) = beams.pop() {
            let seen_state = seen_states[get_idx!(self, row, col, direction)];
            if !seen_state {
                seen_states[get_idx!(self, row, col, direction)] = true;
            } else {
                continue;
            }

            let is_energized = energized_tiles[get_idx!(self, row, col)];
            if !is_energized {
                energized_tiles[get_idx!(self, row, col)] = true;
            }

            beams.extend(self.walk((row as isize, col as isize, direction)));
        }
        self.beams = seen_states;
        energized_tiles.iter().filter(|s| **s).count()
    }

    fn walk(&self, (row, col, dir): (isize, isize, Direction)) -> Vec<(usize, usize, Direction)> {
        use Direction::*;
        let (new_i, new_j) = match dir {
            North => ((row - 1).max(0), col),
            West => (row, (col - 1).max(0)),
            South => ((row + 1).min(self.map.len() as isize - 1), col),
            East => (row, (col + 1).min(self.map[0].len() as isize - 1)),
        };
        let (new_i, new_j) = (new_i as usize, new_j as usize);
        match self.map[new_i][new_j] {
            Tile::VerticalSplitter => {
                if dir == South || dir == North {
                    vec![(new_i, new_j, dir)]
                } else {
                    vec![(new_i, new_j, North), (new_i, new_j, South)]
                }
            }
            Tile::HorizontalSplitter => {
                if dir == East || dir == West {
                    vec![(new_i, new_j, dir)]
                } else {
                    vec![(new_i, new_j, West), (new_i, new_j, East)]
                }
            }
            Tile::SWMirror => match dir {
                North => vec![(new_i, new_j, East)],
                West => vec![(new_i, new_j, South)],
                South => vec![(new_i, new_j, West)],
                East => vec![(new_i, new_j, North)],
            },
            Tile::NWMirror => match dir {
                North => vec![(new_i, new_j, West)],
                West => vec![(new_i, new_j, North)],
                South => vec![(new_i, new_j, East)],
                East => vec![(new_i, new_j, South)],
            },
            Tile::Empty => vec![(new_i, new_j, dir)],
        }
    }
}

pub fn generator(input: &str) -> Grid {
    let mut n_tiles = 0;
    Grid {
        map: input
            .lines()
            .map(|l| {
                l.chars()
                    .map(|c| {
                        n_tiles += 1;
                        match c {
                            '|' => Tile::VerticalSplitter,
                            '-' => Tile::HorizontalSplitter,
                            '/' => Tile::SWMirror,
                            '\\' => Tile::NWMirror,
                            '.' => Tile::Empty,
                            _ => panic!("Invalid character found."),
                        }
                    })
                    .collect()
            })
            .collect(),
        beams: vec![false; n_tiles * 4],
    }
}

pub fn part_1(input: &Grid) -> usize {
    let mut grid = input.clone();
    grid.populate_energized_tiles((0, -1, Direction::East))
}

pub fn part_2(input: &Grid) -> usize {
    let mut grid = input.clone();
    let mut all_energized_tiles = Vec::new();
    // Left wall
    for i in 0..grid.map.len() {
        all_energized_tiles.push(grid.populate_energized_tiles((i as isize, -1, Direction::East)))
    }

    // Right wall
    for i in 0..grid.map.len() {
        all_energized_tiles.push(grid.populate_energized_tiles((
            i as isize,
            grid.map[i].len() as isize,
            Direction::West,
        )))
    }

    // Top wall
    for j in 0..grid.map[0].len() {
        all_energized_tiles.push(grid.populate_energized_tiles((-1, j as isize, Direction::South)))
    }

    // Bottom wall
    for j in 0..grid.map[0].len() {
        all_energized_tiles.push(grid.populate_energized_tiles((
            grid.map.len() as isize,
            j as isize,
            Direction::North,
        )))
    }
    *all_energized_tiles.iter().max().unwrap()
}
