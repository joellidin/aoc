use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Condition {
    part_name: Option<char>,
    comparator: Option<Comparator>,
    threshold: Option<u32>,
    destination: String,
}

#[derive(Debug, Clone, Copy)]
enum Comparator {
    GreaterThan,
    LessThan,
    GreaterOrEqual,
    LessOrEqual,
}

fn parse_condition(condition_text: &str) -> Result<Condition, &str> {
    let parts: Vec<&str> = condition_text.split(':').collect();
    // The 'else' part of the condition
    if parts.len() != 2 {
        return Ok(Condition {
            part_name: None,
            comparator: None,
            threshold: None,
            destination: condition_text.to_owned(),
        });
    }

    let condition_part = parts[0];
    let destination = parts[1].to_string();

    let (rating_part, rest) = condition_part.split_at(1);
    let part_name = rating_part
        .chars()
        .next()
        .ok_or("Could not get part name when parsing condition")?;

    let (comparator, threshold_str) = if let Some(index) = rest.find('>') {
        (Comparator::GreaterThan, &rest[index + 1..])
    } else if let Some(index) = rest.find('<') {
        (Comparator::LessThan, &rest[index + 1..])
    } else {
        return Err("Invalid comparison operator");
    };

    let threshold = threshold_str
        .parse::<u32>()
        .map_err(|_| "Could not parse threshold to u32")?;

    Ok(Condition {
        part_name: Some(part_name),
        comparator: Some(comparator),
        threshold: Some(threshold),
        destination,
    })
}

fn execute_workflow<'a>(
    conditions: &'a [Condition],
    rating_map: &HashMap<char, u32>,
) -> Result<&'a str, String> {
    for condition in conditions {
        match &condition.comparator {
            None => return Ok(&condition.destination),
            Some(comparator) => {
                let part_rating = rating_map
                    .get(&condition.part_name.ok_or("Part name is missing")?)
                    .ok_or("Part rating not found")?;
                let threshold = condition.threshold.ok_or("Threshold value is missing")?;
                match comparator {
                    Comparator::GreaterThan if part_rating > &threshold => {
                        return Ok(&condition.destination)
                    }
                    Comparator::LessThan if part_rating < &threshold => {
                        return Ok(&condition.destination)
                    }
                    _ => continue, // If condition not met, continue to next iteration
                }
            }
        }
    }
    Err("No valid conditions found".to_string())
}

fn invert_comparator(condition: &Condition) -> Condition {
    let inverted_operation = match &condition.comparator {
        Some(Comparator::GreaterThan) => Some(Comparator::LessOrEqual),
        Some(Comparator::LessThan) => Some(Comparator::GreaterOrEqual),
        None => None,
        _ => panic!("Cannot invert this operation"),
    };

    Condition {
        part_name: condition.part_name,
        comparator: inverted_operation,
        threshold: condition.threshold,
        destination: condition.destination.clone(),
    }
}

fn find_paths_to_acceptance(
    workflow_map: &HashMap<&str, Vec<Condition>>,
    current_workflow: &str,
    mut path: Vec<Condition>,
    all_paths: &mut Vec<Vec<Condition>>,
) {
    if let Some(conditions) = workflow_map.get(current_workflow) {
        for condition in conditions {
            if condition.comparator.is_some() {
                let mut new_path = path.clone();
                new_path.push(condition.clone());

                // Explore path when the condition is not met
                let inverted_condition = invert_comparator(condition);
                path.push(inverted_condition.clone());

                // Explore path if condition is met
                if condition.destination == "A" {
                    all_paths.push(new_path.clone());
                } else {
                    find_paths_to_acceptance(
                        workflow_map,
                        &condition.destination,
                        new_path,
                        all_paths,
                    );
                }

            // 'Else' condition in workflow (the last condition)
            } else {
                if condition.destination == "A" {
                    all_paths.push(path.clone());
                    continue;
                }
                find_paths_to_acceptance(
                    workflow_map,
                    &condition.destination,
                    path.clone(),
                    all_paths,
                );
            }
        }
    }
}

