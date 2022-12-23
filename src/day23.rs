use std::{
    collections::{HashMap, HashSet},
    iter::successors,
};

#[derive(Hash, Clone, Copy, Debug, PartialEq, Eq)]
struct Pos {
    x: isize,
    y: isize,
}

#[derive(Debug, Clone, Copy)]
enum Dir {
    North,
    South,
    West,
    East,
}

impl From<usize> for Dir {
    fn from(n: usize) -> Self {
        match n {
            0 => Dir::North,
            1 => Dir::South,
            2 => Dir::West,
            3 => Dir::East,
            _ => panic!("Invalid direction: {n}"),
        }
    }
}

impl Pos {
    fn has_neighbours(&self, positions: &HashSet<Pos>) -> bool {
        (-1..=1).any(|x| {
            (-1..=1).any(|y| {
                if x == 0 && y == 0 {
                    false
                } else {
                    positions.contains(&Pos {
                        x: self.x + x,
                        y: self.y + y,
                    })
                }
            })
        })
    }

    fn is_blocked(&self, positions: &HashSet<Pos>, direction: &Dir) -> bool {
        match direction {
            Dir::North => (-1..=1).any(|x| {
                positions.contains(&Pos {
                    x: self.x + x,
                    y: self.y - 1,
                })
            }),
            Dir::South => (-1..=1).any(|x| {
                positions.contains(&Pos {
                    x: self.x + x,
                    y: self.y + 1,
                })
            }),
            Dir::West => (-1..=1).any(|y| {
                positions.contains(&Pos {
                    x: self.x - 1,
                    y: self.y + y,
                })
            }),
            Dir::East => (-1..=1).any(|y| {
                positions.contains(&Pos {
                    x: self.x + 1,
                    y: self.y + y,
                })
            }),
        }
    }

    fn step(&self, direction: &Dir) -> Pos {
        match direction {
            Dir::North => Pos {
                x: self.x,
                y: self.y - 1,
            },
            Dir::South => Pos {
                x: self.x,
                y: self.y + 1,
            },
            Dir::West => Pos {
                x: self.x - 1,
                y: self.y,
            },
            Dir::East => Pos {
                x: self.x + 1,
                y: self.y,
            },
        }
    }
}

fn generate_proposals(positions: &HashSet<Pos>, proposals: &mut HashMap<Pos, Pos>, direction: Dir) {
    let mut dupes = HashSet::new();
    positions
        .iter()
        .filter(|pos| pos.has_neighbours(positions))
        .for_each(|pos| {
            let dirs = successors(Some(direction as usize), |&dir| Some((dir + 1) % 4));
            for dir in dirs.take(4).map(Dir::from) {
                if !pos.is_blocked(positions, &dir) {
                    if proposals.insert(pos.step(&dir), *pos).is_some() {
                        dupes.insert(pos.step(&dir));
                    }
                    break;
                }
            }
        });
    dupes.iter().for_each(|pos| {
        proposals.remove(pos);
    });
}

fn update_positions(positions: &mut HashSet<Pos>, proposals: &mut HashMap<Pos, Pos>) -> bool {
    let mut changed = false;
    proposals
        .drain()
        .map(|(new_pos, old_position)| {
            positions.remove(&old_position);
            positions.insert(new_pos);
            changed = true;
        })
        .for_each(drop);
    changed
}

fn simulate(max_iterations: Option<usize>, positions: &mut HashSet<Pos>, mut dir: Dir) -> usize {
    let mut proposals = HashMap::new();
    let mut num = 1;
    generate_proposals(positions, &mut proposals, dir);
    while update_positions(positions, &mut proposals) {
        if num == max_iterations.unwrap_or(usize::MAX) {
            break;
        }
        dir = Dir::from((dir as usize + 1) % 4);
        generate_proposals(positions, &mut proposals, dir);
        num += 1;
    }
    num
}

fn get_bounding_box(positions: &HashSet<Pos>) -> (Pos, Pos) {
    let mut min_x = std::isize::MAX;
    let mut min_y = std::isize::MAX;
    let mut max_x = std::isize::MIN;
    let mut max_y = std::isize::MIN;
    positions.iter().for_each(|pos| {
        if pos.x < min_x {
            min_x = pos.x;
        }
        if pos.y < min_y {
            min_y = pos.y;
        }
        if pos.x > max_x {
            max_x = pos.x;
        }
        if pos.y > max_y {
            max_y = pos.y;
        }
    });
    (Pos { x: min_x, y: min_y }, Pos { x: max_x, y: max_y })
}

#[allow(dead_code)]
fn print_positions(positions: &HashSet<Pos>) {
    let (min_pos, max_pos) = get_bounding_box(positions);
    for y in min_pos.y..=max_pos.y {
        for x in min_pos.x..=max_pos.x {
            if positions.contains(&Pos { x, y }) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn get_empty_positions(positions: &HashSet<Pos>) -> usize {
    let (lower_left, upper_right) = get_bounding_box(positions);
    (lower_left.x..=upper_right.x)
        .map(|x| {
            (lower_left.y..=upper_right.y)
                .filter(|y| !positions.contains(&Pos { x, y: *y }))
                .count()
        })
        .sum()
}

fn get_elves_positions(input: &str) -> HashSet<Pos> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| c == &'#')
                .map(move |(x, _)| Pos {
                    x: x as isize,
                    y: y as isize,
                })
        })
        .collect()
}

pub fn solution() {
    let input = std::fs::read_to_string("data/day23.txt").unwrap();
    let mut elves = get_elves_positions(&input);
    simulate(Some(10), &mut elves, Dir::North);
    println!(
        "Empty positions after 10 iterations: {}",
        get_empty_positions(&elves)
    );
    let mut elves = get_elves_positions(&input);
    println!(
        "Number iterations before steady state: {}",
        simulate(None, &mut elves, Dir::North)
    );
}
