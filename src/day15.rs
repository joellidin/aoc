use std::collections::{HashMap, HashSet};
use std::cmp;

#[derive(Debug, Eq, PartialEq, Hash)]
struct Point {
    x: isize,
    y: isize,
}

fn get_non_beacon_positions(pairs: &[(Point, Point)]) -> HashMap<isize, Vec<(isize, isize)>> {
    let mut beacons = HashSet::new();
    let mut non_beacon_min_max: HashMap<isize, Vec<_>> = HashMap::new();
    pairs
        .iter()
        .map(|(sp, bp)| {
            beacons.insert(bp);
            let manhattan_distance = sp.x.abs_diff(bp.x) + sp.y.abs_diff(bp.y);
            let mut width = manhattan_distance as isize;
            let mut dy = 0;
            while width >= 0 {
                for y in [sp.y + dy, sp.y - dy] {
                    let (start, end) = (sp.x - width as isize, sp.x + width as isize);
                    if let Some(min_max_list) = non_beacon_min_max.get_mut(&y) {
                        min_max_list.push((start, end));
                        min_max_list.sort_unstable();
                    } else {
                        non_beacon_min_max.insert(y, vec![(start, end)]);
                    }
                }
                dy += 1;
                width -= 1;
            }
        })
        .for_each(drop);
    non_beacon_min_max
}

fn get_beacons_at(row: usize, pairs: &[(Point, Point)], (min, max): (isize, isize)) -> usize {
    let mut beacons_at_row = HashSet::new();
    pairs
        .iter()
        .filter(|(_, bp)| bp.y == row as isize && bp.x >= min && bp.x <= max)
        .map(|(_, bp)| beacons_at_row.insert(bp))
        .for_each(drop);
    beacons_at_row.iter().count()
}

pub fn solution() {
    let pairs = std::fs::read_to_string("data/day15.txt")
        .unwrap()
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
            (
                Point {
                    x: sensor_x.parse().unwrap(),
                    y: sensor_y.parse().unwrap(),
                },
                Point {
                    x: beacon_x.parse().unwrap(),
                    y: beacon_y.parse().unwrap(),
                },
            )
        })
        .collect::<Vec<_>>();

    let non_beacon_min_max = get_non_beacon_positions(&pairs);
    let row = 2_000_000;
    let min_max_list = non_beacon_min_max.get(&row).unwrap();
    let (mut min, mut max) = (isize::MAX, isize::MIN);
    for (mi, ma) in min_max_list.iter() {
        if mi < &min {
            min = *mi;
        }
        if ma > &max {
            max = *ma;
        }
    }
    println!(
        "There are {} positions that could contain a beacon",
        (max + 1) - min - (get_beacons_at(row as usize, &pairs, (min, max)) as isize)
    );

    let (mut xx, mut yy) = (0, 0);
    for y in 0..4_000_000 {
        let start_end = non_beacon_min_max.get(&y).unwrap();
        let mut range_stack = RangeStack { ranges: Vec::new() };
        for (start, end) in start_end.iter() {
            range_stack.add(&Range {
                start: *start,
                end: *end,
            });
        }
        if range_stack.ranges.len() > 1 {
            yy = y;
            xx = range_stack.ranges[0].end + 1;
        }
    }
    println!("The tuning frequency is: {}", xx * 4000000 + yy);
}

#[derive(Debug, Copy, Clone)]
struct Range {
    start: isize,
    end: isize,
}

impl Range {
    fn overlaps(&self, other: &Range) -> bool {
        (other.start >= self.start && other.start <= self.end)
            || (other.end >= self.start && other.end <= self.end)
    }

    fn merge(&mut self, other: &Range) {
        self.start = cmp::min(self.start, other.start);
        self.end = cmp::max(self.end, other.end);
    }
}

#[derive(Debug, Clone)]
struct RangeStack {
    ranges: Vec<Range>,
}

impl RangeStack {
    fn add(&mut self, range: &Range) {
        if let Some(last) = self.ranges.last_mut() {
            if last.overlaps(range) {
                last.merge(range);
                return;
            }
        }
        self.ranges.push(*range);
    }
}
