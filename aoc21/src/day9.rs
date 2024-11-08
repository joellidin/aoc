use std::collections::HashSet;

pub fn generator(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|l| {
            l.chars()
                .map(|n| n.to_digit(10).expect("Must be parsable to a digit.") as u8)
                .collect::<Vec<u8>>()
        })
        .collect()
}

fn find_low_points(height_map: &[Vec<u8>]) -> Vec<(usize, usize)> {
    let mut low_points = vec![];
    for i in 0..height_map.len() {
        for j in 0..height_map[0].len() {
            let left = if i > 0 {
                Some(height_map[i - 1][j])
            } else {
                None
            };
            let top = if j > 0 {
                Some(height_map[i][j - 1])
            } else {
                None
            };
            let right = if i + 1 < height_map.len() {
                Some(height_map[i + 1][j])
            } else {
                None
            };
            let bottom = if j + 1 < height_map[i].len() {
                Some(height_map[i][j + 1])
            } else {
                None
            };
            let p = height_map[i][j];
            let is_low_point = [left, top, right, bottom]
                .iter()
                .filter_map(|&neighbor| neighbor)
                .all(|neighbor| p < neighbor);
            if is_low_point {
                low_points.push((i, j));
            }
        }
    }
    low_points
}

fn find_basin_size(
    point: &(usize, usize),
    height_map: &[Vec<u8>],
    visited_points: &mut HashSet<(usize, usize)>,
) -> u32 {
    // If the point is already visited or is 9, it’s not part of any basin
    if visited_points.contains(point) || height_map[point.0][point.1] == 9 {
        return 0;
    }

    // Mark the current point as visited
    visited_points.insert(*point);

    let left_point = if point.0 > 0 {
        Some((point.0 - 1, point.1))
    } else {
        None
    };
    let top_point = if point.1 > 0 {
        Some((point.0, point.1 - 1))
    } else {
        None
    };
    let right_point = if point.0 + 1 < height_map.len() {
        Some((point.0 + 1, point.1))
    } else {
        None
    };
    let bottom_point = if point.1 + 1 < height_map[point.0].len() {
        Some((point.0, point.1 + 1))
    } else {
        None
    };

    // Collect neighbors into a vector and filter them based on the basin criteria
    let neighbors: Vec<_> = vec![left_point, top_point, right_point, bottom_point]
        .into_iter()
        .filter_map(|neighbor| {
            if let Some((x, y)) = neighbor {
                let current_value = height_map[point.0][point.1];
                let neighbor_value = height_map[x][y];

                // Include neighbor if it’s in the basin (value is less than 9 and >= current_value)
                if !visited_points.contains(&(x, y))
                    && neighbor_value != 9
                    && neighbor_value >= current_value
                {
                    Some((x, y))
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect();

    // Recursively calculate basin size by visiting each neighbor
    let basin_size: u32 = neighbors
        .into_iter()
        .map(|neighbor| find_basin_size(&neighbor, height_map, visited_points))
        .sum();

    basin_size + 1
}

pub fn part_1(input: &[Vec<u8>]) -> u32 {
    let low_points = find_low_points(input);
    low_points
        .iter()
        .map(|p| (input[p.0][p.1] + 1) as u32)
        .sum::<u32>()
}

pub fn part_2(input: &[Vec<u8>]) -> u32 {
    let low_points = find_low_points(input);
    let mut basin_sizes = low_points
        .iter()
        .map(|p| find_basin_size(p, input, &mut HashSet::new()))
        .collect::<Vec<u32>>();
    basin_sizes.sort_by(|s1, s2| s2.cmp(s1));
    basin_sizes.iter().take(3).product()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"2199943210
3987894921
9856789892
8767896789
9899965678
"#;

    #[test]
    fn part_1_test() {
        let generator_output = generator(INPUT);
        let result = part_1(&generator_output);
        assert_eq!(result, 15);
    }

    #[test]
    fn part_2_test() {
        let generator_output = generator(INPUT);
        let result = part_2(&generator_output);
        assert_eq!(result, 1134);
    }
}
