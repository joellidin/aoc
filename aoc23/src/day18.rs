#[derive(Copy, Clone)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

pub struct Instruction {
    direction: Direction,
    steps: i64,
    color: String,
}

fn get_steps_from_color(color: &str) -> i64 {
    i64::from_str_radix(&color.trim_start_matches('#')[..5], 16).unwrap()
}

fn get_dir_from_color(color: &str) -> Direction {
    match color.chars().last() {
        Some('0') => Direction::Right,
        Some('1') => Direction::Down,
        Some('2') => Direction::Left,
        Some('3') => Direction::Up,
        _ => panic!("Invalid color direction."),
    }
}

pub fn generator(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|l| {
            let mut parts = l.split_whitespace();
            let direction = match parts.next() {
                Some("R") => Direction::Right,
                Some("L") => Direction::Left,
                Some("U") => Direction::Up,
                Some("D") => Direction::Down,
                _ => panic!("Something wrong with parsing input."),
            };
            let steps = parts
                .next()
                .expect("Not any number to parse.")
                .parse()
                .expect("Could not parse into a number.");
            let color = parts
                .next()
                .expect("Not any color to parse.")
                .replace(['(', ')'], "");
            Instruction {
                direction,
                steps,
                color,
            }
        })
        .collect()
}

fn solve(instructions: &[Instruction], is_color_instructions: bool) -> i64 {
    let mut pos = (0, 0);
    let mut shoelace = 0;
    let mut border_length = 0;
    instructions.iter().for_each(|i| {
        let step = if is_color_instructions {
            get_steps_from_color(&i.color)
        } else {
            i.steps
        };
        let direction = if is_color_instructions {
            get_dir_from_color(&i.color)
        } else {
            i.direction
        };
        let new_pos = match direction {
            Direction::Up => (pos.0 - step, pos.1),
            Direction::Down => (pos.0 + step, pos.1),
            Direction::Right => (pos.0, pos.1 + step),
            Direction::Left => (pos.0, pos.1 - step),
        };
        shoelace += pos.0 * new_pos.1 - pos.1 * new_pos.0;
        border_length += step;
        pos = new_pos;
    });
    // Shoelace theorem
    let area = (shoelace).abs() / 2;
    // Picks theorem
    area + 1 + border_length / 2
}

pub fn part_1(input: &[Instruction]) -> i64 {
    solve(input, false)
}

pub fn part_2(input: &[Instruction]) -> i64 {
    solve(input, true)
}
