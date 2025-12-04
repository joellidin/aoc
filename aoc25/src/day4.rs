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
    let mut grid = input.to_vec();
    let mut replacements = Vec::new();
    let mut count = 0;
    loop {
        let mut removals = 0;
        for (y, row) in grid.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                if matches!(tile, Tile::Roll) && count_neighbors(&grid, x, y) < 4 {
                    count += 1;
                    removals += 1;
                    replacements.push((x, y));
                }
            }
        }
        if removals == 0 {
            break;
        }
        for (x, y) in replacements.drain(..) {
            grid[y][x] = Tile::Space;
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
