use std::fs::File;
use std::io::prelude::BufRead;
use std::io::BufReader;

pub fn solution() {
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
