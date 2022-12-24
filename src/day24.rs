use std::{collections::HashSet, fmt::Debug};

#[derive(Clone)]
struct Grid {
    width: isize,
    height: isize,
    start: Pos,
    goal: Pos,
    blizzard: Vec<(Pos, Pos)>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    x: isize,
    y: isize,
}

impl Debug for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;
        for x in 0..self.width {
            if self.start.x == x {
                write!(f, ".")?;
            } else {
                write!(f, "#")?;
            }
        }
        writeln!(f)?;
        for y in 1..self.height - 1 {
            write!(f, "#")?;
            for x in 1..self.width - 1 {
                let pos = Pos { x, y };
                let count = self.blizzard.iter().filter(|(p, _)| *p == pos).count();
                if count > 1 {
                    write!(f, "{count}")?;
                } else if self.blizzard.contains(&(pos, Pos { x: 1, y: 0 })) {
                    write!(f, ">")?;
                } else if self.blizzard.contains(&(pos, Pos { x: 0, y: 1 })) {
                    write!(f, "v")?;
                } else if self.blizzard.contains(&(pos, Pos { x: -1, y: 0 })) {
                    write!(f, "<")?;
                } else if self.blizzard.contains(&(pos, Pos { x: 0, y: -1 })) {
                    write!(f, "^")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f, "#")?;
        }
        for x in 0..self.width {
            if self.goal.x == x {
                write!(f, ".")?;
            } else {
                write!(f, "#")?;
            }
        }
        Ok(())
    }
}

impl Grid {
    fn new(input: &str) -> Self {
        let height = input.lines().count() as isize;
        let width = input.lines().next().unwrap().len() as isize;
        let start = Pos {
            x: input
                .lines()
                .next()
                .map(|line| line.find('.').unwrap())
                .unwrap() as isize,
            y: 0,
        };
        let goal = Pos {
            x: input
                .lines()
                .rev()
                .next()
                .map(|line| line.find('.').unwrap())
                .unwrap() as isize,
            y: height - 1,
        };
        let mut blizzard = Vec::new();
        input.lines().enumerate().for_each(|(y, line)| {
            line.chars().enumerate().for_each(|(x, c)| {
                let (x, y) = (x as isize, y as isize);
                match c {
                    '^' => blizzard.push((Pos { x, y }, Pos { x: 0, y: -1 })),
                    'v' => blizzard.push((Pos { x, y }, Pos { x: 0, y: 1 })),
                    '<' => blizzard.push((Pos { x, y }, Pos { x: -1, y: 0 })),
                    '>' => blizzard.push((Pos { x, y }, Pos { x: 1, y: 0 })),
                    _ => (),
                }
            })
        });
        Grid {
            width,
            height,
            start,
            goal,
            blizzard,
        }
    }

    fn update(&mut self) {
        // Simulate blizzards for this turn
        for (bliz, dir) in self.blizzard.iter_mut() {
            bliz.x = (bliz.x + dir.x - 1).rem_euclid(self.width - 2) + 1;
            bliz.y = (bliz.y + dir.y - 1).rem_euclid(self.height - 2) + 1;
        }
    }

    fn reverse_update(&mut self) {
        for (bliz, dir) in self.blizzard.iter_mut() {
            bliz.x = (bliz.x - dir.x - 1).rem_euclid(self.width - 2) + 1;
            bliz.y = (bliz.y - dir.y - 1).rem_euclid(self.height - 2) + 1;
        }
    }

    fn is_inside(&self, pos: Pos) -> bool {
        (pos.x > 0 && pos.x < self.width - 1 && pos.y > 0 && pos.y < self.height - 1)
            || (self.start == pos || self.goal == pos)
    }

    fn is_blizzard(&self, pos: Pos) -> bool {
        self.blizzard.iter().any(|(p, _)| *p == pos)
    }

    fn valid_positions(&self, pos: Pos) -> Vec<Pos> {
        let mut valid_positions = Vec::new();
        let dirs = [
            Pos { x: 0, y: -1 },
            Pos { x: 0, y: 1 },
            Pos { x: -1, y: 0 },
            Pos { x: 1, y: 0 },
            Pos { x: 0, y: 0 },
        ];
        for dir in &dirs {
            let new_pos = Pos {
                x: pos.x + dir.x,
                y: pos.y + dir.y,
            };
            if !self.is_inside(new_pos) || self.is_blizzard(new_pos) {
                continue;
            }
            valid_positions.push(new_pos);
        }
        valid_positions
    }
}

fn bfs(grid: &mut Grid) -> usize {
    let mut dist = 0;
    let mut choices = HashSet::from_iter(vec![grid.start].into_iter());
    'search: loop {
        grid.update();
        let mut new_choices = HashSet::new();
        for pos in choices {
            if pos == grid.goal {
                grid.reverse_update();
                break 'search;
            }

            for valid_pos in grid.valid_positions(pos) {
                new_choices.insert(valid_pos);
            }
        }
        choices = new_choices;
        dist += 1;
    }
    dist
}

pub fn solution() {
    let input = std::fs::read_to_string("data/day24.txt").unwrap();
    let mut grid = Grid::new(&input);
    let first = bfs(&mut grid);
    std::mem::swap(&mut grid.start, &mut grid.goal);
    let second = bfs(&mut grid);
    std::mem::swap(&mut grid.start, &mut grid.goal);
    let third = bfs(&mut grid);
    println!("Fewest minutes to get to the goal and avoid the blizzards {first}");
    println!(
        "Fewest minutes to go to the goal, back and to the goal again: {}",
        first + second + third
    );
}
