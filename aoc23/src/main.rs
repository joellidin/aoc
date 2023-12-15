#![recursion_limit = "256"]

mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

aoc_main::main! {
    year 2023;
    day1  : generator => part_1, part_2;
    day2  : generator => part_1, part_2;
    day3  : generator => part_1, part_1_solution_2, part_2;
    day4  : generator => part_1, part_2;
    day5  : generator => part_1, part_2;
    day6  : generator => part_1, part_2;
    day7  : generator => part_1, part_2;
    day8  : generator => part_1, part_2;
    day9  : generator => part_1, part_2;
    day10 : generator => part_1, part_2;
    day11 : generator => part_1, part_2;
    day12 : generator => part_1, part_2;
    day13 : generator => part_1, part_2;
    day14 : generator => part_1, part_2;
    day15 : generator => part_1, part_2;
}
