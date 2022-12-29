use std::str::FromStr;

struct Reindeer {
    speed: usize,
    fly_time: usize,
    rest_time: usize,
}

impl FromStr for Reindeer {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        let speed = parts.nth(3).unwrap().parse().unwrap();
        let fly_time = parts.nth(2).unwrap().parse().unwrap();
        let rest_time = parts.nth(6).unwrap().parse().unwrap();
        Ok(Reindeer {
            speed,
            fly_time,
            rest_time,
        })
    }
}

impl Reindeer {
    fn distance(&self, time: usize) -> usize {
        let cycle_time = self.fly_time + self.rest_time;
        let cycles = time / cycle_time;
        let remaining_time = time % cycle_time;
        let remaining_distance = remaining_time.min(self.fly_time) * self.speed;
        cycles * self.fly_time * self.speed + remaining_distance
    }
}

fn get_points(reindeers: &[Reindeer], time: usize) -> Vec<usize> {
    let mut points = vec![0; reindeers.len()];
    (1..=time).for_each(|t| {
        let max_distance = reindeers.iter().map(|r| r.distance(t)).max().unwrap();
        reindeers
            .iter()
            .enumerate()
            .filter(|(_, r)| r.distance(t) == max_distance)
            .for_each(|(i, _)| points[i] += 1);
    });
    points
}

pub fn solution() {
    let input = std::fs::read_to_string("data/day14.txt").unwrap();
    println!(
        "Distance after 2503 seconds: {}",
        input
            .lines()
            .map(|line| line.parse::<Reindeer>().unwrap())
            .map(|reindeer| reindeer.distance(2503))
            .max()
            .unwrap()
    );

    println!(
        "Points after 2503 seconds: {}",
        get_points(
            &input
                .lines()
                .map(|line| line.parse::<Reindeer>().unwrap())
                .collect::<Vec<_>>(),
            2503
        )
        .iter()
        .max()
        .unwrap()
    );
}
