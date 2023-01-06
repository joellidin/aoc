use std::collections::{BinaryHeap, HashMap, HashSet};

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug, PartialOrd, Ord)]
struct Point {
    x: isize,
    y: isize,
    z: isize,
}

#[derive(Eq, Ord, PartialEq, PartialOrd)]
struct State {
    cost: isize,
    point: Point,
}

impl Point {
    fn neighbours(&self) -> HashSet<Point> {
        HashSet::from([
            Point {
                x: self.x + 1,
                ..*self
            },
            Point {
                x: self.x - 1,
                ..*self
            },
            Point {
                y: self.y + 1,
                ..*self
            },
            Point {
                y: self.y - 1,
                ..*self
            },
            Point {
                z: self.z + 1,
                ..*self
            },
            Point {
                z: self.z - 1,
                ..*self
            },
        ])
    }
    fn sides_touching(&self, other: &HashSet<Point>) -> usize {
        self.neighbours()
            .iter()
            .filter(|p| other.contains(p))
            .count()
    }
}

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
struct BoundingBox {
    x_max: isize,
    x_min: isize,
    y_max: isize,
    y_min: isize,
    z_max: isize,
    z_min: isize,
}

impl BoundingBox {
    fn new(points: &HashSet<Point>) -> BoundingBox {
        BoundingBox {
            x_max: points
                .iter()
                .fold(isize::MIN, |acc, a| std::cmp::max(acc, a.x))
                + 1,
            x_min: points
                .iter()
                .fold(isize::MAX, |acc, a| std::cmp::min(acc, a.x))
                - 1,
            y_max: points
                .iter()
                .fold(isize::MIN, |acc, a| std::cmp::max(acc, a.y))
                + 1,
            y_min: points
                .iter()
                .fold(isize::MAX, |acc, a| std::cmp::min(acc, a.y))
                - 1,
            z_max: points
                .iter()
                .fold(isize::MIN, |acc, a| std::cmp::max(acc, a.z))
                + 1,
            z_min: points
                .iter()
                .fold(isize::MAX, |acc, a| std::cmp::min(acc, a.z))
                - 1,
        }
    }

    fn contains(&self, p: Point) -> bool {
        p.x >= self.x_min
            && p.x <= self.x_max
            && p.y >= self.y_min
            && p.y <= self.y_max
            && p.z >= self.z_min
            && p.z <= self.z_max
    }

    fn adjacent_points_outside(self, solid: &HashSet<Point>) -> HashSet<Point> {
        let mut res = HashSet::new();
        let mut distances: HashMap<Point, isize> = HashMap::new();
        let mut work = BinaryHeap::new();
        work.push(State {
            cost: 0,
            point: Point {
                x: self.x_min,
                y: self.y_min,
                z: self.z_min,
            },
        });
        while let Some(State { cost, point }) = work.pop() {
            if !solid.contains(&point) && !res.contains(&point) && self.contains(point) {
                res.insert(point);
                point.neighbours().iter().for_each(|n| {
                    let new_cost = cost + 1;
                    let entry = distances.entry(*n).or_insert(isize::MAX);
                    if new_cost < *entry {
                        *entry = new_cost;
                        work.push(State {
                            cost: new_cost,
                            point: *n,
                        });
                    }
                });
            }
        }
        res
    }
}

fn parse_points() -> HashSet<Point> {
    let mut points = HashSet::new();
    include_str!("../data/day18.txt")
        .trim()
        .split('\n')
        .map(|line| {
            let nums: Vec<isize> = line
                .split(',')
                .map(|x| x.parse::<isize>().unwrap())
                .collect();
            points.insert(Point {
                x: nums[0],
                y: nums[1],
                z: nums[2],
            })
        })
        .for_each(drop);
    points
}

pub fn solution() {
    let points = parse_points();
    let exposed_sides = points
        .iter()
        .fold(0, |acc, p| (acc + 6) - p.sides_touching(&points));
    println!("Surface area of all points: {exposed_sides}");
    let points = parse_points();
    let outer = BoundingBox::new(&points).adjacent_points_outside(&points);
    let exposed_sides2: usize = points.iter().map(|p| p.sides_touching(&outer)).sum();
    println!("Exterior surface area: {exposed_sides2}");
}
