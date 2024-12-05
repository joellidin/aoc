#![recursion_limit = "256"]

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

aoc_main::main! {
    year 2024;
    day1 : generator => part_1, part_2;
    day2 : generator => part_1, part_2;
    day3 : generator => part_1, part_2;
    day4 : generator => part_1, part_2;
    day5 : generator => part_1, part_2;
}
