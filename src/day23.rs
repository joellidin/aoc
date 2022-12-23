use std::collections::HashMap;

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

impl Dir {
    const DIRS: &[Dir] = &[Dir::North, Dir::South, Dir::West, Dir::East];
}

impl Pos {
    fn has_neighbours<T>(&self, positions: &HashMap<Pos, T>) -> bool {
        (-1..=1).any(|x| {
            (-1..=1).any(|y| {
                if x == 0 && y == 0 {
                    false
                } else {
                    positions.contains_key(&Pos {
                        x: self.x + x,
                        y: self.y + y,
                    })
                }
            })
        })
    }

    fn is_blocked<T>(&self, positions: &HashMap<Pos, T>, direction: &Dir) -> bool {
        match direction {
            Dir::North => (-1..=1).any(|x| {
                positions.contains_key(&Pos {
                    x: self.x + x,
                    y: self.y - 1,
                })
            }),
            Dir::South => (-1..=1).any(|x| {
                positions.contains_key(&Pos {
                    x: self.x + x,
                    y: self.y + 1,
                })
            }),
            Dir::West => (-1..=1).any(|y| {
                positions.contains_key(&Pos {
                    x: self.x - 1,
                    y: self.y + y,
                })
            }),
            Dir::East => (-1..=1).any(|y| {
                positions.contains_key(&Pos {
                    x: self.x + 1,
                    y: self.y + y,
                })
            }),
        }
    }
}

fn generate_proposals(positions: &mut HashMap<Pos, Option<Pos>>, direction: usize) {
    let copy_pos = positions.clone();
    positions
        .iter_mut()
        .map(|(pos, prop)| {
            *prop = None;
            (pos, prop)
        })
        .filter(|(pos, _)| pos.has_neighbours(&copy_pos))
        .for_each(|(pos, prop)| {
            *prop = None;
            for i in 0..4 {
                let dir = Dir::DIRS[(direction + i) % 4];
                if !pos.is_blocked(&copy_pos, &dir) {
                    match dir {
                        Dir::North => {
                            *prop = Some(Pos {
                                x: pos.x,
                                y: pos.y - 1,
                            });
                        }
                        Dir::South => {
                            *prop = Some(Pos {
                                x: pos.x,
                                y: pos.y + 1,
                            });
                        }
                        Dir::West => {
                            *prop = Some(Pos {
                                x: pos.x - 1,
                                y: pos.y,
                            });
                        }
                        Dir::East => {
                            *prop = Some(Pos {
                                x: pos.x + 1,
                                y: pos.y,
                            });
                        }
                    }
                    break;
                }
            }
        });
}

fn is_proposal_unique(proposal: &Pos, positions: &HashMap<Pos, Option<Pos>>) -> bool {
    let mut unique = true;
    if positions
        .values()
        .filter(|p| *p == &Some(*proposal))
        .count()
        > 1
    {
        unique = false;
    }
    unique
}

fn update_positions(positions: &mut HashMap<Pos, Option<Pos>>) -> bool {
    let copy_pos = positions.clone();
    let mut changed = false;
    copy_pos.iter().for_each(|(pos, prop)| {
        if let Some(proposal) = *prop {
            if is_proposal_unique(&proposal, &copy_pos) {
                positions.remove(pos);
                positions.insert(proposal, None);
                changed = true;
            }
        }
    });
    changed
}

fn simulate(i: usize, positions: &mut HashMap<Pos, Option<Pos>>, mut dir: usize) -> usize {
    for num in 0..i {
        generate_proposals(positions, dir);
        let changed = update_positions(positions);
        dir = (dir + 1) % 4;
        if !changed {
            return num + 1;
        }
    }
    i
}

fn get_bounding_box<T>(positions: &HashMap<Pos, T>) -> (Pos, Pos) {
    let mut min_x = std::isize::MAX;
    let mut min_y = std::isize::MAX;
    let mut max_x = std::isize::MIN;
    let mut max_y = std::isize::MIN;
    positions.keys().for_each(|pos| {
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
fn print_positions<T>(positions: &HashMap<Pos, T>) {
    let (min_pos, max_pos) = get_bounding_box(positions);
    for y in min_pos.y..=max_pos.y {
        for x in min_pos.x..=max_pos.x {
            if positions.contains_key(&Pos { x, y }) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn get_empty_positions<T>(positions: &HashMap<Pos, T>) -> usize {
    let (lower_left, upper_right) = get_bounding_box(positions);
    let mut res = 0;
    for x in lower_left.x..=upper_right.x {
        for y in lower_left.y..=upper_right.y {
            if !positions.contains_key(&Pos { x, y }) {
                res += 1;
            }
        }
    }
    res
}

fn get_elves_positions(input: &str) -> HashMap<Pos, Option<Pos>> {
    let mut positions = HashMap::new();
    input.lines().enumerate().for_each(|(y, line)| {
        line.chars()
            .enumerate()
            .filter(|(_, c)| c == &'#')
            .for_each(|(x, _)| {
                positions.insert(
                    Pos {
                        x: x as isize,
                        y: y as isize,
                    },
                    None,
                );
            });
    });
    positions
}

pub fn solution() {
    let input = std::fs::read_to_string("data/day23.txt").unwrap();
    let mut elves = get_elves_positions(&input);
    simulate(10, &mut elves, 0);
    println!(
        "Empty positions after 10 iterations: {}",
        get_empty_positions(&elves)
    );
    let mut elves = get_elves_positions(&input);
    println!(
        "Number iterations before steady state: {}",
        simulate(usize::MAX, &mut elves, 0)
    );
}
