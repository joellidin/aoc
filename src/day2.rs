use std::cmp::Ordering;
use std::fs::File;
use std::io::prelude::BufRead;
use std::io::BufReader;
use std::str::FromStr;
use Shape::*;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl PartialOrd for Shape {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Rock, Paper) | (Paper, Scissors) | (Scissors, Rock) => Some(Ordering::Less),
            (Scissors, Paper) | (Paper, Rock) | (Rock, Scissors) => Some(Ordering::Greater),
            (_, _) => Some(Ordering::Equal),
        }
    }
}

impl From<Shape> for u32 {
    fn from(shape: Shape) -> Self {
        match shape {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }
}

impl FromStr for Shape {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Rock),
            "B" | "Y" => Ok(Paper),
            "C" | "Z" => Ok(Scissors),
            _ => unreachable!(),
        }
    }
}

impl Shape {
    fn get_winner_shape(&self) -> Shape {
        match self {
            Rock => Scissors,
            Paper => Rock,
            Scissors => Paper,
        }
    }

    fn get_loser_shape(&self) -> Shape {
        match self {
            Rock => Paper,
            Paper => Scissors,
            Scissors => Rock,
        }
    }
}

pub fn solution() {
    let file = File::open("data/day2.txt").unwrap();
    let reader = BufReader::new(file);
    let result: u32 = reader.lines().fold(0, |acc, line| {
        let line = line.unwrap();
        let (a, b) = line.split_once(' ').unwrap();
        let (a, b) = (Shape::from_str(a).unwrap(), Shape::from_str(b).unwrap());
        let score = if a < b {
            6
        } else if a == b {
            3
        } else {
            0
        };
        acc + u32::from(b) + score
    });
    println!("The total score is {}", result);

    // Part 2
    let file = File::open("data/day2.txt").unwrap();
    let reader = BufReader::new(file);
    let result: u32 = reader.lines().fold(0, |acc, line| {
        let line = line.unwrap();
        let (a, b) = line.split_once(' ').unwrap();
        let a = Shape::from_str(a).unwrap();
        let score = match b {
            "X" => u32::from(a.get_winner_shape()),
            "Y" => 3 + u32::from(a),
            "Z" => 6 + u32::from(a.get_loser_shape()),
            _ => panic!("NO MATCH"),
        };
        acc + score
    });
    println!("The total score is (part2) {}", result)
}
