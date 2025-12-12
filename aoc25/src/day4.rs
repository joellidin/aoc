use std::collections::VecDeque;

#[derive(PartialEq, Eq, Clone)]
pub enum Tile {
    Roll,
    Space,
}

pub fn generator(input: &str) -> Vec<Vec<Tile>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '@' => Tile::Roll,
                    '.' => Tile::Space,
                    _ => panic!("Invalid input"),
                })
                .collect()
        })
        .collect()
}

pub fn count_neighbors(input: &[Vec<Tile>], x: usize, y: usize) -> u32 {
    let mut count = 0;
    for dy in -1..=1_i32 {
        for dx in -1..=1_i32 {
            if dx == 0 && dy == 0 {
                continue;
            }
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            if ny >= 0
                && ny < input.len() as i32
                && nx >= 0
                && nx < input[ny as usize].len() as i32
                && matches!(input[ny as usize][nx as usize], Tile::Roll)
            {
                count += 1;
            }
        }
    }
    count
}

fn count_neighbors_bool(grid: &[Vec<bool>], x: usize, y: usize) -> u32 {
    let mut count = 0;
    for dy in -1..=1_i32 {
        for dx in -1..=1_i32 {
            if dx == 0 && dy == 0 {
                continue;
            }
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            if ny >= 0
                && ny < grid.len() as i32
                && nx >= 0
                && nx < grid[ny as usize].len() as i32
                && grid[ny as usize][nx as usize]
            {
                count += 1;
            }
        }
    }
    count
}

pub fn part_1(input: &[Vec<Tile>]) -> u32 {
    input.iter().enumerate().fold(0, |acc, (y, row)| {
        acc + row
            .iter()
            .enumerate()
            .filter(|(x, tile)| matches!(tile, Tile::Roll) && count_neighbors(input, *x, y) < 4)
            .count() as u32
    })
}

pub fn part_2(input: &[Vec<Tile>]) -> u32 {
    let height = input.len();
    let width = input[0].len();

    let mut is_roll: Vec<Vec<bool>> = input
        .iter()
        .map(|row| row.iter().map(|t| matches!(t, Tile::Roll)).collect())
        .collect();

    // Precompute neighbor counts
    let mut neighbors: Vec<Vec<u32>> = (0..height)
        .map(|y| {
            (0..width)
                .map(|x| count_neighbors_bool(&is_roll, x, y))
                .collect()
        })
        .collect();

    // Initialize queue with all accessible rolls
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
    for y in 0..height {
        for x in 0..width {
            if is_roll[y][x] && neighbors[y][x] < 4 {
                queue.push_back((x, y));
            }
        }
    }

    let mut count = 0;
    while let Some((x, y)) = queue.pop_front() {
        if !is_roll[y][x] {
            continue; // Already removed
        }

        is_roll[y][x] = false;
        count += 1;

        // Update neighbors and enqueue newly accessible rolls
        for dy in -1..=1_i32 {
            for dx in -1..=1_i32 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;
                if ny >= 0 && ny < height as i32 && nx >= 0 && nx < width as i32 {
                    let (nx, ny) = (nx as usize, ny as usize);
                    if is_roll[ny][nx] {
                        neighbors[ny][nx] -= 1;
                        if neighbors[ny][nx] == 3 {
                            queue.push_back((nx, ny));
                        }
                    }
                }
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
"#;

    #[test]
    fn part_1_test() {
        let generator_output = generator(INPUT);
        let result = part_1(&generator_output);
        assert_eq!(result, 13);
    }

    #[test]
    fn part_2_test() {
        let generator_output = generator(INPUT);
        let result = part_2(&generator_output);
        assert_eq!(result, 43);
    }
}
