use std::{fmt, str::FromStr};

#[derive(Clone)]
pub struct BingoBoard {
    numbers: Vec<Vec<u8>>,
    marked_numbers: [bool; 25],
}

impl FromStr for BingoBoard {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let numbers = s
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .map(|n| n.parse().expect("Should be a number."))
                    .collect()
            })
            .collect();
        let marked_numbers = [false; 25];
        Ok(BingoBoard {
            numbers,
            marked_numbers,
        })
    }
}

impl fmt::Debug for BingoBoard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, row) in self.numbers.iter().enumerate() {
            for (j, &num) in row.iter().enumerate() {
                let index = i * 5 + j;
                if self.marked_numbers[index] {
                    write!(f, "\x1b[1;32m{:2}\x1b[0m ", num)?;
                } else {
                    write!(f, "{:2} ", num)?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl BingoBoard {
    pub fn is_any_row_complete(&self) -> bool {
        for row in self.marked_numbers.chunks(5) {
            if row.iter().all(|&marked| marked) {
                return true;
            }
        }
        false
    }

    pub fn is_any_column_complete(&self) -> bool {
        for col in 0..5 {
            if (0..5).all(|row| self.marked_numbers[row * 5 + col]) {
                return true;
            }
        }
        false
    }

    pub fn sum_unmarked_numbers(&self) -> u32 {
        let mut sum = 0;
        for (i, row) in self.numbers.iter().enumerate() {
            for (j, &num) in row.iter().enumerate() {
                let index = i * 5 + j;
                if !self.marked_numbers[index] {
                    sum += num as u32;
                }
            }
        }
        sum
    }
}

pub fn generator(input: &str) -> (Vec<u8>, Vec<BingoBoard>) {
    let (numbers_input, boards_input) = input.split_once("\n\n").expect("Wrong format.");
    let numbers = numbers_input
        .split(',')
        .map(|n| n.parse().expect("Should be a number."))
        .collect();
    let boards = boards_input
        .split("\n\n")
        .map(|board| board.parse().expect("Must be parsable to a bingo board."))
        .collect();
    (numbers, boards)
}

pub fn part_1(input: &(Vec<u8>, Vec<BingoBoard>)) -> u32 {
    let len_boards = input.1[0].numbers.len();
    let (numbers, mut boards) = (&input.0, input.1.clone());
    let mut have_won = false;
    let mut final_number = u8::MAX;
    let mut winning_board_idx = usize::MAX;
    for number in numbers {
        boards.iter_mut().enumerate().for_each(|(i, board)| {
            board.numbers.iter().enumerate().for_each(|(j, row)| {
                if let Some(n) = row.iter().position(|n| n == number) {
                    board.marked_numbers[j * len_boards + n] = true;
                }
            });
            if !have_won {
                winning_board_idx = i
            }
            have_won |= board.is_any_row_complete() | board.is_any_column_complete();
        });
        if have_won {
            final_number = *number;
            break;
        }
    }
    boards[winning_board_idx].sum_unmarked_numbers() * final_number as u32
}

pub fn part_2(input: &(Vec<u8>, Vec<BingoBoard>)) -> u32 {
    let len_boards = input.1[0].numbers.len();
    let (numbers, mut boards) = (&input.0, input.1.clone());
    let mut have_won = vec![false; boards.len()];
    let mut final_number = u8::MAX;
    let mut winning_board_idx = usize::MAX;
    for number in numbers {
        boards.iter_mut().enumerate().for_each(|(i, board)| {
            board.numbers.iter().enumerate().for_each(|(j, row)| {
                if let Some(n) = row.iter().position(|n| n == number) {
                    board.marked_numbers[j * len_boards + n] = true;
                }
            });
            if !have_won.iter().all(|&w| w) {
                winning_board_idx = i
            }
            have_won[i] |= board.is_any_row_complete() | board.is_any_column_complete();
        });
        if have_won.iter().all(|&w| w) {
            final_number = *number;
            break;
        }
    }
    boards[winning_board_idx].sum_unmarked_numbers() * final_number as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7
"#;

    #[test]
    fn part_1_test() {
        let generator_output = generator(INPUT);
        let result = part_1(&generator_output);
        assert_eq!(result, 4512);
    }

    #[test]
    fn part_2_test() {
        let generator_output = generator(INPUT);
        let result = part_2(&generator_output);
        assert_eq!(result, 1924);
    }
}
