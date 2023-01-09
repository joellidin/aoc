use std::collections::HashSet;

#[derive(Debug, Eq, PartialEq, Hash)]
struct Point {
    x: isize,
    y: isize,
}
impl Point {
    fn manhattan_distance(&self, location: &Point) -> usize {
        self.x.abs_diff(location.x) + self.y.abs_diff(location.y)
    }
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct Beacon {
    location: Point,
}

#[derive(Debug)]
pub struct Sensor {
    location: Point,
    beacon: Beacon,
    beacon_distance: usize,
}

impl Sensor {
    fn new(location: Point, beacon: Beacon) -> Self {
        let beacon_distance = location.manhattan_distance(&beacon.location);
        Sensor {
            location,
            beacon,
            beacon_distance,
        }
    }

    fn get_border_points(&self) -> Vec<Point> {
        let distance = self.beacon_distance + 1;
        let (sx, sy) = (self.location.x, self.location.y);
        (0..distance)
            .flat_map(|step| {
                vec![
                    Point {
                        x: sx + step as isize,
                        y: sy + step as isize - distance as isize,
                    },
                    Point {
                        x: sx - step as isize,
                        y: sy - step as isize + distance as isize,
                    },
                    Point {
                        x: sx + step as isize - distance as isize,
                        y: sy - step as isize,
                    },
                    Point {
                        x: sx - step as isize + distance as isize,
                        y: sy + step as isize,
                    },
                ]
            })
            .collect()
    }

    fn is_within_beacon_distance(&self, point: &Point) -> bool {
        self.location.manhattan_distance(point) <= self.beacon_distance
    }

    fn get_beacons(sensors: &[Sensor]) -> HashSet<&Beacon> {
        let mut unique_beacons = HashSet::new();
        sensors
            .iter()
            .map(|s| unique_beacons.insert(&s.beacon))
            .for_each(drop);
        unique_beacons
    }
}

pub fn generator(input: &str) -> Vec<Sensor> {
    input
        .trim()
        .lines()
        .map(|line| {
            let (sensor_x, line) = line
                .strip_prefix("Sensor at x=")
                .unwrap()
                .split_once(',')
                .unwrap();
            let (sensor_y, line) = line.strip_prefix(" y=").unwrap().split_once(':').unwrap();
            let (beacon_x, line) = line
                .strip_prefix(" closest beacon is at x=")
                .unwrap()
                .split_once(',')
                .unwrap();
            let beacon_y = line.strip_prefix(" y=").unwrap();
            let location = Point {
                x: sensor_x.parse().unwrap(),
                y: sensor_y.parse().unwrap(),
            };
            let beacon = Beacon {
                location: Point {
                    x: beacon_x.parse().unwrap(),
                    y: beacon_y.parse().unwrap(),
                },
            };
            Sensor::new(location, beacon)
        })
        .collect()
}

pub fn part_1(input: &[Sensor]) -> u32 {
    let beacons = Sensor::get_beacons(input);
    let y = 2_000_000;
    let mut count = 0;
    for x in -y..=3 * y {
        let position = Point { x, y };
        if input.iter().any(|s| s.is_within_beacon_distance(&position)) {
            count += 1
        }
        if beacons
            .iter()
            .any(|b| b.location.x == position.x && b.location.y == position.y)
        {
            count -= 1
        }
    }
    count
}

pub fn part_2(input: &[Sensor]) -> u64 {
    let bound = 4_000_000;
    for sensor in input {
        for p in sensor.get_border_points() {
            if p.x < 0 || p.y < 0 || p.x > bound || p.y > bound {
                continue;
            }
            if input.iter().any(|s| s.is_within_beacon_distance(&p)) {
                continue;
            }
            return p.x as u64 * 4_000_000 + p.y as u64;
        }
    }
    0
}
