use std::collections::{HashMap, HashSet};

pub fn generator(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn find_unique_antennas(antennas: &[Vec<char>]) -> HashMap<char, Vec<(usize, usize)>> {
    antennas.iter().enumerate().fold(
        HashMap::new(),
        |mut acc: HashMap<char, Vec<_>>, (i, row)| {
            row.iter().enumerate().for_each(|(j, c)| {
                if *c != '.' {
                    acc.entry(*c).or_default().push((i, j));
                }
            });
            acc
        },
    )
}

fn find_anti_nodes(
    antenna_map: &HashMap<char, Vec<(usize, usize)>>,
    height: usize,
    width: usize,
    distance: Option<isize>,
) -> HashSet<(usize, usize)> {
    antenna_map
        .iter()
        .fold(HashSet::new(), |mut acc, (_, pos)| {
            for i in 0..pos.len() {
                for j in i + 1..pos.len() {
                    let (i1, j1) = pos[i];
                    let (i2, j2) = pos[j];

                    // Compute the difference
                    let di = i2 as isize - i1 as isize;
                    let dj = j2 as isize - j1 as isize;

                    let max_factor = width.max(height) as isize;
                    for factor in 0..=max_factor {
                        // Antinode related to the first antenna:
                        // Move opposite to the direction from the first to the second.
                        if let Some(distance) = distance {
                            if factor != distance - 1 {
                                continue;
                            }
                        }
                        let antinode_from_first = (
                            (i1 as isize - di * factor) as usize,
                            (j1 as isize - dj * factor) as usize,
                        );

                        // Antinode related to the second antenna:
                        // Move in the direction from the first to the second.
                        let antinode_from_second = (
                            (i2 as isize + di * factor) as usize,
                            (j2 as isize + dj * factor) as usize,
                        );

                        // Check bounds and occupancy before inserting
                        if antinode_from_first.0 < height && antinode_from_first.1 < width {
                            acc.insert(antinode_from_first);
                        }

                        if antinode_from_second.0 < height && antinode_from_second.1 < width {
                            acc.insert(antinode_from_second);
                        }
                    }
                }
            }
            acc
        })
}

pub fn part_1(input: &[Vec<char>]) -> u32 {
    let antenna_map = find_unique_antennas(input);
    find_anti_nodes(&antenna_map, input.len(), input[0].len(), Some(2)).len() as u32
}

pub fn part_2(input: &[Vec<char>]) -> u32 {
    let antenna_map = find_unique_antennas(input);
    find_anti_nodes(&antenna_map, input.len(), input[0].len(), None).len() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#""............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
"#;

    #[test]
    fn part_1_test() {
        let generator_output = generator(INPUT);
        let result = part_1(&generator_output);
        assert_eq!(result, 14);
    }

    #[test]
    fn part_2_test() {
        let generator_output = generator(INPUT);
        let result = part_2(&generator_output);
        assert_eq!(result, 34);
    }
}
