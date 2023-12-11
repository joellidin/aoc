use std::{collections::HashSet, fmt, str::FromStr};

pub struct Map {
    map: Vec<Row>,
    start: Position,
    loop_positions: Option<HashSet<Position>>,
    tiles_inside_loop: Option<HashSet<Position>>,
}

struct Row {
    tiles: Vec<Tile>,
}

#[derive(Eq, PartialEq, Copy, Clone, Hash)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(PartialEq)]
enum Tile {
    Floor,
    VPipe,
    HPipe,
    NEPipe,
    NWPipe,
    SEPipe,
    SWPipe,
    Start,
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Tile {
    fn get_tile_char(&self) -> &'static str {
        match self {
            Tile::VPipe => "┃",
            Tile::HPipe => "━",
            Tile::NEPipe => "┗",
            Tile::NWPipe => "┛",
            Tile::SEPipe => "┏",
            Tile::SWPipe => "┓",
            Tile::Floor => "•",
            Tile::Start => "\x1b[5;1;31;43mS\x1b[0m",
        }
    }
}

impl fmt::Debug for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Map:")?;
        for (y, row) in self.map.iter().enumerate() {
            for (x, tile) in row.tiles.iter().enumerate() {
                let position = Position { x, y };
                let default_char = tile.get_tile_char();

                let output_char = if position == self.start {
                    format!("\x1b[5;102;31m{}\x1b[0m", default_char)
                } else if self
                    .tiles_inside_loop
                    .as_ref()
                    .map_or(false, |tiles| tiles.contains(&position))
                {
                    format!("\x1b[93m{}\x1b[0m", default_char)
                } else if self
                    .loop_positions
                    .as_ref()
                    .map_or(false, |loops| loops.contains(&position))
                {
                    format!("\x1b[31m{}\x1b[0m", default_char)
                } else {
                    default_char.to_string()
                };
                write!(f, "{}", output_char)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl FromStr for Row {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut v = vec![];
        for c in s.chars() {
            v.push(match c {
                '.' => Tile::Floor,
                '|' => Tile::VPipe,
                '-' => Tile::HPipe,
                'L' => Tile::NEPipe,
                'J' => Tile::NWPipe,
                'F' => Tile::SEPipe,
                '7' => Tile::SWPipe,
                'S' => Tile::Start,
                _ => panic!("Invalid character parsed."),
            });
        }
        Ok(Row { tiles: v })
    }
}

impl Map {
    fn new(input: &str) -> Self {
        let map = input.lines().map(|l| l.parse().unwrap()).collect();
        let start = Map::find_start_position(input).expect("There is no start in this map.");

        Map {
            map,
            start,
            loop_positions: None,
            tiles_inside_loop: None,
        }
    }

    fn find_start_position(map_str: &str) -> Result<Position, &'static str> {
        map_str
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars().enumerate().filter_map(move |(x, c)| {
                    if c == 'S' {
                        Some(Position { x, y })
                    } else {
                        None
                    }
                })
            })
            .next()
            .ok_or("Start position not found")
    }

    fn calculate_next_position(
        &self,
        position: &Position,
        direction: &Direction,
    ) -> Result<Position, &'static str> {
        let (mut x, mut y) = (position.x as i32, position.y as i32);
        match direction {
            Direction::North => y -= 1,
            Direction::South => y += 1,
            Direction::East => x += 1,
            Direction::West => x -= 1,
        }

        if x < 0 || y < 0 || x >= self.map[0].tiles.len() as i32 || y >= self.map.len() as i32 {
            return Err("Out of bounds");
        }

        Ok(Position {
            x: x as usize,
            y: y as usize,
        })
    }

    fn update_direction(
        &self,
        position: &Position,
        current_direction: &Direction,
    ) -> Result<Direction, &'static str> {
        let tile = &self.map[position.y].tiles[position.x];
        match tile {
            Tile::Floor => Err("Cannot move on floor tile"),
            Tile::VPipe => match current_direction {
                Direction::North | Direction::South => Ok(*current_direction),
                _ => Err("Invalid direction for vertical pipe"),
            },
            Tile::HPipe => match current_direction {
                Direction::East | Direction::West => Ok(*current_direction),
                _ => Err("Invalid direction for horizontal pipe"),
            },
            Tile::NEPipe => match current_direction {
                Direction::West => Ok(Direction::North),
                Direction::South => Ok(Direction::East),
                _ => Err("Invalid direction for north-east pipe"),
            },
            Tile::NWPipe => match current_direction {
                Direction::East => Ok(Direction::North),
                Direction::South => Ok(Direction::West),
                _ => Err("Invalid direction for north-west pipe"),
            },
            Tile::SEPipe => match current_direction {
                Direction::West => Ok(Direction::South),
                Direction::North => Ok(Direction::East),
                _ => Err("Invalid direction for south-east pipe"),
            },
            Tile::SWPipe => match current_direction {
                Direction::North => Ok(Direction::West),
                Direction::East => Ok(Direction::South),
                _ => Err("Invalid direction for south-west pipe"),
            },
            Tile::Start => Ok(*current_direction),
        }
    }

    fn find_loop(&mut self) -> Result<(), &'static str> {
        let mut current_position = self.start;
        let mut loop_positions = HashSet::new();
        loop_positions.insert(self.start);

        for dir in [
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ] {
            let mut direction = dir;
            while let Ok(next_position) =
                self.calculate_next_position(&current_position, &direction)
            {
                // Check if next position is the start position
                if next_position == self.start && loop_positions.len() > 1 {
                    self.loop_positions = Some(loop_positions);
                    self.map[self.start.y].tiles[self.start.x] = self.find_start_pipe_type()?;
                    return Ok(());
                }

                // Update the current position and direction
                direction = match self.update_direction(&next_position, &direction) {
                    Ok(d) => d,
                    Err(_) => {
                        current_position = self.start;
                        loop_positions.drain();
                        loop_positions.insert(self.start);
                        break;
                    }
                };
                current_position = next_position;
                loop_positions.insert(current_position);
            }
        }
        Err("No loops found.")
    }

    fn find_start_pipe_type(&self) -> Result<Tile, &'static str> {
        let mut allowed_directions = vec![];
        let loop_positions = self
            .loop_positions
            .as_ref()
            .expect("Need to find loop before using the loop positions.");

        for dir in [
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ] {
            let adjacent_pos = match self.calculate_next_position(&self.start, &dir) {
                Ok(p) => p,
                Err(_) => continue,
            };

            if loop_positions.contains(&adjacent_pos)
                && self.update_direction(&adjacent_pos, &dir).is_ok()
            {
                allowed_directions.push(dir);
            }
        }

        match (allowed_directions[0], allowed_directions[1]) {
            (Direction::North, Direction::South) => Ok(Tile::VPipe),
            (Direction::North, Direction::East) => Ok(Tile::NEPipe),
            (Direction::North, Direction::West) => Ok(Tile::NWPipe),
            (Direction::South, Direction::West) => Ok(Tile::SWPipe),
            (Direction::East, Direction::South) => Ok(Tile::SEPipe),
            (Direction::East, Direction::West) => Ok(Tile::HPipe),
            _ => Err("Something was wrong with finding start pipe"),
        }
    }

    fn find_tiles_inside_loop(&mut self) -> Result<(), &'static str> {
        let loop_positions = self
            .loop_positions
            .as_ref()
            .ok_or("Loop positions not found.")?;
        self.tiles_inside_loop = Some(HashSet::new());

        for (y, row) in self.map.iter().enumerate() {
            let mut corner = None;
            let mut inside_loop = false;

            row.tiles.iter().enumerate().for_each(|(x, tile)| {
                let position = Position { x, y };
                let loop_item = loop_positions.contains(&position);
                let interior_tile = !loop_item && inside_loop;
                match tile {
                    // flips the orientation
                    Tile::VPipe if loop_item => {
                        inside_loop = !inside_loop;
                        corner = None;
                    }
                    // flips the orientation if a north-east pipe is followed by a south-west pipe
                    // or if a south-east pipe is followed by a north-west pipe
                    Tile::NEPipe | Tile::SEPipe if loop_item => corner = Some(tile),
                    Tile::SWPipe | Tile::NWPipe if loop_item => match (corner, tile) {
                        (Some(Tile::NEPipe), Tile::SWPipe) | (Some(Tile::SEPipe), Tile::NWPipe) => {
                            inside_loop = !inside_loop
                        }
                        _ => {}
                    },
                    // does not change the orientation
                    _ => {}
                }
                if interior_tile {
                    if let Some(ref mut tiles) = self.tiles_inside_loop {
                        tiles.insert(position);
                    }
                }
            });
        }
        Ok(())
    }
}

pub fn generator(input: &str) -> Result<Map, &'static str> {
    let mut map = Map::new(input);
    map.find_loop()?;
    map.find_tiles_inside_loop()?;
    Ok(map)
}

pub fn part_1(map: &Result<Map, &'static str>) -> u32 {
    map.as_ref()
        .expect("Something is wrong with the map parsing")
        .loop_positions
        .as_ref()
        .unwrap()
        .len() as u32
        / 2
}

pub fn part_2(map: &Result<Map, &'static str>) -> u32 {
    map.as_ref()
        .expect("Something is wrong with the map parsing")
        .tiles_inside_loop
        .as_ref()
        .unwrap()
        .len() as u32
}
