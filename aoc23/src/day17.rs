use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Direction {
    North,
    South,
    East,
    West,
    None,
}

#[derive(Debug, Clone, Copy)]
pub struct Tile {
    heat_loss: u8,
    g_scores: [u16; 2],
}

fn a_star<const MIN_STEPS: usize, const MAX_STEPS: usize>(map: &mut [Vec<Tile>]) -> u16 {
    let mut open_set = BinaryHeap::new();
    let goal = Position {
        y: map.len() - 1,
        x: map[0].len() - 1,
    };

    open_set.push((Reverse(0), Position { x: 0, y: 0 }, Direction::None));
    map[0][0].g_scores = [0; 2];

    while let Some((Reverse(g_score), position, direction)) = open_set.pop() {
        if position == goal {
            return g_score;
        }
        if direction != Direction::None
            && g_score != map[position.y][position.x].g_scores[direction as usize / 2]
        {
            continue;
        }

        for new_direction in match direction {
            Direction::North | Direction::South => [Direction::East, Direction::West],
            Direction::East | Direction::West => [Direction::South, Direction::North],
            Direction::None => [Direction::East, Direction::South],
        } {
            let mut new_g_score = g_score;
            for step in 1..=MAX_STEPS {
                let (new_x, new_y) = match new_direction {
                    Direction::North if position.y > step - 1 => (position.x, position.y - step),
                    Direction::East if position.x < map[0].len() - step => {
                        (position.x + step, position.y)
                    }
                    Direction::South if position.y < map.len() - step => {
                        (position.x, position.y + step)
                    }
                    Direction::West if position.x > step - 1 => (position.x - step, position.y),
                    _ => continue,
                };

                let new_position = Position { x: new_x, y: new_y };
                let neighbour = &mut map[new_y][new_x];
                new_g_score += neighbour.heat_loss as u16;
                if step >= MIN_STEPS {
                    let old_g_score = neighbour.g_scores[new_direction as usize / 2];

                    if new_g_score < old_g_score {
                        // Found a better way to this position, in this direction
                        neighbour.g_scores[new_direction as usize / 2] = new_g_score;
                        open_set.push((Reverse(new_g_score), new_position, new_direction))
                    }
                }
            }
        }
    }
    u16::MAX
}

pub fn generator(input: &str) -> Vec<Vec<Tile>> {
    input
        .lines()
        .map(|l| {
            l.chars()
                .map(|n| Tile {
                    heat_loss: n.to_digit(10).unwrap() as u8,
                    g_scores: [u16::MAX; 2],
                })
                .collect()
        })
        .collect()
}

pub fn part_1(input: &[Vec<Tile>]) -> u16 {
    let mut grid = input.to_owned();
    a_star::<1, 3>(&mut grid)
}

pub fn part_2(input: &[Vec<Tile>]) -> u16 {
    let mut grid = input.to_owned();
    a_star::<4, 10>(&mut grid)
}
