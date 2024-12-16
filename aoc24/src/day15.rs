use std::{
    io::{self, Write},
    thread::sleep,
    time::Duration,
};

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

    fn get_next_position(self, position: &Vec2<i32>) -> Vec2<i32> {
        let (dy, dx) = self.as_tuple();
        *position + (dx, dy)
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
        let pos = instruction.get_next_position(&self.robot);
        self.move_boxes(instruction);
        let tile = self.map[pos.i()][pos.j()];
        if matches!(tile, Tile::Void) {
            self.robot = pos;
        }
    }

    fn collect_vertical_boxes(
        &self,
        instruction: &Instruction,
        start_pos: Vec2<i32>,
    ) -> Option<Vec<(Tile, Vec2<i32>)>> {
        let mut result = Vec::new();
        let next_pos = instruction.get_next_position(&start_pos);

        let tile = self.map[next_pos.i()][next_pos.j()];

        match tile {
            Tile::Wall => return None,
            Tile::Void => return Some(result),
            Tile::BoxLeft | Tile::BoxRight => {
                // Identify the box pair
                let (left_pos, right_pos) = match tile {
                    Tile::BoxLeft => {
                        let l = next_pos;
                        let r = next_pos + (1, 0);
                        (l, r)
                    }
                    Tile::BoxRight => {
                        let r = next_pos;
                        let l = next_pos + (-1, 0);
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
            current_pos = instruction.get_next_position(&current_pos);

            let tile = self.map[current_pos.i()][current_pos.j()];

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
                self.map[pos.i()][pos.j()] = Tile::Void;
            }

            // Move them in the given direction
            for (tile, pos) in boxes {
                let new_pos = instruction.get_next_position(&pos);
                self.map[new_pos.i()][new_pos.j()] = tile;
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
    fn solve(&mut self, instructions: &[Instruction]) -> u32 {
        for instruction in instructions {
            self.move_one(instruction);
        }
        let mut res = 0;
        for (i, row) in self.map.iter().enumerate() {
            for (j, &tile) in row.iter().enumerate() {
                if matches!(tile, Tile::BoxLeft) {
                    res += 100 * i + j;
                }
            }
        }
        res as u32
    }
}

#[allow(dead_code)]
impl Grid {
    /// Draws a tile at a specific position
    fn draw_tile<W: Write>(&self, handle: &mut W, x: i32, y: i32, is_robot: bool) {
        let symbol = if is_robot {
            "\x1B[1;93m@\x1B[0m" // Robot in bold yellow
        } else {
            match self.map.get(y as usize).and_then(|row| row.get(x as usize)) {
                Some(Tile::Wall) => "\x1B[34m#\x1B[0m",        // Blue wall
                Some(Tile::BoxLeft) => "\x1B[32m[\x1B[0m",     // Green box left
                Some(Tile::BoxRight) => "\x1B[32m]\x1B[0m",    // Green box right
                Some(Tile::Void) | None => "\x1B[90m.\x1B[0m", // Gray empty or out-of-bounds
            }
        };

        write!(handle, "\x1B[{};{}H{}", y + 1, x + 1, symbol).unwrap();
    }

    /// Draws the entire grid
    fn draw_full_grid(&self) {
        let stdout = io::stdout();
        let mut handle = io::BufWriter::new(stdout.lock());

        for (y, row) in self.map.iter().enumerate() {
            for (x, _) in row.iter().enumerate() {
                let is_robot = x as i32 == self.robot.x && y as i32 == self.robot.y;
                self.draw_tile(&mut handle, x as i32, y as i32, is_robot);
            }
            writeln!(handle).unwrap();
        }

        handle.flush().unwrap();
    }

    fn redraw_square(&self, sleep_interval: u64) {
        let stdout = io::stdout();
        let mut handle = io::BufWriter::new(stdout.lock());

        let radius = 25;
        let center_x = self.robot.x;
        let center_y = self.robot.y;

        for dy in -radius..=radius {
            for dx in -radius..=radius {
                let x = center_x + dx;
                let y = center_y + dy;

                let is_robot = x == self.robot.x && y == self.robot.y;
                if x >= 0 && y >= 0 {
                    self.draw_tile(&mut handle, x, y, is_robot);
                }
            }
        }

        handle.flush().unwrap();
        sleep(Duration::from_millis(sleep_interval));
    }

    /// Solve function with animation
    fn solve_animated(&mut self, instructions: &[Instruction], sleep_interval: u64) -> u32 {
        // Clear the screen and hide the cursor
        print!("\x1B[2J\x1B[H\x1B[?25l");

        // Draw the entire grid initially
        self.draw_full_grid();

        for instruction in instructions {
            self.move_one(instruction);
            self.redraw_square(sleep_interval);
        }

        // Calculate the result
        let mut res = 0;
        for (i, row) in self.map.iter().enumerate() {
            for (j, &tile) in row.iter().enumerate() {
                if matches!(tile, Tile::BoxLeft) {
                    res += 100 * i + j;
                }
            }
        }

        // Move the cursor below the grid
        let grid_height = self.map.len();
        print!("\x1B[{};1H", grid_height + 3);

        // Frame settings
        let total_width = 32; // Total width of the frame
        let title = "FINAL RESULT";
        let score_text = format!("SCORE: {}", res);

        // Helper to center-align text with odd/even padding handling
        fn center_text(text: &str, width: usize) -> String {
            let total_padding = width.saturating_sub(text.len());
            let left_padding = total_padding / 2;
            let right_padding = total_padding - left_padding;
            format!(
                "{:left$}{}{:right$}",
                "",
                text,
                "",
                left = left_padding,
                right = right_padding
            )
        }

        // Print the frame and content
        println!("\x1B[1;96m{}\x1B[0m", "*".repeat(total_width)); // Top border
        println!("\x1B[1;96m*{}*\x1B[0m", center_text(title, total_width - 2));
        println!(
            "\x1B[1;92m*{}*\x1B[0m",
            center_text(&score_text, total_width - 2)
        );
        println!("\x1B[1;96m{}\x1B[0m", "*".repeat(total_width)); // Bottom border

        // Show the cursor again
        print!("\x1B[?25h");

        res as u32
    }
}

pub fn part_1(input: &(Grid, Vec<Instruction>)) -> u32 {
    let (grid, instructions) = input;
    let mut grid = grid.to_owned();
    grid.solve(instructions)
}

pub fn part_2(input: &(Grid, Vec<Instruction>)) -> u32 {
    let (grid, instructions) = input;
    let mut grid = grid.to_owned();
    grid.expand();
    grid.solve(instructions)
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
