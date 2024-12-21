use std::collections::{hash_map::Entry, HashMap, VecDeque};

use aoc_utils::prelude::*;

pub fn generator(input: &str) -> Vec<(&str, u64)> {
    input
        .lines()
        .map(|line| {
            let nums = extract_integers::<u64>(line);
            (line, nums[0])
        })
        .collect()
}

fn bfs(start: char, end: char, num: bool) -> Vec<String> {
    let mut dist = HashMap::new();
    let mut q = VecDeque::new();
    let mut results = Vec::new();

    dist.insert(start, 0);
    q.push_back((start, String::new(), 0));

    let mut min_dist_end = None;

    while let Some((pos, path, d)) = q.pop_front() {
        if let Some(min_dist) = min_dist_end {
            if d > min_dist {
                continue;
            }
        }

        if pos == end {
            if min_dist_end.is_none() || d < min_dist_end.unwrap() {
                min_dist_end = Some(d);
                results.clear();
            }

            if Some(d) == min_dist_end {
                let mut new_path = path.clone();
                new_path.push('A');
                results.push(new_path);
            }
            continue;
        }

        for (new_pos, action) in get_neighbours(&pos, num) {
            let next_dist = d + 1;

            if let Entry::Vacant(e) = dist.entry(new_pos) {
                e.insert(next_dist);
                let mut new_path = path.clone();
                new_path.push(action);
                q.push_back((new_pos, new_path, next_dist));
            } else if dist[&new_pos] == next_dist {
                let mut new_path = path.clone();
                new_path.push(action);
                q.push_back((new_pos, new_path, next_dist));
            }
        }
    }

    results
}

fn get_neighbours(dir: &char, num: bool) -> Vec<(char, char)> {
    match dir {
        '0' => vec![('A', '>'), ('2', '^')],
        '1' => vec![('2', '>'), ('4', '^')],
        '2' => vec![('1', '<'), ('3', '>'), ('5', '^'), ('0', 'v')],
        '3' => vec![('2', '<'), ('6', '^'), ('A', 'v')],
        '4' => vec![('5', '>'), ('7', '^'), ('1', 'v')],
        '5' => vec![('4', '<'), ('6', '>'), ('8', '^'), ('2', 'v')],
        '6' => vec![('5', '<'), ('9', '^'), ('3', 'v')],
        '7' => vec![('8', '>'), ('4', 'v')],
        '8' => vec![('7', '<'), ('9', '>'), ('5', 'v')],
        '9' => vec![('8', '<'), ('6', 'v')],
        '^' => vec![('v', 'v'), ('A', '>')],
        '>' => vec![('v', '<'), ('A', '^')],
        'v' => vec![('^', '^'), ('>', '>'), ('<', '<')],
        '<' => vec![('v', '>')],
        'A' if num => vec![('0', '<'), ('3', '^')],
        'A' if !num => vec![('^', '<'), ('>', 'v')],
        _ => unreachable!(),
    }
}

fn solve(
    goal: &String,
    depth: usize,
    target_depth: usize,
    cache: &mut HashMap<(String, usize, usize), u64>,
) -> u64 {
    if let Some(res) = cache.get(&(goal.to_string(), depth, target_depth)) {
        return *res;
    }

    if depth == target_depth {
        let mut new_start = 'A';
        let res = goal
            .chars()
            .map(|c| {
                let res = bfs(new_start, c, false)[0].len() as u64;
                new_start = c;
                res
            })
            .sum();
        cache.insert((goal.to_string(), depth, target_depth), res);
        return res;
    }

    let mut new_start = 'A';
    let mut sum = 0;
    for c in goal.chars() {
        let paths = if depth == 0 {
            bfs(new_start, c, true)
        } else {
            bfs(new_start, c, false)
        };
        sum += paths
            .iter()
            .map(|s| solve(s, depth + 1, target_depth, cache))
            .min()
            .expect("Must find a path");
        new_start = c;
    }
    cache.insert((goal.to_string(), depth, target_depth), sum);
    sum
}

pub fn part_1(input: &[(&str, u64)]) -> u64 {
    input.iter().fold(0, |acc, (s, num)| {
        let mut cache = HashMap::new();
        acc + solve(&s.to_string(), 0, 2, &mut cache) * num
    })
}

pub fn part_2(input: &[(&str, u64)]) -> u64 {
    input.iter().fold(0, |acc, (s, num)| {
        let mut cache = HashMap::new();
        acc + solve(&s.to_string(), 0, 25, &mut cache) * num
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"029A
980A
179A
456A
379A
"#;

    #[test]
    fn part_1_test() {
        let generator_output = generator(INPUT);
        let result = part_1(&generator_output);
        assert_eq!(result, 126384);
    }

    #[test]
    fn part_2_test() {
        let generator_output = generator("029A");
        let result = part_2(&generator_output);
        assert_eq!(result, 2379451789590);
    }
}
