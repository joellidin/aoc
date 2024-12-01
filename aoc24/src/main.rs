#![recursion_limit = "256"]

mod day1;

aoc_main::main! {
    year 2024;
    day1 : generator => part_1, part_2;
}