fn adjust_range(operation: Comparator, threshold: u32, min: u64, max: u64) -> (u64, u64) {
    match operation {
        Comparator::GreaterThan => {
            if min > threshold as u64 {
                (min, max)
            } else {
                (threshold as u64 + 1, max)
            }
        }
        Comparator::LessThan => {
            if max < threshold as u64 {
                (min, max)
            } else {
                (min, threshold as u64 - 1)
            }
        }
        Comparator::GreaterOrEqual => {
            if min >= threshold as u64 {
                (min, max)
            } else {
                (threshold as u64, max)
            }
        }
        Comparator::LessOrEqual => {
            if max <= threshold as u64 {
                (min, max)
            } else {
                (min, threshold as u64)
            }
        }
    }
}

fn get_combinations(conditions: &[Condition], min_val: u64, max_val: u64) -> Result<u64, &str> {
    let (mut x_min, mut x_max) = (min_val, max_val);
    let (mut m_min, mut m_max) = (min_val, max_val);
    let (mut a_min, mut a_max) = (min_val, max_val);
    let (mut s_min, mut s_max) = (min_val, max_val);

    for cond in conditions {
        let threshold = match cond.threshold {
            Some(n) => n,
            None => continue,
        };
        let operation = cond
            .comparator
            .ok_or("Operation not present in condition")?;

        match cond.part_name {
            Some('x') => (x_min, x_max) = adjust_range(operation, threshold, x_min, x_max),
            Some('m') => (m_min, m_max) = adjust_range(operation, threshold, m_min, m_max),
            Some('a') => (a_min, a_max) = adjust_range(operation, threshold, a_min, a_max),
            Some('s') => (s_min, s_max) = adjust_range(operation, threshold, s_min, s_max),
            _ => return Err("Invalid part name"),
        }
    }

    Ok((x_max - x_min + 1) * (m_max - m_min + 1) * (a_max - a_min + 1) * (s_max - s_min + 1))
}

type WorkflowRatingTuple<'a> = (HashMap<&'a str, Vec<Condition>>, Vec<HashMap<char, u32>>);
pub fn generator(input: &str) -> Result<WorkflowRatingTuple, &str> {
    let (workflows_str, ratings_str) = input.split_once("\n\n").ok_or("Incorrect input format")?;

    let mut workflow_map = HashMap::new();
    for line in workflows_str.lines() {
        let (name, conditions_str) = line
            .trim_end_matches('}')
            .split_once('{')
            .ok_or("Wrong workflow format")?;

        let mut conditions = Vec::new();
        for condition_str in conditions_str.split(',') {
            let condition = parse_condition(condition_str)?;
            conditions.push(condition);
        }
        workflow_map.insert(name, conditions);
    }

    let mut rating_maps = Vec::new();
    for line in ratings_str.lines() {
        let mut rating_map = HashMap::new();
        for part_str in line
            .trim_start_matches('{')
            .trim_end_matches('}')
            .split(',')
        {
            let (name, rating_str) = part_str.split_once('=').ok_or("Invalid rating map")?;
            let name = name.chars().next().ok_or("Invalid part name")?;
            let rating = rating_str
                .parse::<u32>()
                .map_err(|_| "Rating parse error")?;
            rating_map.insert(name, rating);
        }
        rating_maps.push(rating_map);
    }

    Ok((workflow_map, rating_maps))
}

pub fn part_1(workflow_rating_tuple: &Result<WorkflowRatingTuple, &str>) -> u32 {
    let (workflow_map, part_ratings) = workflow_rating_tuple
        .as_ref()
        .expect("Could not properly parse input");
    part_ratings
        .iter()
        .map(|ratings| {
            let mut destination = execute_workflow(&workflow_map["in"], ratings)
                .expect("Could not execute workflow for 'in'");
            while !matches!(destination, "A" | "R") {
                destination = execute_workflow(&workflow_map[destination], ratings)
                    .expect("Could not execute workflow");
            }
            if destination == "A" {
                ratings.iter().fold(0, |acc, (_, rating)| acc + rating)
            } else {
                0
            }
        })
        .sum()
}

pub fn part_2(workflow_rating_tuple: &Result<WorkflowRatingTuple, &str>) -> u64 {
    let workflow_map = &workflow_rating_tuple
        .as_ref()
        .expect("Could not properly parse input")
        .0;
    let mut all_paths = Vec::new();
    find_paths_to_acceptance(workflow_map, "in", Vec::new(), &mut all_paths);
    all_paths.iter().fold(0, |acc, cond| {
        acc + get_combinations(cond, 1, 4000).expect("Could not calculate combinations")
    })
}
