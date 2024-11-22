use std::fmt::Debug;

const FILLED_CHARACTER: &str = "██";
const EMPTY_CHARACTER: &str = "░░";

#[derive(Clone)]
pub struct Grid {
    map: Vec<Vec<bool>>,
    instructions: Vec<Direction>,
}

#[derive(Debug, Clone)]
enum Direction {
    Up(u32),
    Left(u32),
}

#[derive(Debug)]
pub enum GeneratorError {
    ParseError,
}

pub fn generator(input: &str) -> Result<Grid, GeneratorError> {
    if let Some((map_str, instructions_str)) = input.split_once("\n\n") {
        let points = map_str
            .lines()
            .map(|l| {
                let (x, y) = l.split_once(',').expect("Must be comma separated numbers");
                (
                    x.parse::<u32>().expect("Must be a valid number"),
                    y.parse::<u32>().expect("Must be a valid number"),
                )
            })
            .collect::<Vec<_>>();
        let max_x = points.iter().map(|(x, _)| x).max().unwrap();
        let max_y = points.iter().map(|(_, y)| y).max().unwrap();
        let mut map = vec![vec![false; *max_x as usize + 1]; *max_y as usize + 1];
        points
            .iter()
            .for_each(|(x, y)| map[*y as usize][*x as usize] = true);

        let instructions = instructions_str
            .lines()
            .map(|l| match l.starts_with("fold along y=") {
                true => {
                    let fold_value = l
                        .trim_start_matches("fold along y=")
                        .parse::<u32>()
                        .expect("Must be a valid fold point");
                    Direction::Up(fold_value)
                }
                false => {
                    let fold_value = l
                        .trim_start_matches("fold along x=")
                        .parse::<u32>()
                        .expect("Must be a valid fold point");
                    Direction::Left(fold_value)
                }
            })
            .collect();
        Ok(Grid { map, instructions })
    } else {
        Err(GeneratorError::ParseError)
    }
}

impl Debug for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;
        for row in &self.map {
            for &n in row {
                if n {
                    write!(f, "{}", FILLED_CHARACTER)?;
                } else {
                    write!(f, "{}", EMPTY_CHARACTER)?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Grid {
    fn fold_direction(&mut self, direction: &Direction) {
        match &direction {
            Direction::Up(fold_number) => {
                for i in *fold_number as usize + 1..self.map.len() {
                    for j in 0..self.map[i].len() {
                        self.map[(*fold_number as usize * 2).saturating_sub(i)][j] |= self.map[i][j]
                    }
                }
                self.map.drain(*fold_number as usize..);
            }
            Direction::Left(fold_number) => {
                for row in &mut self.map {
                    for j in (*fold_number as usize + 1)..row.len() {
                        row[(*fold_number as usize * 2).saturating_sub(j)] |= row[j];
                    }
                    row.drain(*fold_number as usize..);
                }
            }
        }
    }

    fn fold(&mut self) -> String {
        self.instructions
            .clone()
            .into_iter()
            .for_each(|dir| self.fold_direction(&dir));
        self.map
            .iter()
            .map(|row| {
                row.iter()
                    .map(|&is_dot| {
                        if is_dot {
                            FILLED_CHARACTER
                        } else {
                            EMPTY_CHARACTER
                        }
                    })
                    .collect::<String>()
            })
            .collect::<Vec<String>>()
            .join("\n")
    }
}

pub fn part_1(input: &Result<Grid, GeneratorError>) -> u32 {
    let mut grid = input.as_ref().unwrap().clone();
    grid.fold_direction(&grid.instructions[0].clone());
    grid.map
        .iter()
        .flat_map(|row| row.iter().map(|&is_dot| if is_dot { 1 } else { 0 }))
        .sum()
}

pub fn part_2(input: &Result<Grid, GeneratorError>) -> String {
    let mut grid = input.as_ref().unwrap().clone();
    grid.fold()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5
"#;

    #[test]
    fn part_1_test() {
        let generator_output = generator(INPUT);
        let result = part_1(&generator_output);
        assert_eq!(result, 17);
    }

    #[test]
    fn part_2_test() {
        let generator_output = generator(INPUT);
        let result = part_2(&generator_output);
        let expected = r#"██████████
██░░░░░░██
██░░░░░░██
██░░░░░░██
██████████
░░░░░░░░░░
░░░░░░░░░░"#;
        assert_eq!(result, expected.to_string());
    }
}
