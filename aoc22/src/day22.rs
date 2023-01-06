use std::str::FromStr;
use std::vec::Vec;

#[derive(Debug)]
struct Map {
    map: Vec<Row>,
    player: Player,
}

#[derive(Debug, Clone)]
struct Row {
    tiles: Vec<Tile>,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
struct Player {
    pos: Position,
    dir: Direction,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    East,
    South,
    West,
    North,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tile {
    Void,
    Open,
    Solid,
}

#[derive(Debug)]
struct Instructions {
    instructions: Vec<Action>,
}

#[derive(Debug)]
enum Action {
    TurnLeft,
    TurnRight,
    MoveForward(usize),
}

impl FromStr for Instructions {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut v = Vec::new();
        let mut num = 0;
        for c in s.trim().chars() {
            if c.is_alphabetic() {
                v.push(Action::MoveForward(num));
                num = 0;
                v.push(match c {
                    'L' => Action::TurnLeft,
                    'R' => Action::TurnRight,
                    _ => panic!("Unknown instruction: {c}"),
                });
            } else {
                num = num * 10 + c.to_digit(10).unwrap() as usize;
            };
        }
        v.push(Action::MoveForward(num));
        Ok(Instructions { instructions: v })
    }
}

impl FromStr for Row {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut v = vec![Tile::Void];
        for c in s.chars() {
            v.push(match c {
                '#' => Tile::Solid,
                '.' => Tile::Open,
                _ => Tile::Void,
            });
        }
        v.push(Tile::Void);
        Ok(Row { tiles: v })
    }
}

impl Map {
    fn new(input: &str) -> Self {
        let rows = input
            .lines()
            .map(|l| l.parse().unwrap())
            .collect::<Vec<Row>>();

        let width = rows.iter().map(|row| row.tiles.len()).max().unwrap();
        let mut map = rows
            .iter()
            .map(|row| {
                let mut tiles = row.tiles.clone();
                if row.tiles.len() < width {
                    tiles.resize(width, Tile::Void);
                    return Row { tiles };
                };
                row.clone()
            })
            .collect::<Vec<_>>();
        let start_x = rows[0].tiles.iter().position(|t| *t == Tile::Open).unwrap();
        let player = Player {
            pos: Position { x: start_x, y: 1 },
            dir: Direction::East,
        };

        map.insert(
            0,
            Row {
                tiles: vec![Tile::Void; width],
            },
        );
        map.insert(
            map.len(),
            Row {
                tiles: vec![Tile::Void; width],
            },
        );
        Map { map, player }
    }
}

fn solve(instructions: &Instructions, map: &mut Map, warp: fn(&mut Map, Position)) -> usize {
    for action in instructions.instructions.iter() {
        match action {
            Action::TurnLeft => {
                map.player.dir = match map.player.dir {
                    Direction::North => Direction::West,
                    Direction::East => Direction::North,
                    Direction::South => Direction::East,
                    Direction::West => Direction::South,
                }
            }
            Action::TurnRight => {
                map.player.dir = match map.player.dir {
                    Direction::North => Direction::East,
                    Direction::East => Direction::South,
                    Direction::South => Direction::West,
                    Direction::West => Direction::North,
                }
            }
            Action::MoveForward(n) => {
                for _ in 0..*n {
                    let old_pos = map.player.pos;
                    let (dx, dy) = match map.player.dir {
                        Direction::North => (map.player.pos.x, map.player.pos.y - 1),
                        Direction::East => (map.player.pos.x + 1, map.player.pos.y),
                        Direction::South => (map.player.pos.x, map.player.pos.y + 1),
                        Direction::West => (map.player.pos.x - 1, map.player.pos.y),
                    };
                    if map.map[dy].tiles[dx] == Tile::Solid {
                        break;
                    }
                    map.player.pos = Position { x: dx, y: dy };
                    if map.map[dy].tiles[dx] == Tile::Void {
                        warp(map, old_pos);
                    }
                }
            }
        }
        assert!(map.map[map.player.pos.y].tiles[map.player.pos.x] == Tile::Open);
    }
    map.player.pos.y * 1000 + map.player.pos.x * 4 + map.player.dir as usize
}

