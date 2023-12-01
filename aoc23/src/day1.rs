use std::collections::HashMap;

pub fn generator(input: &str) -> Vec<Vec<&str>> {
    let numbers = input
        .lines()
        .map(|line| {
            let copy_line = line.clone();
            line.chars()
                .enumerate()
                .filter_map(|(i, c)| match c {
                    'o' => {
                        if copy_line.get(i..i + 3) == Some("one") {
                            Some("one")
                        } else {
                            None
                        }
                    }
                    't' => {
                        if copy_line.get(i..i + 3) == Some("two") {
                            Some("two")
                        } else if copy_line.get(i..i + 5) == Some("three") {
                            Some("three")
                        } else {
                            None
                        }
                    }
                    'f' => {
                        if copy_line.get(i..i + 4) == Some("four") {
                            Some("four")
                        } else if copy_line.get(i..i + 4) == Some("five") {
                            Some("five")
                        } else {
                            None
                        }
                    }
                    's' => {
                        if copy_line.get(i..i + 3) == Some("six") {
                            Some("six")
                        } else if copy_line.get(i..i + 5) == Some("seven") {
                            Some("seven")
                        } else {
                            None
                        }
                    }
                    'e' => {
                        if copy_line.get(i..i + 5) == Some("eight") {
                            Some("eight")
                        } else {
                            None
                        }
                    }
                    'n' => {
                        if copy_line.get(i..i + 4) == Some("nine") {
                            Some("nine")
                        } else {
                            None
                        }
                    }
                    c if c.is_ascii_digit() => copy_line.get(i..i + 1),
                    _ => None,
                })
                .collect()
        })
        .collect();
    numbers
}

pub fn part_1(input: &[Vec<&str>]) -> u32 {
    let only_numbers = input
        .iter()
        .map(|v| v.iter().filter_map(|s| s.parse::<u32>().ok()).collect())
        .collect::<Vec<Vec<u32>>>();
    only_numbers
        .iter()
        .filter(|v| !v.is_empty())
        .map(|v| v[0] * 10 + v.last().unwrap())
        .sum()
}

pub fn part_2(input: &[Vec<&str>]) -> u32 {
    let number_words = HashMap::from([
        ("one", 1u32),
        ("two", 2u32),
        ("three", 3u32),
        ("four", 4u32),
        ("five", 5u32),
        ("six", 6u32),
        ("seven", 7u32),
        ("eight", 8u32),
        ("nine", 9u32),
    ]);
    let numbers: Vec<Vec<u32>> = input.iter().map(|v| v.iter().map(|n| if n.len() == 1 {
        n.parse().unwrap()
    } else {
        number_words[n]
    }).collect()).collect();
    numbers
        .iter()
        .map(|v| v[0] * 10 + v.last().unwrap())
        .sum()
}
