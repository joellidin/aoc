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
struct Sensor {
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

pub fn solution() {
    let sensors = include_str!("../data/day15.txt")
        .trim()
        .split('\n')
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
        .collect::<Vec<_>>();

    let beacons = Sensor::get_beacons(&sensors);
    let y = 2_000_000;
    let mut count = 0;
    for x in -y..=3 * y {
        let position = Point { x, y };
        if sensors
            .iter()
            .any(|s| s.is_within_beacon_distance(&position))
        {
            count += 1
        }
        if beacons
            .iter()
            .any(|b| b.location.x == position.x && b.location.y == position.y)
        {
            count -= 1
        }
    }
    println!("There are {count} positions that could contain a beacon");

    let bound = 4_000_000;
    for sensor in &sensors {
        for p in sensor.get_border_points() {
            if p.x < 0 || p.y < 0 || p.x > bound || p.y > bound {
                continue;
            }
            if sensors.iter().any(|s| s.is_within_beacon_distance(&p)) {
                continue;
            }
            println!("The tuning frequency is: {}", p.x * 4000000 + p.y);
            return;
        }
    }
}
