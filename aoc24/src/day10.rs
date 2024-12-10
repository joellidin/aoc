use std::collections::HashSet;

pub fn generator(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

fn count_paths(
    map: &[Vec<u32>],
    current: (isize, isize),
    visited_ends: &mut Option<&mut HashSet<(isize, isize)>>,
) -> u32 {
    let neighbours = get_next(map, current);
    let mut total_paths = 0;

    for neighbour in neighbours {
        let height = map[neighbour.0 as usize][neighbour.1 as usize];
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

fn get_next(map: &[Vec<u32>], point: (isize, isize)) -> Vec<(isize, isize)> {
    let mut neighbours = Vec::new();
    for (di, dj) in [(-1, 0), (0, -1), (1, 0), (0, 1)] {
        let new_point = (point.0 + di, point.1 + dj);
        if new_point.0 >= 0
            && new_point.1 >= 0
            && new_point.0 < map.len() as isize
            && new_point.1 < map[0].len() as isize
            && map[new_point.0 as usize][new_point.1 as usize]
                - map[point.0 as usize][point.1 as usize]
                == 1
        {
            neighbours.push(new_point);
        }
    }
    neighbours
}

pub fn part_1(input: &[Vec<u32>]) -> u32 {
    let mut total_score = 0;

    for (i, row) in input.iter().enumerate() {
        for (j, &height) in row.iter().enumerate() {
            if height == 0 {
                let start = (i as isize, j as isize);
                let mut visited = HashSet::new();
                let score = count_paths(input, start, &mut Some(&mut visited));
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
                let start = (i as isize, j as isize);
                let score = count_paths(input, start, &mut None);
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
