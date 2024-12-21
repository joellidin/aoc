use std::{
    collections::{HashMap, VecDeque},
    rc::Rc,
};

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

fn bfs(
    start: char,
    end: char,
    num: bool,
    cache: &mut HashMap<(char, char, bool), Vec<Rc<str>>>,
) -> Vec<Rc<str>> {
    if let Some(res) = cache.get(&(start, end, num)) {
        return res.to_vec();
    }
    let mut distances = HashMap::new();
    let mut q = VecDeque::new();
    let mut results = Vec::new();

    distances.insert(start, 0);
    q.push_back((start, String::new(), 0));

    let mut min_dist_end = None;

    while let Some((pos, path, d)) = q.pop_front() {
        if let Some(min_dist) = min_dist_end {
            if d > min_dist {
                continue;
            }
        }

        if pos == end {
            let mut new_path = path.clone();
            new_path.push('A');
            let path_rc = Rc::<str>::from(new_path);
            results.push(path_rc);
            min_dist_end = Some(d);
            continue;
        }

        distances.insert(pos, d);
        for (new_pos, action) in get_neighbours(&pos, num) {
            let mut new_path = path.clone();
            new_path.push(action);
            q.push_back((new_pos, new_path, d + 1));
        }
    }
    cache.insert((start, end, num), results);
    cache.get(&(start, end, num)).unwrap().to_vec()
}

fn get_neighbours(curr: &char, num: bool) -> Vec<(char, char)> {
    match curr {
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

type CacheKey = (Rc<str>, usize, usize);
fn solve(
    goal: &str,
    depth: usize,
    target_depth: usize,
    cache: &mut HashMap<CacheKey, u64>,
    bfs_cache: &mut HashMap<(char, char, bool), Vec<Rc<str>>>,
) -> u64 {
    // Create the key just once.
    let goal_rc = Rc::<str>::from(goal);
    let key = (goal_rc.clone(), depth, target_depth);

    if let Some(res) = cache.get(&key) {
        return *res;
    }

    if depth == target_depth {
        let mut new_start = 'A';
        let res = goal
            .chars()
            .map(|c| {
                let res = bfs(new_start, c, false, bfs_cache)[0].len() as u64;
                new_start = c;
                res
            })
            .sum();
        cache.insert(key, res);
        return res;
    }

    let mut new_start = 'A';
    let mut sum = 0;
    goal.chars().for_each(|c| {
        let paths = if depth == 0 {
            bfs(new_start, c, true, bfs_cache)
        } else {
            bfs(new_start, c, false, bfs_cache)
        };
        sum += paths
            .iter()
            .map(|s| solve(s, depth + 1, target_depth, cache, bfs_cache))
            .min()
            .expect("Must find a path");
        new_start = c;
    });
    cache.insert(key, sum);
    sum
}

pub fn part_1(input: &[(&str, u64)]) -> u64 {
    input.iter().fold(0, |acc, (s, num)| {
        let mut cache = HashMap::new();
        let mut bfs_cache = HashMap::new();
        acc + solve(s, 0, 2, &mut cache, &mut bfs_cache) * num
    })
}

pub fn part_2(input: &[(&str, u64)]) -> u64 {
    input.iter().fold(0, |acc, (s, num)| {
        let mut cache = HashMap::new();
        let mut bfs_cache = HashMap::new();
        acc + solve(s, 0, 25, &mut cache, &mut bfs_cache) * num
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
