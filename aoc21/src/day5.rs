use std::str::FromStr;

#[derive(Debug)]
pub struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
pub struct ParsePointError;

impl FromStr for Point {
    type Err = ParsePointError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(',').ok_or(ParsePointError)?;
        let x = x.trim().parse::<i32>().map_err(|_| ParsePointError)?;
        let y = y.trim().parse::<i32>().map_err(|_| ParsePointError)?;
        Ok(Point { x, y })
    }
}

pub fn generator(input: &str) -> (Point, Point, Vec<(Point, Point)>) {
    let (mut min_x, mut min_y, mut max_x, mut max_y) = (i32::MAX, i32::MAX, i32::MIN, i32::MIN);

    let maps: Vec<(Point, Point)> = input
        .lines()
        .map(|line| {
            let (from, to) = line.split_once(" -> ").expect("Wrong input format.");
            let from = from.parse::<Point>().expect("Invalid point format.");
            let to = to.parse::<Point>().expect("Invalid point format.");

            max_x = max_x.max(from.x).max(to.x);
            max_y = max_y.max(from.y).max(to.y);
            min_x = min_x.min(from.x).min(to.x);
            min_y = min_y.min(from.y).min(to.y);

            (from, to)
        })
        .collect();

    (
        Point { x: min_x, y: min_y },
        Point { x: max_x, y: max_y },
        maps,
    )
}

pub fn part_1(input: &(Point, Point, Vec<(Point, Point)>)) -> u32 {
    let (_, max_point, maps) = input;
    let mut grid = vec![0; ((max_point.x + 1) * (max_point.y + 1)) as usize];
    maps.iter().for_each(|(from, to)| {
        if (from.x - to.x).abs() > 0 && (from.y - to.y).abs() > 0 {
            return;
        }

        let x_range = if from.x <= to.x {
            from.x..=to.x
        } else {
            to.x..=from.x
        };
        for i in x_range {
            let y_range = if from.y <= to.y {
                from.y..=to.y
            } else {
                to.y..=from.y
            };
            for j in y_range {
                grid[(j * (max_point.x + 1) + i) as usize] += 1;
            }
        }
    });
    grid.iter().filter(|&&x| x > 1).count() as u32
}

pub fn part_2(input: &(Point, Point, Vec<(Point, Point)>)) -> u32 {
    let (_, max_point, maps) = input;

    // Calculate grid dimensions
    let mut grid = vec![0; ((max_point.x + 1) * (max_point.y + 1)) as usize];
    maps.iter().for_each(|(from, to)| {
        if (from.x - to.x).abs() > 0 && (from.y - to.y).abs() > 0 {
            let x_step = if from.x < to.x { 1 } else { -1 };
            let y_step = if from.y < to.y { 1 } else { -1 };
            let steps = (from.x - to.x).abs();

            for step in 0..=steps {
                let x = from.x + step * x_step;
                let y = from.y + step * y_step;
                let idx = (y * (max_point.x + 1) + x) as usize;
                grid[idx] += 1;
            }
            return;
        }

        let x_range = if from.x <= to.x {
            from.x..=to.x
        } else {
            to.x..=from.x
        };
        for i in x_range {
            let y_range = if from.y <= to.y {
                from.y..=to.y
            } else {
                to.y..=from.y
            };
            for j in y_range {
                grid[(j * (max_point.x + 1) + i) as usize] += 1;
            }
        }
    });
    grid.iter().filter(|&&x| x > 1).count() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2
"#;

    #[test]
    fn part_1_test() {
        let generator_output = generator(INPUT);
        let result = part_1(&generator_output);
        assert_eq!(result, 5);
    }

    #[test]
    fn part_2_test() {
        let generator_output = generator(INPUT);
        let result = part_2(&generator_output);
        assert_eq!(result, 12);
    }
}