fn go_through_void(map: &mut Map, old_pos: Position) {
    let (mut x, mut y) = (map.player.pos.x, map.player.pos.y);
    loop {
        match map.player.dir {
            Direction::North => y += 1,
            Direction::East => x -= 1,
            Direction::South => y -= 1,
            Direction::West => x += 1,
        };
        if map.map[y].tiles[x] == Tile::Void {
            break;
        }
    }

    // Go back one step
    match map.player.dir {
        Direction::North => y -= 1,
        Direction::East => x += 1,
        Direction::South => y += 1,
        Direction::West => x -= 1,
    };
    if map.map[y].tiles[x] != Tile::Solid {
        map.player.pos = Position { x, y };
    } else {
        map.player.pos = old_pos;
    }
}

fn get_quadrant(pos: &Position) -> usize {
    match (pos.x, pos.y) {
        (x, y) if (51..=100).contains(&x) && (1..=50).contains(&y) => 1,
        (x, y) if (101..=150).contains(&x) && (1..=50).contains(&y) => 2,
        (x, y) if (51..=100).contains(&x) && (51..=100).contains(&y) => 3,
        (x, y) if (1..=50).contains(&x) && (101..=150).contains(&y) => 4,
        (x, y) if (51..=100).contains(&x) && (101..=150).contains(&y) => 5,
        (x, y) if (1..=50).contains(&x) && (151..=200).contains(&y) => 6,
        _ => panic!("Point not on Cube"),
    }
}

fn go_through_void_cube(map: &mut Map, old_pos: Position) {
    let (x, y);
    let mut dir = map.player.dir;
    let quadrant = get_quadrant(&old_pos);
    match (quadrant, map.player.dir) {
        (1, Direction::North) => {
            x = 1;
            y = map.player.pos.x + 100;
            dir = Direction::East;
        }
        (1, Direction::West) => {
            x = 1;
            y = 151 - map.player.pos.y;
            dir = Direction::East;
        }
        (2, Direction::North) => {
            x = map.player.pos.x - 100;
            y = 200;
        }
        (2, Direction::East) => {
            x = 100;
            y = 151 - map.player.pos.y;
            dir = Direction::West;
        }
        (2, Direction::South) => {
            x = 100;
            y = map.player.pos.x - 50;
            dir = Direction::West;
        }
        (3, Direction::West) => {
            x = map.player.pos.y - 50;
            y = 101;
            dir = Direction::South;
        }
        (3, Direction::East) => {
            x = map.player.pos.y + 50;
            y = 50;
            dir = Direction::North;
        }
        (4, Direction::North) => {
            x = 51;
            y = map.player.pos.x + 50;
            dir = Direction::East;
        }
        (4, Direction::West) => {
            x = 51;
            y = 151 - map.player.pos.y;
            dir = Direction::East;
        }
        (5, Direction::East) => {
            x = 150;
            y = 151 - map.player.pos.y;
            dir = Direction::West;
        }
        (5, Direction::South) => {
            x = 50;
            y = map.player.pos.x + 100;
            dir = Direction::West;
        }
        (6, Direction::South) => {
            x = map.player.pos.x + 100;
            y = 1;
        }
        (6, Direction::East) => {
            x = map.player.pos.y - 100;
            y = 150;
            dir = Direction::North;
        }
        (6, Direction::West) => {
            x = map.player.pos.y - 100;
            y = 1;
            dir = Direction::South;
        }
        (_, _) => panic!("Not a possible void value"),
    };
    if map.map[y].tiles[x] != Tile::Solid {
        map.player.pos = Position { x, y };
        map.player.dir = dir;
    } else {
        map.player.pos = old_pos;
    }
}

pub fn solution() {
    let input = include_str!("../data/day22.txt");
    let (map, instructions) = input.split_once("\n\n").unwrap();
    let mut map = Map::new(map);
    let i = instructions.parse::<Instructions>().unwrap();
    let start = map.player;
    println!("Final Password: {}", solve(&i, &mut map, go_through_void));
    map.player = start;
    println!(
        "Cube Final Password: {}",
        solve(&i, &mut map, go_through_void_cube)
    );
}

