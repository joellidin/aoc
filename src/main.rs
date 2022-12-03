use std::env;
mod day1;
mod day2;
mod day3;

fn main() {
    let args: Vec<String> = env::args().collect();
    let day_solution = &args[1];
    match day_solution.as_str() {
        "1" => day1::solution(),
        "2" => day2::solution(),
        "3" => day3::solution(),
        _ => panic!("Not a valid day for advent of code."),
    };
}
