use std::vec;

use aoc_utils::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Wall,
    BoxLeft,
    BoxRight,
    Void,
}

#[derive(Debug, Clone, Copy)]
pub enum Instruction {
    Up,
    Down,
    Left,
    Right,
}

impl Instruction {
    pub fn as_tuple(self) -> (i32, i32) {
        match self {
            Instruction::Up => (-1, 0),
            Instruction::Down => (1, 0),
            Instruction::Left => (0, -1),
            Instruction::Right => (0, 1),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Grid {
    map: Vec<Vec<Tile>>,
    robot: Vec2<i32>,
    expandend: bool,
}

pub fn generator(input: &str) -> (Grid, Vec<Instruction>) {
    let (map_str, instructions_str) = input.split_once("\n\n").unwrap();
    let mut robot = Vec2 { x: 0, y: 0 };
    let map = map_str
        .lines()
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(|(j, c)| match c {
                    '#' => Tile::Wall,
                    'O' => Tile::BoxLeft,
                    '.' => Tile::Void,
                    '@' => {
                        robot = (j, i).into();
                        Tile::Void
                    }
                    _ => panic!(),
                })
                .collect()
        })
        .collect();
    let instructions = instructions_str
        .lines()
        .flat_map(|line| {
            line.chars()
                .map(|c| match c {
                    '^' => Instruction::Up,
                    '>' => Instruction::Right,
                    '<' => Instruction::Left,
                    'v' => Instruction::Down,
                    _ => panic!(),
                })
                .collect::<Vec<_>>()
        })
        .collect();
    (
        Grid {
            map,
            robot,
            expandend: false,
        },
        instructions,
    )
}

impl Grid {
    fn move_one(&mut self, instruction: &Instruction) {
        let (dy, dx) = instruction.as_tuple();
        self.move_boxes(instruction);
        let tile = self.map[(self.robot.y + dy) as usize][(self.robot.x + dx) as usize];
        if matches!(tile, Tile::Void) {
            self.robot.x += dx;
            self.robot.y += dy;
        }
    }

    fn collect_vertical_boxes(
        &self,
        instruction: &Instruction,
        start_pos: Vec2<i32>,
    ) -> Option<Vec<(Tile, Vec2<i32>)>> {
        let mut result = Vec::new();
        let mut next_pos = start_pos;

        let (dy, _) = instruction.as_tuple();
        next_pos.y += dy;

        let tile = self.map[next_pos.y as usize][next_pos.x as usize];

        match tile {
            Tile::Wall => return None,
            Tile::Void => return Some(result),
            Tile::BoxLeft | Tile::BoxRight => {
                // Identify the box pair
                let (left_pos, right_pos) = match tile {
                    Tile::BoxLeft => {
                        let l = next_pos;
                        let r = (next_pos.x + 1, next_pos.y).into();
                        (l, r)
                    }
                    Tile::BoxRight => {
                        let r = next_pos;
                        let l = (next_pos.x - 1, next_pos.y).into();
                        (l, r)
                    }
                    _ => unreachable!(),
                };

                result.push((Tile::BoxLeft, left_pos));
                // Only do this in the expanded grid
                if self.expandend {
                    result.push((Tile::BoxRight, right_pos));
                }

                let positions = if self.expandend {
                    &vec![left_pos, right_pos]
                } else {
                    &vec![left_pos]
                };
                // Check boxes above or below
                for &pos in positions {
                    if let Some(mut above_boxes) = self.collect_vertical_boxes(instruction, pos) {
                        result.append(&mut above_boxes);
                    } else {
                        return None;
                    }
                }
            }
        }
        Some(result)
    }

