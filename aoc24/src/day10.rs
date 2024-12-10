use std::collections::HashSet;

pub fn generator(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

fn count_paths(
    map: &[Vec<u32>],
    current: (usize, usize),
    visited_ends: &mut Option<&mut HashSet<(usize, usize)>>,
) -> u32 {
    let neighbours = get_next(map, current);
    let mut total_paths = 0;

    for neighbour in neighbours {
        let height = map[neighbour.0][neighbour.1];
        if height == 9 {
            if let Some(ends) = visited_ends.as_mut() {
                if !ends.contains(&neighbour) {
                    total_paths += 1;
                    ends.insert(neighbour);
                }
            } else {
                total_paths += 1;
            }
        } else {
            total_paths += count_paths(map, neighbour, visited_ends);
        }
    }
    total_paths
}

fn get_next(
    map: &[Vec<u32>],
    point: (usize, usize),
) -> impl Iterator<Item = (usize, usize)> + use<'_> {
    [(-1, 0), (0, -1), (1, 0), (0, 1)]
        .iter()
        .filter_map(move |(di, dj)| {
            let new_point = (point.0 as isize + di, point.1 as isize + dj);
            if new_point.0 >= 0
                && new_point.1 >= 0
                && new_point.0 < map.len() as isize
                && new_point.1 < map[0].len() as isize
                && map[new_point.0 as usize][new_point.1 as usize] - map[point.0][point.1] == 1
            {
                return Some((new_point.0 as usize, new_point.1 as usize));
            }
            None
        })
}

pub fn part_1(input: &[Vec<u32>]) -> u32 {
    let mut total_score = 0;

    for (i, row) in input.iter().enumerate() {
        for (j, &height) in row.iter().enumerate() {
            if height == 0 {
                let mut visited = HashSet::new();
                let score = count_paths(input, (i, j), &mut Some(&mut visited));
                total_score += score;
            }
        }
    }
    total_score
}

pub fn part_2(input: &[Vec<u32>]) -> u32 {
    let mut total_score = 0;

    for (i, row) in input.iter().enumerate() {
        for (j, &height) in row.iter().enumerate() {
            if height == 0 {
                let score = count_paths(input, (i, j), &mut None);
                total_score += score;
            }
        }
    }
    total_score
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
"#;

    #[test]
    fn part_1_test() {
        let generator_output = generator(INPUT);
        let result = part_1(&generator_output);
        assert_eq!(result, 36);
    }

    #[test]
    fn part_2_test() {
        let generator_output = generator(INPUT);
        let result = part_2(&generator_output);
        assert_eq!(result, 81);
    }
}
