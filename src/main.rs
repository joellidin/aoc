use std::env;

mod day1;

fn main() {
    let args: Vec<String> = env::args().collect();
    let day_solution = &args[1];
    match day_solution.as_str() {
        "1" => day1::solution(),
        s => {
            let num = s.parse::<usize>().expect("Expecting a positive number");
            if num > 25 {
                panic!("Not a valid day for advent of code. Input a number between 1 and 25.");
            }
            panic!("This day is not solved yet.");
        }
    };
}
