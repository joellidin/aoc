use std::{
    collections::{HashMap, HashSet},
    fmt::{Debug, Display, Formatter, Result},
};

#[derive(Clone)]
pub struct Grid {
    map: Vec<Vec<Tile>>,
    beams: HashSet<(isize, isize, Direction)>,
}

#[derive(Clone, PartialEq, Eq)]
enum Tile {
    VerticalSplitter,
    HorizontalSplitter,
    SWMirror,
    NWMirror,
    Empty,
}

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
enum Direction {
    North,
    West,
    South,
    East,
}

impl Debug for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut directions_map = HashMap::new();

        // Convert HashSet to HashMap
        for &(i, j, dir) in &self.beams {
            directions_map
                .entry((i, j))
                .or_insert_with(Vec::new)
                .push(dir);
        }

        // Printing logic
        for (i, row) in self.map.iter().enumerate() {
            for (j, tile) in row.iter().enumerate() {
                match directions_map.get(&(i.try_into().unwrap(), j.try_into().unwrap())) {
                    Some(dirs) if *tile == Tile::Empty => {
                        // Print directions only if the tile is empty
                        if dirs.len() == 1 {
                            write!(f, "{}", dirs[0])?;
                        } else {
                            write!(f, "{}", dirs.len())?;
                        }
                    }
                    _ => {
                        // Print tile representation in all other cases
                        write!(f, "{}", tile)?;
                    }
                }
            }
            writeln!(f)?; // New line at the end of each row
        }
        Ok(())
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
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
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Direction::North => write!(f, "^"),
            Direction::South => write!(f, "v"),
            Direction::East => write!(f, ">"),
            Direction::West => write!(f, "<"),
        }
    }
}

impl Grid {
    fn populate_energized_tiles(&mut self, start_beam: (isize, isize, Direction)) -> &mut Self {
        self.beams.drain();
        let mut beams = self.walk(start_beam);
        while let Some((i, j, direction)) = beams.pop() {
            let new_tile = self.beams.insert((i, j, direction));
            if !new_tile {
                continue;
            }
            beams.extend(self.walk((i, j, direction)));
        }
        self
    }

    fn walk(&self, (i, j, dir): (isize, isize, Direction)) -> Vec<(isize, isize, Direction)> {
        use Direction::*;
        let (new_i, new_j) = match dir {
            North => ((i - 1).max(0), j),
            West => (i, (j - 1).max(0)),
            South => ((i + 1).min(self.map.len() as isize - 1), j),
            East => (i, (j + 1).min(self.map[0].len() as isize - 1)),
        };
        match self.map[new_i as usize][new_j as usize] {
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

    fn count_energized_tiles(&self) -> usize {
        let mut seen = HashSet::with_capacity(self.beams.len());
        self.beams
            .iter()
            .filter(|&&(i, j, _)| seen.insert((i, j)))
            .count()
    }
}

pub fn generator(input: &str) -> Grid {
    Grid {
        map: input
            .lines()
            .map(|l| {
                l.chars()
                    .map(|c| match c {
                        '|' => Tile::VerticalSplitter,
                        '-' => Tile::HorizontalSplitter,
                        '/' => Tile::SWMirror,
                        '\\' => Tile::NWMirror,
                        '.' => Tile::Empty,
                        _ => panic!("Invalid character found."),
                    })
                    .collect()
            })
            .collect(),
        beams: HashSet::new(),
    }
}

pub fn part_1(input: &Grid) -> usize {
    let mut grid = input.clone();
    grid.populate_energized_tiles((0, -1, Direction::East))
        .count_energized_tiles()
}

pub fn part_2(input: &Grid) -> usize {
    let mut grid = input.clone();
    let mut all_energized_tiles = Vec::new();
    // Left wall
    for i in 0..grid.map.len() {
        all_energized_tiles.push(
            grid.populate_energized_tiles((i as isize, -1, Direction::East))
                .count_energized_tiles(),
        )
    }

    // Right wall
    for i in 0..grid.map.len() {
        all_energized_tiles.push(
            grid.populate_energized_tiles((
                i as isize,
                grid.map[i].len() as isize,
                Direction::West,
            ))
            .count_energized_tiles(),
        )
    }

    // Top wall
    for j in 0..grid.map[0].len() {
        all_energized_tiles.push(
            grid.populate_energized_tiles((-1, j as isize, Direction::South))
                .count_energized_tiles(),
        )
    }

    // Bottom wall
    for j in 0..grid.map[0].len() {
        all_energized_tiles.push(
            grid.populate_energized_tiles((grid.map.len() as isize, j as isize, Direction::North))
                .count_energized_tiles(),
        )
    }
    *all_energized_tiles.iter().max().unwrap()
}
