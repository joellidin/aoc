use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::BufRead;
use std::io::BufReader;

fn main() {
    day2();
}

fn day1() {
    let file = File::open("data/day1.txt").unwrap();
    let reader = BufReader::new(file);

    let mut max_calories = [0; 3];
    let mut calories = 0;
    for line in reader.lines() {
        let cals = line.unwrap_or("".to_owned());
        if cals.is_empty() {
            for i in 0..max_calories.len() {
                if calories > max_calories[i] {
                    let old_max_c = max_calories[i];
                    max_calories[i] = calories;
                    calories = old_max_c;
                }
            }
            calories = 0;
            continue;
        }
        calories = calories + cals.parse::<i32>().unwrap();
    }
    println!(
        "The elf with max amount of calories has {} calories",
        max_calories[0]
    );

    println!(
        "The top three elfs with max amount of calories has {} calories",
        max_calories.iter().sum::<i32>()
    );
}

fn day2() {
    let mut winner_map = HashMap::new();
    winner_map.insert("A".to_owned(), "Y".to_owned());
    winner_map.insert("B".to_owned(), "Z".to_owned());
    winner_map.insert("C".to_owned(), "X".to_owned());

    let mut equal_map = HashMap::new();
    equal_map.insert("A".to_owned(), "X".to_owned());
    equal_map.insert("B".to_owned(), "Y".to_owned());
    equal_map.insert("C".to_owned(), "Z".to_owned());

    let mut scores = HashMap::new();
    scores.insert("X".to_owned(), 1);
    scores.insert("Y".to_owned(), 2);
    scores.insert("Z".to_owned(), 3);
    scores.insert("A".to_owned(), 1);
    scores.insert("B".to_owned(), 2);
    scores.insert("C".to_owned(), 3);

    let mut winner_map2 = HashMap::new();
    winner_map2.insert("A".to_owned(), "B".to_owned());
    winner_map2.insert("B".to_owned(), "C".to_owned());
    winner_map2.insert("C".to_owned(), "A".to_owned());

    let mut loser_map = HashMap::new();
    loser_map.insert("A".to_owned(), "C".to_owned());
    loser_map.insert("B".to_owned(), "A".to_owned());
    loser_map.insert("C".to_owned(), "B".to_owned());

    let file = File::open("data/day2.txt").unwrap();
    let reader = BufReader::new(file);

    let result: u32 = reader.lines().fold(0, |acc, line| {
        let line = line.unwrap();
        let (a, b) = line.split_once(" ").unwrap();
        let mut score = 0;
        if winner_map.get(a).unwrap() == b {
            score += 6;
        } else if equal_map.get(a).unwrap() == b {
            score += 3;
        }
        acc + scores.get(b).unwrap() + score
    });
    println!("The total score is {}", result);

    let file = File::open("data/day2.txt").unwrap();
    let reader = BufReader::new(file);
    let result: u32 = reader.lines().fold(0, |acc, line| {
        let line = line.unwrap();
        let (a, b) = line.split_once(" ").unwrap();
        let score = match b {
            "X" => *scores.get(loser_map.get(a).unwrap()).unwrap(),
            "Y" => 3 + scores.get(a).unwrap(),
            "Z" => 6 + *scores.get(winner_map2.get(a).unwrap()).unwrap(),
            _ => panic!("NO MATCH")
        };
        acc + score
    });
    println!("The total score is (part2) {}", result)
}
