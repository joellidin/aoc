#![recursion_limit = "256"]

mod day1;
mod day2;

aoc_main::main! {
    year 2021;
    day1 : generator => part_1, part_2;
    day2 : generator => part_1, part_2;
}
