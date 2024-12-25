const COLUMNS: usize = 5;

pub fn generator(input: &str) -> Vec<Vec<Vec<char>>> {
    input
        .split("\n\n")
        .map(|grid| grid.lines().map(|line| line.chars().collect()).collect())
        .collect()
}

pub fn part_1(input: &[Vec<Vec<char>>]) -> u32 {
    let mut locks = Vec::new();
    let mut keys = Vec::new();
    input.iter().for_each(|grid| {
        let mut col_heights = vec![0; COLUMNS];
        for row in grid {
            for (j, &c) in row.iter().enumerate() {
                if c == '#' {
                    col_heights[j] += 1
                }
            }
        }
        if grid[0].iter().all(|&c| c == '#') {
            locks.push(col_heights);
        } else {
            keys.push(col_heights);
        }
    });
    let mut res = 0;
    for key in &keys {
        for lock in &locks {
            if (0..COLUMNS).all(|i| key[i] + lock[i] < 8) {
                res += 1;
            }
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####
"#;

    #[test]
    fn part_1_test() {
        let generator_output = generator(INPUT);
        let result = part_1(&generator_output);
        assert_eq!(result, 3);
    }
}
