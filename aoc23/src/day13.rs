pub fn generator(input: &str) -> Vec<Vec<Vec<char>>> {
    input
        .split("\n\n")
        .map(|notes| notes.lines().map(|l| l.chars().collect()).collect())
        .collect()
}

enum LineDirection {
    Horizontal,
    Vertical,
}

/// Searches for a mirror line in a character matrix. A mirror line is a line where the pattern on
/// one side is a mirror image of the other side, allowing for some errors.
///
/// `notes`: the character matrix
/// `allowed_smudges`: the allowed number of discrepancies (smudges)
/// `direction`: the direction of the line (horizontal or vertical)
///
/// Returns the index of the mirror line if found, otherwise None.
fn find_mirror_line(
    notes: &[Vec<char>],
    direction: &LineDirection,
    allowed_smudges: u32,
) -> Option<u32> {
    let num_possible_lines = match direction {
        LineDirection::Horizontal => notes.len(),
        LineDirection::Vertical => notes.first().map_or(0, Vec::len),
    };

    for mirror_line in 0..num_possible_lines {
        let mut is_mirror = false;
        let mut num_errors = 0;

        for offset in 0..=mirror_line {
            // Calculate indices for comparison based on the current mirror line and offset.
            // `first` represents the top row if we are looking for a horizontal line,
            // or the left column if we are looking for a vertical line.
            let first = mirror_line.checked_sub(offset);

            // `second` represents the bottom row if we are looking for a horizontal line,
            // or the right column if we are looking for a vertical line.
            let second = mirror_line.checked_add(offset + 1);

            match (first, second) {
                (Some(first_idx), Some(second_idx)) if second_idx < num_possible_lines => {
                    num_errors += find_errors(notes, direction, first_idx, second_idx);
                    is_mirror = num_errors <= allowed_smudges;
                    if !is_mirror {
                        break;
                    }
                }
                _ => break, // Out of bounds
            }
        }
        if is_mirror && num_errors == allowed_smudges {
            return Some(mirror_line as u32);
        }
    }
    None
}

/// Compares two lines or columns in the matrix and counts the number of discrepancies.
fn find_errors(
    notes: &[Vec<char>],
    direction: &LineDirection,
    first_idx: usize,
    second_idx: usize,
) -> u32 {
    match direction {
        LineDirection::Horizontal => notes[first_idx]
            .iter()
            .zip(&notes[second_idx])
            .filter(|(c1, c2)| c1 != c2)
            .count() as u32,
        LineDirection::Vertical => notes
            .iter()
            .filter(|row| row[first_idx] != row[second_idx])
            .count() as u32,
    }
}

fn solve(input: &[Vec<Vec<char>>], n_smudges: u32) -> u32 {
    input
        .iter()
        .map(|notes| {
            if let Some(horizontal_line) =
                find_mirror_line(notes, &LineDirection::Horizontal, n_smudges)
            {
                (horizontal_line + 1) * 100
            } else if let Some(vertical_line) =
                find_mirror_line(notes, &LineDirection::Vertical, n_smudges)
            {
                vertical_line + 1
            } else {
                0
            }
        })
        .sum()
}
pub fn part_1(input: &[Vec<Vec<char>>]) -> u32 {
    solve(input, 0)
}

pub fn part_2(input: &[Vec<Vec<char>>]) -> u32 {
    solve(input, 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
";
    // Test for the `part_1` function
    #[test]
    fn test_zero_smudges() {
        assert_eq!(solve(&generator(INPUT), 0), 405);
    }

    // Test for the `part_2` function
    #[test]
    fn test_one_allowed_smudge() {
        assert_eq!(solve(&generator(INPUT), 1), 400);
    }
}