    fn collect_horizontal_boxes(
        &self,
        instruction: &Instruction,
        start_pos: Vec2<i32>,
    ) -> Option<Vec<(Tile, Vec2<i32>)>> {
        let mut result = Vec::new();
        let mut current_pos = start_pos;

        loop {
            // Move one step in the given horizontal direction
            let (_, dx) = instruction.as_tuple();
            current_pos.x += dx;

            let tile = self.map[current_pos.y as usize][current_pos.x as usize];

            match tile {
                Tile::Wall => return None,
                Tile::Void => return Some(result),
                Tile::BoxLeft | Tile::BoxRight => {
                    result.push((tile, current_pos));
                }
            }
        }
    }

    fn collect_boxes(&self, instruction: &Instruction) -> Option<Vec<(Tile, Vec2<i32>)>> {
        match instruction {
            Instruction::Left | Instruction::Right => {
                self.collect_horizontal_boxes(instruction, self.robot)
            }

            Instruction::Up | Instruction::Down => {
                self.collect_vertical_boxes(instruction, self.robot)
            }
        }
    }

    fn move_boxes(&mut self, instruction: &Instruction) {
        if let Some(boxes) = self.collect_boxes(instruction) {
            // Remove them from the map
            for (_, pos) in &boxes {
                self.map[pos.y as usize][pos.x as usize] = Tile::Void;
            }

            // Move them in the given direction
            for (tile, pos) in boxes {
                let (dy, dx) = instruction.as_tuple();
                self.map[(pos.y + dy) as usize][(pos.x + dx) as usize] = tile;
            }
        }
    }

    fn expand(&mut self) {
        let height = self.map.len();
        let width = self.map[0].len();
        for i in 0..height {
            for j in (0..width * 2).step_by(2) {
                let tile = self.map[i][j];
                match tile {
                    Tile::Wall | Tile::Void => self.map[i].insert(j, tile),
                    Tile::BoxLeft => self.map[i].insert(j + 1, Tile::BoxRight),
                    _ => unreachable!(),
                }
            }
        }
        self.robot.x *= 2;
        self.expandend = true;
    }

    #[allow(dead_code)]
    fn print_grid(&self) {
        for (y, row) in self.map.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                if x as i32 == self.robot.x && y as i32 == self.robot.y {
                    print!("@");
                } else {
                    match tile {
                        Tile::Wall => print!("#"),
                        Tile::BoxLeft => print!("["),
                        Tile::BoxRight => print!("]"),
                        Tile::Void => print!("."),
                    }
                }
            }
            println!();
        }
    }
}

pub fn part_1(input: &(Grid, Vec<Instruction>)) -> u32 {
    let (grid, instructions) = input;
    let mut grid = grid.to_owned();
    for instruction in instructions {
        grid.move_one(instruction);
    }
    let mut res = 0;
    for (i, row) in grid.map.iter().enumerate() {
        for (j, &tile) in row.iter().enumerate() {
            if matches!(tile, Tile::BoxLeft) {
                res += 100 * i + j;
            }
        }
    }
    res as u32
}

pub fn part_2(input: &(Grid, Vec<Instruction>)) -> u32 {
    let (grid, instructions) = input;
    let mut grid = grid.to_owned();
    grid.expand();
    for instruction in instructions {
        grid.move_one(instruction);
    }
    let mut res = 0;
    for (i, row) in grid.map.iter().enumerate() {
        for (j, &tile) in row.iter().enumerate() {
            if matches!(tile, Tile::BoxLeft) {
                res += 100 * i + j;
            }
        }
    }
    res as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_1: &str = r#"##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
"#;

    const INPUT_2: &str = r#"########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<
"#;

    #[test]
    fn part_1_test() {
        let generator_output = generator(INPUT_1);
        let result = part_1(&generator_output);
        assert_eq!(result, 10092);

        let generator_output = generator(INPUT_2);
        let result = part_1(&generator_output);
        assert_eq!(result, 2028);
    }

    #[test]
    fn part_2_test() {
        let generator_output = generator(INPUT_1);
        let result = part_2(&generator_output);
        assert_eq!(result, 9021);
    }
}
