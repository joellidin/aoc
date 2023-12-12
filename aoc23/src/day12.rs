use std::error::Error;
use std::fmt;
use std::str::FromStr;

#[derive(Debug)]
pub enum SpringsError {
    UnknownCharacter(String),
    ParseError(String),
}

#[derive(Debug, Clone)]
pub struct SpringMap {
    springs: Vec<Vec<Condition>>,
    configurations: Vec<Vec<usize>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Condition {
    Operational,
    Damaged,
    Unknown,
}

impl fmt::Display for SpringsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SpringsError::UnknownCharacter(s) => write!(f, "Unknown character: {}", s),
            SpringsError::ParseError(s) => write!(f, "Parse error: {}", s),
        }
    }
}

impl Error for SpringsError {}

impl FromStr for SpringMap {
    type Err = SpringsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let n_lines = s.lines().count();
        let mut springs = Vec::with_capacity(n_lines);
        let mut configurations = Vec::with_capacity(n_lines);
        for l in s.lines() {
            let (springs_str, configuration_str) =
                l.split_once(' ').expect("Could not correctly parse input");

            let row_springs = springs_str
                .chars()
                .map(|c| match c {
                    '#' => Ok(Condition::Damaged),
                    '.' => Ok(Condition::Operational),
                    '?' => Ok(Condition::Unknown),
                    _ => Err(SpringsError::UnknownCharacter(c.to_string())),
                })
                .collect::<Result<Vec<_>, _>>()?;

            let cfg = configuration_str
                .split(',')
                .map(|n| match n.parse::<usize>() {
                    Ok(n) => Ok(n),
                    Err(_) => Err(SpringsError::ParseError(format!(
                        "Could not parse {} as a usize.",
                        n
                    ))),
                })
                .collect::<Result<Vec<_>, SpringsError>>()?;

            springs.push(row_springs);
            configurations.push(cfg);
        }
        Ok(SpringMap {
            springs,
            configurations,
        })
    }
}

fn solve(springs: &[Condition], counts: Vec<usize>) -> usize {
    // Remove trailing operational conditions
    let mut processed_springs = springs.to_vec();
    // Prepend a single Operational condition
    processed_springs.insert(0, Condition::Operational);

    let mut dp = vec![0; processed_springs.len() + 1];
    dp[0] = 1;

    // Initial DP setup
    for (i, &condition) in processed_springs.iter().enumerate() {
        if condition == Condition::Damaged {
            break;
        }
        dp[i + 1] = 1;
    }

    // Main DP calculation
    for &count in &counts {
        let mut next_dp = vec![0; processed_springs.len() + 1];
        let mut non_operational_streak = 0;

        for (i, &condition) in processed_springs.iter().enumerate() {
            non_operational_streak = if condition != Condition::Operational {
                non_operational_streak + 1
            } else {
                0
            };

            if condition != Condition::Damaged {
                next_dp[i + 1] = next_dp[i];
            }

            if non_operational_streak >= count
                && processed_springs
                    .get(i - count)
                    .map_or(false, |&c| c != Condition::Damaged)
            {
                next_dp[i + 1] += dp[i - count];
            }
        }

        dp = next_dp;
    }

    *dp.last().unwrap_or(&0)
}

pub fn generator(input: &str) -> SpringMap {
    input.parse().expect("Could not correctly parse input.")
}

pub fn part_1(spring_map: &SpringMap) -> usize {
    spring_map
        .springs
        .iter()
        .enumerate()
        .map(|(i, spring)| solve(spring, spring_map.configurations[i].clone()))
        .sum()
}

pub fn part_2(spring_map: &SpringMap) -> usize {
    let new_configurations = spring_map
        .configurations
        .iter()
        .map(|config| config.repeat(5))
        .collect::<Vec<_>>();
    let new_springs = spring_map
        .springs
        .iter()
        .map(|spring| {
            let mut extended = spring.to_owned();
            extended.push(Condition::Unknown);
            extended = extended.repeat(5);
            extended.pop();
            extended
        })
        .collect::<Vec<_>>();
    new_springs
        .iter()
        .zip(new_configurations.iter())
        .map(|(spring, configs)| solve(spring, configs.to_vec()))
        .sum()
}