#[cfg(test)]
mod test {
    use super::{Direction, Instructions, Map, Player, Position};

    fn get_map() -> Map {
        let input = include_str!("../data/day22.test");
        let (map, _) = input.split_once("\n\n").unwrap();
        Map::new(map)
    }

    #[test]
    fn test_go_though_void() {
        let mut map = get_map();
        let dummy = Position { x: 0, y: 0 };
        // D -> C
        map.player.pos = Position { x: 6, y: 4 };
        map.player.dir = Direction::North;
        super::go_through_void(&mut map, dummy);
        assert_eq!(map.player.pos, Position { x: 6, y: 8 });

        // C -> D
        map.player.pos = Position { x: 6, y: 4 };
        map.player.dir = Direction::South;
        super::go_through_void(&mut map, dummy);
        assert_eq!(map.player.pos, Position { x: 6, y: 4 });

        // A -> B
        map.player.pos = Position { x: 13, y: 7 };
        map.player.dir = Direction::East;
        super::go_through_void(&mut map, dummy);
        assert_eq!(map.player.pos, Position { x: 1, y: 7 });

        // B -> A
        map.player.pos = Position { x: 0, y: 7 };
        map.player.dir = Direction::West;
        super::go_through_void(&mut map, dummy);
        assert_eq!(map.player.pos, Position { x: 12, y: 7 });

        // Go into wall
        map.player.pos = Position { x: 8, y: 1 };
        map.player.dir = Direction::West;
        super::go_through_void(&mut map, Position { x: 9, y: 1 });
        assert_eq!(map.player.pos, Position { x: 9, y: 1 });
    }

    #[test]
    fn test_solve() {
        // Test get stopped by solid
        let warp = super::go_through_void;
        let mut map = get_map();
        let ins = "10R".parse::<Instructions>().unwrap();
        map.player.pos = Position { x: 9, y: 1 };
        map.player.dir = Direction::East;
        super::solve(&ins, &mut map, warp);
        assert_eq!(
            map.player,
            Player {
                pos: Position { x: 11, y: 1 },
                dir: Direction::South
            }
        );

        let ins = "100L".parse::<Instructions>().unwrap();
        map.player.pos = Position { x: 11, y: 1 };
        map.player.dir = Direction::North;
        super::solve(&ins, &mut map, warp);
        assert_eq!(
            map.player,
            Player {
                pos: Position { x: 11, y: 9 },
                dir: Direction::West
            }
        );

        let ins = "3L".parse::<Instructions>().unwrap();
        map.player.pos = Position { x: 12, y: 8 };
        map.player.dir = Direction::East;
        super::solve(&ins, &mut map, warp);
        assert_eq!(
            map.player,
            Player {
                pos: Position { x: 3, y: 8 },
                dir: Direction::North
            }
        );

        let ins = "3L1R".parse::<Instructions>().unwrap();
        map.player.pos = Position { x: 1, y: 8 };
        map.player.dir = Direction::East;
        super::solve(&ins, &mut map, warp);
        assert_eq!(
            map.player,
            Player {
                pos: Position { x: 4, y: 7 },
                dir: Direction::East
            }
        );
    }

    #[test]
    fn test_get_quadrant() {
        let pos = Position { x: 75, y: 25 };
        assert_eq!(super::get_quadrant(&pos), 1);

        let pos = Position { x: 125, y: 25 };
        assert_eq!(super::get_quadrant(&pos), 2);

        let pos = Position { x: 75, y: 75 };
        assert_eq!(super::get_quadrant(&pos), 3);

        let pos = Position { x: 25, y: 125 };
        assert_eq!(super::get_quadrant(&pos), 4);

        let pos = Position { x: 75, y: 125 };
        assert_eq!(super::get_quadrant(&pos), 5);

        let pos = Position { x: 25, y: 175 };
        assert_eq!(super::get_quadrant(&pos), 6);

        let pos = Position { x: 51, y: 53 };
        assert_eq!(super::get_quadrant(&pos), 3);
    }

    #[test]
    #[should_panic]
    fn test_get_quadrant_panic() {
        let pos = Position { x: 0, y: 0 };
        super::get_quadrant(&pos);
    }
}
