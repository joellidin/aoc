use std::collections::HashSet;

pub fn generator(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

fn is_adjacent(input: &[Vec<char>], i: usize, j: usize) -> bool {
    let rows = input.len();
    let cols = input[0].len();
    for n in i.saturating_sub(1)..=usize::min(i + 1, rows - 1) {
        for m in j.saturating_sub(1)..=usize::min(j + 1, cols - 1) {
            if n == i && m == j {
                continue; // Skip the current cell
            }
            match input.get(n).and_then(|row| row.get(m)) {
                Some(&'.' | &('0'..='9')) => continue,
                Some(_) => return true,
                None => continue,
            }
        }
    }
    false
}

fn find_adjacent_numbers(schematic: &[Vec<char>], position: (usize, usize)) -> Vec<u32> {
    let mut adjacent_numbers = Vec::new();
    let mut added_number_starts = Vec::new();
    let (x, y) = position;
    let rows = schematic.len();
    let cols = schematic[0].len();

    (x.saturating_sub(1)..=x.saturating_add(1)).for_each(|i| {
        (y.saturating_sub(1)..=y.saturating_add(1)).for_each(|j| {
            if i >= rows || j >= cols || (i == x && j == y) {
                return;
            }
            let c = schematic[i][j];
            if c.is_ascii_digit() {
                let mut k = j;
                while k > 0 && schematic[i][k - 1].is_ascii_digit() {
                    k -= 1;
                }

                if !added_number_starts.contains(&(i, k)) {
                    let mut num = 0;
                    let mut m = k;
                    while m < cols && schematic[i][m].is_ascii_digit() {
                        num = num * 10 + schematic[i][m].to_digit(10).unwrap();
                        m += 1;
                    }
                    adjacent_numbers.push(num);
                    added_number_starts.push((i, k));
                }
            }
        });
    });
    adjacent_numbers
}

pub fn part_1(input: &[Vec<char>]) -> u32 {
    let mut sum = 0;
    let rows = input.len();
    let cols = input[0].len();

    (0..rows).for_each(|i| {
        let mut number = 0;
        let mut any_adjacent = false;
        for j in 0..cols {
            let c = input[i][j];
            match c {
                '0'..='9' => {
                    number = number * 10 + c.to_digit(10).unwrap();
                    if !any_adjacent {
                        any_adjacent = is_adjacent(input, i, j)
                    }
                }
                _ => {
                    if any_adjacent {
                        sum += number;
                    }
                    number = 0;
                    any_adjacent = false;
                }
            }
        }

        if any_adjacent {
            sum += number;
        }
    });
    sum
}

pub fn part_1_solution_2(input: &[Vec<char>]) -> u32 {
    input
        .iter()
        .enumerate()
        .flat_map(|(i, v)| {
            v.iter().enumerate().filter_map(move |(j, c)| match c {
                '0'..='9' | '.' => None,
                _ => Some((i, j)),
            })
        })
        .map(|pos| find_adjacent_numbers(input, pos).iter().sum::<u32>())
        .sum()
}

pub fn part_2(input: &[Vec<char>]) -> u32 {
    let gear_positions: HashSet<(usize, usize)> = input
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter().enumerate().filter_map(
                move |(j, c)| {
                    if c == &'*' {
                        Some((i, j))
                    } else {
                        None
                    }
                },
            )
        })
        .collect();

    gear_positions.iter().fold(0, |acc, (i, j)| {
        let adjacent_numbers = find_adjacent_numbers(input, (*i, *j));
        if adjacent_numbers.len() == 2 {
            return acc + adjacent_numbers[0] * adjacent_numbers[1];
        }
        acc
    })
}
