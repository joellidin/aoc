pub fn generator(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

pub fn part_1(input: &[Vec<char>]) -> u32 {
    let target = ['X', 'M', 'A', 'S'];
    let mut count = 0;

    let rows = input.len();
    let cols = input[0].len();

    // Directions: (row_delta, col_delta)
    let directions = [
        (0, 1),
        (0, -1),
        (1, 0),
        (-1, 0),
        (1, 1),
        (-1, -1),
        (1, -1),
        (-1, 1),
    ];

    for i in 0..rows {
        for j in 0..cols {
            // Check all directions
            for &(row_delta, col_delta) in &directions {
                let mut found = true;

                for (k, &target_char) in target.iter().enumerate() {
                    let row = i as isize + k as isize * row_delta;
                    let col = j as isize + k as isize * col_delta;

                    // Ensure we're within bounds and that the next character is the target
                    if row < 0
                        || col < 0
                        || row >= rows as isize
                        || col >= cols as isize
                        || input[row as usize][col as usize] != target_char
                    {
                        found = false;
                        break;
                    }
                }

                if found {
                    count += 1;
                }
            }
        }
    }

    count
}

pub fn part_2(input: &[Vec<char>]) -> u32 {
    let mut count = 0;

    // Dimensions of the grid
    let rows = input.len();
    let cols = input[0].len();

    // Directions for diagonals
    let directions = [(-1, -1), (-1, 1), (1, -1), (1, 1)];

    for i in 0..rows {
        for j in 0..cols {
            // Check only if the current cell is 'A'
            if input[i][j] != 'A' {
                continue;
            }

            let mut valid_count = 0;

            for &(row_delta, col_delta) in &directions {
                let mut valid = true;

                let m_row = i as isize + row_delta;
                let m_col = j as isize + col_delta;
                if m_row < 0
                    || m_col < 0
                    || m_row >= rows as isize
                    || m_col >= cols as isize
                    || input[m_row as usize][m_col as usize] != 'M'
                {
                    valid = false;
                }

                let s_row = i as isize - row_delta;
                let s_col = j as isize - col_delta;
                if s_row < 0
                    || s_col < 0
                    || s_row >= rows as isize
                    || s_col >= cols as isize
                    || input[s_row as usize][s_col as usize] != 'S'
                {
                    valid = false;
                }

                if valid {
                    valid_count += 1;
                }
            }

            // Increment the count if two diagonals are valid
            if valid_count == 2 {
                count += 1;
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
"#;

    #[test]
    fn part_1_test() {
        let generator_output = generator(INPUT);
        let result = part_1(&generator_output);
        assert_eq!(result, 18);
    }

    #[test]
    fn part_2_test() {
        let generator_output = generator(INPUT);
        let result = part_2(&generator_output);
        assert_eq!(result, 9);
    }
}
