use std::{iter::Sum, str::FromStr};
struct Snafu {
    base_10: isize,
    base_snafu: String,
}

impl FromStr for Snafu {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut num = 0;
        for c in s.chars() {
            match c {
                '0'..='2' => {
                    num = num * 5 + c.to_digit(5).unwrap() as isize;
                }
                '-' => {
                    num = num * 5 - 1;
                }
                '=' => {
                    num = num * 5 - 2;
                }
                _ => unreachable!(),
            }
        }
        Ok(Snafu {
            base_10: num,
            base_snafu: to_snafu(num),
        })
    }
}
impl Sum for Snafu {
    fn sum<I: Iterator<Item = Snafu>>(iter: I) -> Self {
        let num = iter.map(|s| s.base_10).sum();
        Snafu {
            base_10: num,
            base_snafu: to_snafu(num),
        }
    }
}

fn to_snafu(base_10: isize) -> String {
    let mut res = "".to_owned();
    let mut old_q = base_10;
    loop {
        let (q, r) = ((old_q + 2) / 5, (old_q + 2) % 5 - 2);
        let c = match r {
            0 => '0',
            1 => '1',
            2 => '2',
            -1 => '-',
            -2 => '=',
            _ => unreachable!(),
        };
        res = c.to_string() + &res;
        if q == 0 {
            break res;
        }
        old_q = q;
    }
}

pub fn solution() {
    println!(
        "We supply {} to Bob's console.",
        include_str!("../data/day25.txt")
            .lines()
            .map(|l| l.parse::<Snafu>().unwrap())
            .sum::<Snafu>()
            .base_snafu
    )
}
