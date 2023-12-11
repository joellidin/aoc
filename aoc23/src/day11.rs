pub fn generator(input: &str) -> (Vec<Vec<char>>, Vec<(usize, usize)>) {
    let mut galaxy_positions = Vec::new();
    let map = input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(|(j, c)| {
                    if c == '#' {
                        galaxy_positions.push((i, j));
                    }
                    c
                })
                .collect()
        })
        .collect();
    (map, galaxy_positions)
}

fn min_path_length(
    start: &(usize, usize),
    end: &(usize, usize),
    galaxy_factor: usize,
    empty_rows: &[bool],
    empty_columns: &[bool],
) -> u64 {
    (start.0.abs_diff(end.0)
        + (galaxy_factor - 1) * number_of_empty_items(empty_rows, start.0, end.0)
        + start.1.abs_diff(end.1)
        + (galaxy_factor - 1) * number_of_empty_items(empty_columns, start.1, end.1)) as u64
}

fn precompute_empty_spaces(map: &[Vec<char>]) -> (Vec<bool>, Vec<bool>) {
    let row_count = map.len();
    let col_count = map.first().map_or(0, Vec::len);

    let empty_rows = (0..row_count)
        .map(|row| map[row].iter().all(|&c| c == '.'))
        .collect();
    let empty_columns = (0..col_count)
        .map(|col| map.iter().all(|row| col < row.len() && row[col] == '.'))
        .collect();
    (empty_rows, empty_columns)
}

fn number_of_empty_items(empty_rows_or_col: &[bool], start: usize, end: usize) -> usize {
    let (lower, upper) = if start < end {
        (start, end)
    } else {
        (end, start)
    };

    empty_rows_or_col
        .iter()
        .skip(lower)
        .take(upper - lower)
        .filter(|&&is_empty| is_empty)
        .count()
}

fn sum_all_lengths(
    map: &[Vec<char>],
    galaxy_positions: &[(usize, usize)],
    galaxy_factor: usize,
) -> u64 {
    let (empty_rows, empty_columns) = precompute_empty_spaces(map);
    galaxy_positions
        .iter()
        .enumerate()
        .flat_map(|(i, start)| {
            galaxy_positions.iter().skip(i).map(|end| {
                min_path_length(
                    start,
                    end,
                    galaxy_factor,
                    empty_rows.as_ref(),
                    empty_columns.as_ref(),
                )
            })
        })
        .sum()
}
pub fn part_1((map, galaxy_positions): &(Vec<Vec<char>>, Vec<(usize, usize)>)) -> u64 {
    sum_all_lengths(map, galaxy_positions, 2)
}

pub fn part_2((map, galaxy_positions): &(Vec<Vec<char>>, Vec<(usize, usize)>)) -> u64 {
    sum_all_lengths(map, galaxy_positions, 1_000_000)
}
