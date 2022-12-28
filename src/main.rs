use std::env;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

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
        s => {
            let num = s.parse::<usize>().expect("Expecting a positive number");
            if num > 25 {
                panic!("Not a valid day for advent of code. Input a number between 1 and 25.");
            }
            panic!("This day is not solved yet.");
        }
    };
}
