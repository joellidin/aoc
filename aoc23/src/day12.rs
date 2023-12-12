use std::collections::HashMap;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

/// Counts all possible configurations of springs based on their condition.
///
/// This function recursively evaluates each spring in the given list, considering
/// their operational, damaged, or unknown states, and calculates the total number
/// of valid configurations that match the provided configuration sizes.
fn count_possible_configs<'a>(
    springs: &'a [Condition],
    configurations: &'a [usize],
    memoization: &mut HashMap<(&'a [usize], &'a [Condition]), usize>,
) -> usize {
    // Base case: If there are no springs left, check if configurations are also empty.
    if springs.is_empty() {
        return (configurations.is_empty()) as usize;
    }

    // Handling different conditions of the first spring in the list.
    match springs[0] {
        // If the first spring is operational, skip it and continue with the rest.
        Condition::Operational => {
            count_possible_configs(&springs[1..], configurations, memoization)
        }
        // If the first spring is damaged or its condition is unknown,
        // we need to consider scenarios where it's part of a damaged group.
        Condition::Damaged | Condition::Unknown => {
            let configs_with_damaged = count_damaged_configs(springs, configurations, memoization);

            if springs[0] == Condition::Unknown {
                // For unknown conditions, consider both possibilities: as damaged and as operational.
                let configs_skipping_unknown =
                    count_possible_configs(&springs[1..], configurations, memoization);
                configs_with_damaged + configs_skipping_unknown
            } else {
                // If the spring is confirmed as damaged, only consider configurations with it as damaged.
                configs_with_damaged
            }
        }
    }
}

/// Counts configurations where groups of springs are considered to be damaged.
///
/// This function evaluates the possibility of forming groups of damaged springs
/// based on the given configurations. It considers both confirmed damaged and unknown
/// springs, excluding operational springs from forming valid damaged groups.
fn count_damaged_configs<'a>(
    springs: &'a [Condition],
    configuration: &'a [usize],
    memoization: &mut HashMap<(&'a [usize], &'a [Condition]), usize>,
) -> usize {
    // Check if the result is already computed to avoid redundant calculations.
    if let Some(&result) = memoization.get(&(configuration, springs)) {
        return result;
    }

    // If there are no configurations left or no springs left, no valid solutions can be formed.
    let result = if configuration.is_empty() || springs.is_empty() {
        0
    } else {
        let next_group_size = configuration[0];

        // Check if there are enough springs left for the current configuration.
        // If the number of springs is less than the required group size, no solution is possible.
        if springs.len() < next_group_size
            // If any of the springs in the required group size are operational, it invalidates
            // the configuration since we are looking for groups of damaged springs.
            || springs[..next_group_size].contains(&Condition::Operational)
            // If the spring immediately following the current group is also damaged,
            // it means this configuration merges two groups of damaged springs, which is not allowed.
            || springs.get(next_group_size) == Some(&Condition::Damaged)
        {
            0
        } else if next_group_size == springs.len() && configuration.len() == 1 {
            // If the current group size equals the total number of springs left, and there's
            // only one configuration left, it means this is a valid end configuration.
            1
        } else {
            // Recurse with the remaining springs and configurations, skipping the current group of springs.
            count_possible_configs(
                &springs[std::cmp::min(next_group_size + 1, springs.len())..],
                &configuration[1..],
                memoization,
            )
        }
    };

    // Store the computed result in memoization to avoid recalculating for the same inputs.
    memoization.insert((configuration, springs), result);
    result
}

pub fn generator(input: &str) -> SpringMap {
    input.parse().expect("Could not correctly parse input.")
}

pub fn part_1(spring_map: &SpringMap) -> usize {
    let mut memoization = HashMap::new();
    spring_map
        .springs
        .iter()
        .zip(spring_map.configurations.iter())
        .map(|(spring, configuration)| {
            count_possible_configs(spring, configuration, &mut memoization)
        })
        .sum()
}

pub fn part_2(spring_map: &SpringMap) -> usize {
    let mut memoization = HashMap::new();

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

    let new_configurations = spring_map
        .configurations
        .iter()
        .map(|config| config.repeat(5))
        .collect::<Vec<_>>();

    new_springs
        .iter()
        .zip(new_configurations.iter())
        .map(|(spring, configuration)| {
            count_possible_configs(spring, configuration, &mut memoization)
        })
        .sum()
}
