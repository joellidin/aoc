use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: isize,
    y: isize,
}

#[derive(Debug, PartialEq, Eq)]
enum Tile {
    Sand,
    Rock,
}

#[derive(PartialEq, Eq)]
struct Cave {
    map: HashMap<Point, Tile>,
    source: Point,
    floor: usize,
    sand_counter: usize,
}

impl FromStr for Point {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let point: Vec<isize> = s.split(',').map(|s| s.trim().parse().unwrap()).collect();
        Ok(Point {
            x: point[0],
            y: point[1],
        })
    }
}

impl Cave {
    fn new(scan: &str, source: Point) -> Cave {
        let mut map = HashMap::new();
        let mut highest_y = 0;
        for line in scan.lines() {
            let mut iter = line.split("->");
            let first_point: Point = iter.next().unwrap().parse().unwrap();
            let mut prev_point = first_point;
            for point in line.split("->") {
                let new_point: Point = point.parse().unwrap();
                if new_point.x == prev_point.x {
                    // Vertical line
                    let range = if prev_point.y < new_point.y {
                        prev_point.y..=new_point.y
                    } else {
                        new_point.y..=prev_point.y
                    };
                    for i in range {
                        if i > highest_y {
                            highest_y = i;
                        }
                        map.insert(
                            Point {
                                x: prev_point.x,
                                y: i,
                            },
                            Tile::Rock,
                        );
                    }
                } else {
                    // Horizontal line
                    let range = if prev_point.x < new_point.x {
                        prev_point.x..=new_point.x
                    } else {
                        new_point.x..=prev_point.x
                    };
                    for i in range {
                        map.insert(
                            Point {
                                x: i,
                                y: prev_point.y,
                            },
                            Tile::Rock,
                        );
                    }
                }
                // Update current coordinates
                prev_point = new_point;
            }
        }
        let floor = (highest_y + 2) as usize;
        Cave {
            map,
            source,
            floor,
            sand_counter: 0,
        }
    }

    fn simulate(&mut self) {
        let mut current_point = self.source;
        let mut stack = vec![current_point];
        loop {
            if current_point.y >= self.floor as isize
                || self.map.get(&self.source) == Some(&Tile::Sand)
            {
                break;
            }
            match self.get_move(current_point) {
                Some((dx, dy)) => {
                    stack.push(current_point);
                    current_point = Point {
                        x: current_point.x + dx,
                        y: current_point.y + dy,
                    };
                }
                None => {
                    self.map.insert(current_point, Tile::Sand);
                    self.sand_counter += 1;
                    current_point = stack.pop().unwrap();
                }
            };
        }
    }

    fn get_move(&self, point: Point) -> Option<(isize, isize)> {
        for (dx, dy) in [(0, 1), (-1, 1), (1, 1)] {
            let new_point = Point {
                x: point.x + dx,
                y: point.y + dy,
            };
            if self.map.get(&new_point).is_none() {
                return Some((dx, dy));
            };
        }
        None
    }

    fn add_floor(&mut self) {
        for i in 500 - self.floor..=500 + self.floor {
            self.map.insert(
                Point {
                    x: i as isize,
                    y: self.floor as isize,
                },
                Tile::Rock,
            );
        }
    }
}

impl std::fmt::Debug for Cave {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut string = String::new();
        for j in 0..self.floor as isize + 2 {
            for i in -(self.floor as isize) - 3..=self.floor as isize + 3 {
                let point = Point { x: i + 500, y: j };
                if point == self.source && self.map.get(&point).is_none() {
                    string.push('+');
                    continue;
                }
                string.push(match self.map.get(&point) {
                    Some(Tile::Sand) => 'o',
                    Some(Tile::Rock) => '#',
                    _ => '.',
                })
            }
            string.push('\n');
        }
        write!(f, "{string}")
    }
}

pub fn solution() {
    let input = include_str!("../data/day14.txt");
    let mut cave = Cave::new(input, Point { x: 500, y: 0 });
    cave.simulate();
    println!("Sand counter without floor: {}", cave.sand_counter);
    cave.add_floor();
    cave.simulate();
    println!("Sand counter with floor: {}", cave.sand_counter);
}
