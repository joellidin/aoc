use std::env;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;

fn main() {
    let args: Vec<String> = env::args().collect();
    let day_solution = &args[1];
    match day_solution.as_str() {
        "1" => day1::solution(),
        "2" => day2::solution(),
        "3" => day3::solution(),
        "4" => day4::solution(),
        "5" => day5::solution(),
        "6" => day6::solution(),
        "7" => day7::solution(),
        "8" => day8::solution(),
        "9" => day9::solution(),
        "10" => day10::solution(),
        "11" => day11::solution(),
        "12" => day12::solution(),
        "13" => day13::solution(),
        "14" => day14::solution(),
        "15" => day15::solution(),
        "16" => day16::solution(),
        "17" => day17::solution(),
        "18" => day18::solution(),
        "19" => day19::solution(),
        "20" => day20::solution(),
        s => {
            let num = s.parse::<usize>().expect("Expecting a positive number");
            if num > 25 {
                panic!("Not a valid day for advent of code. Input a number between 1 and 25.");
            }
            panic!("This day is not solved yet.");
        }
    };
}
