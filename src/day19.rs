use std::collections::{HashSet, VecDeque};
use std::str::FromStr;

#[derive(Debug)]
struct Blueprint {
    ore_cost: usize,
    clay_cost: usize,
    obsidian_cost: (usize, usize),
    geode_cost: (usize, usize),
}

impl FromStr for Blueprint {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let nums = s
            .trim()
            .split(' ')
            .filter_map(|x| x.parse::<usize>().ok())
            .collect::<Vec<_>>();
        Ok(Blueprint {
            ore_cost: nums[0],
            clay_cost: nums[1],
            obsidian_cost: (nums[2], nums[3]),
            geode_cost: (nums[4], nums[5]),
        })
    }
}
impl Blueprint {
    fn can_build_ore(&self, items: &Backpack) -> bool {
        items.ore >= self.ore_cost
    }
    fn can_build_clay(&self, items: &Backpack) -> bool {
        items.ore >= self.clay_cost
    }
    fn can_build_obsidian(&self, items: &Backpack) -> bool {
        let (ore_cost, clay_cost) = self.obsidian_cost;
        items.ore >= ore_cost && items.clay >= clay_cost
    }
    fn can_build_geode(&self, items: &Backpack) -> bool {
        let (ore_cost, obsidian_cost) = self.geode_cost;
        items.ore >= ore_cost && items.obsidian >= obsidian_cost
    }
}

#[derive(Default, Hash, PartialEq, Eq, Clone, Copy)]
struct Backpack {
    ore: usize,
    clay: usize,
    obsidian: usize,
    geode: usize,
}

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
struct Robots {
    ore: usize,
    clay: usize,
    obsidian: usize,
    geode: usize,
}

impl Robots {
    fn generate_material(&self, backpack: &mut Backpack) {
        backpack.ore += self.ore;
        backpack.clay += self.clay;
        backpack.obsidian += self.obsidian;
        backpack.geode += self.geode;
    }
}

impl Default for Robots {
    fn default() -> Self {
        Robots {
            ore: 1,
            clay: 0,
            obsidian: 0,
            geode: 0,
        }
    }
}

fn simulate(blueprint: &Blueprint, time: usize) -> usize {
    let mut state_map = HashSet::new();
    let backpack = Backpack::default();
    let robots = Robots::default();
    let t = 0;
    let mut q = VecDeque::new();
    q.push_back((backpack, robots, t));
    let mut max_geodes = 0;
    let (max_ore_robots, max_obsidian_robot) = blueprint.geode_cost;
    while let Some((backpack, robots, t)) = q.pop_front() {
        if t >= time
            || state_map.contains(&(backpack, robots))
            || (robots.ore >= max_ore_robots && robots.obsidian >= max_obsidian_robot)
            || backpack.geode < max_geodes
        {
            max_geodes = std::cmp::max(max_geodes, backpack.geode);
            continue;
        }
        state_map.insert((backpack, robots));
        if blueprint.can_build_geode(&backpack) {
            let (ore_cost, obsidian_cost) = blueprint.geode_cost;
            let mut new_backpack = backpack;
            let mut new_robots = robots;
            new_backpack.ore -= ore_cost;
            new_backpack.obsidian -= obsidian_cost;
            new_robots.generate_material(&mut new_backpack);

            new_robots.geode += 1;
            q.push_back((new_backpack, new_robots, t + 1));
            continue;
        }

        if blueprint.can_build_ore(&backpack)
            && robots.ore
                < (blueprint.clay_cost + blueprint.geode_cost.0 + blueprint.obsidian_cost.0)
        {
            let mut new_backpack = backpack;
            let mut new_robots = robots;
            new_backpack.ore -= blueprint.ore_cost;
            new_robots.generate_material(&mut new_backpack);

            new_robots.ore += 1;
            if new_backpack.geode >= max_geodes {
                q.push_back((new_backpack, new_robots, t + 1));
            }
        }
        if blueprint.can_build_clay(&backpack) && robots.clay < blueprint.obsidian_cost.1 {
            let mut new_backpack = backpack;
            let mut new_robots = robots;
            new_backpack.ore -= blueprint.clay_cost;
            new_robots.generate_material(&mut new_backpack);

            new_robots.clay += 1;
            if new_backpack.geode >= max_geodes {
                q.push_back((new_backpack, new_robots, t + 1))
            }
        }
        if blueprint.can_build_obsidian(&backpack) && robots.obsidian < blueprint.geode_cost.1 {
            let (ore_cost, clay_cost) = blueprint.obsidian_cost;
            let mut new_backpack = backpack;
            let mut new_robots = robots;
            new_backpack.ore -= ore_cost;
            new_backpack.clay -= clay_cost;
            new_robots.generate_material(&mut new_backpack);

            new_robots.obsidian += 1;
            if new_backpack.geode >= max_geodes {
                q.push_back((new_backpack, new_robots, t + 1))
            }
        }

        let mut new_backpack = backpack;
        let new_robots = robots;
        new_robots.generate_material(&mut new_backpack);
        if new_backpack.geode >= max_geodes {
            q.push_back((new_backpack, new_robots, t + 1))
        }
    }
    max_geodes
}

pub fn solution() {
    let blueprints = std::fs::read_to_string("data/day19.txt")
        .unwrap()
        .trim()
        .split('\n')
        .map(|bp| bp.parse::<Blueprint>().unwrap())
        .collect::<Vec<_>>();

    println!(
        "Quality level of all blueprints: {}",
        blueprints
            .iter()
            .enumerate()
            .fold(0, |acc, (i, bp)| { acc + (i + 1) * simulate(bp, 24) })
    );

    println!(
        "Quality level of all blueprints: {}",
        blueprints
            .iter()
            .take(3)
            .fold(1, |factor, bp| { factor * simulate(bp, 32) })
    );
}
