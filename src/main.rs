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
        _ => panic!("Not a valid day for advent of code."),
    };
}
