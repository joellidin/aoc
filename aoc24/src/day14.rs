use aoc_utils::prelude::*;
use std::collections::{HashMap, HashSet};

pub struct Robot {
    pos: Vec2<i32>,
    vel: Vec2<i32>,
}

pub fn generator(input: &str) -> Vec<Robot> {
    input
        .lines()
        .map(|line| {
            let &[p_x, p_y, v_x, v_y, ..] = extract_integers::<i32>(line).as_slice() else {
                panic!("Could not find numbers")
            };

            Robot {
                pos: (p_x, p_y).into(),
                vel: (v_x, v_y).into(),
            }
        })
        .collect()
}

#[allow(dead_code)]
fn print_robots(points: &[Vec2<i32>], width: i32, height: i32) {
    let mut grid = vec![vec!['.'; width as usize]; height as usize];
    let mut counts: HashMap<Vec2<i32>, usize> = HashMap::new();

    // Count robots at each point
    for point in points {
        *counts.entry(*point).or_insert(0) += 1;
    }

    // Populate grid based on counts
    for (point, count) in counts {
        grid[point.y as usize][point.x as usize] =
            std::char::from_digit(count as u32, 10).unwrap_or('.');
    }

    // Print the grid
    for row in grid {
        println!("{}", row.iter().collect::<String>());
    }
}

fn wrap_coord(value: i32, max: i32) -> i32 {
    let mut val = value % max;
    if val < 0 {
        val += max;
    }
    val
}
fn elapse(robots: &[Robot], time: u32, width: i32, height: i32) -> Vec<Vec2<i32>> {
    robots
        .iter()
        .map(|robot| {
            let x = wrap_coord(robot.pos.x + robot.vel.x * time as i32, width);
            let y = wrap_coord(robot.pos.y + robot.vel.y * time as i32, height);
            Vec2 { x, y }
        })
        .collect()
}

fn find_quadrants(robots: &[Vec2<i32>], width: i32, height: i32) -> (u32, u32, u32, u32) {
    robots.iter().fold((0, 0, 0, 0), |(q1, q2, q3, q4), p| {
        if p.x < width / 2 && p.y < height / 2 {
            (1 + q1, q2, q3, q4)
        } else if p.x > width / 2 && p.y < height / 2 {
            (q1, 1 + q2, q3, q4)
        } else if p.x < width / 2 && p.y > height / 2 {
            (q1, q2, 1 + q3, q4)
        } else if p.x > width / 2 && p.y > height / 2 {
            (q1, q2, q3, 1 + q4)
        } else {
            (q1, q2, q3, q4)
        }
    })
}

const WIDTH: i32 = 101;
const HEIGHT: i32 = 103;

pub fn part_1(input: &[Robot]) -> u32 {
    let robot_positions = elapse(input, 100, WIDTH, HEIGHT);
    let q = find_quadrants(&robot_positions, WIDTH, HEIGHT);
    q.0 * q.1 * q.2 * q.3
}

pub fn part_2(input: &[Robot]) -> u32 {
    (1..)
        .find_map(|i| {
            let final_positions = elapse(input, i, WIDTH, HEIGHT);
            let positions_set: HashSet<_> = final_positions.iter().copied().collect();

            positions_set
                .iter()
                .find(|&&point| {
                    (1..7).all(|j| {
                        let new_left_point = (point.x - j, point.y + j).into();
                        let new_right_point = (point.x + j, point.y + j).into();
                        positions_set.contains(&new_left_point)
                            && positions_set.contains(&new_right_point)
                    })
                })
                .map(|_| i)
        })
        .expect("No suitable time found")
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
"#;

    #[test]
    fn part_1_test() {
        let generator_output = generator(INPUT);
        let robot_positions = elapse(&generator_output, 100, 11, 7);
        assert_eq!(robot_positions.len(), 12);
        let q = find_quadrants(&robot_positions, 11, 7);
        assert_eq!(q.0 * q.1 * q.2 * q.3, 12);
    }
}
