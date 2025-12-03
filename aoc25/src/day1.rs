use aoc_utils::prelude::*;

pub enum Dir {
    Left,
    Right,
}

pub fn generator(input: &str) -> Vec<(Dir, i32)> {
    input
        .lines()
        .map(|line| {
            let dir = match &line[0..1] {
                "L" => Dir::Left,
                "R" => Dir::Right,
                _ => panic!("Unknown direction"),
            };
            let num: i32 = line[1..].parse().unwrap();
            (dir, num)
        })
        .collect()
}

pub fn part_1(input: &[(Dir, i32)]) -> u32 {
    let mut res = 50;
    let mut n_zeros = 0;
    input.iter().for_each(|(dir, num)| {
        match dir {
            Dir::Left => res = (res - num).rem_euclid(100),
            Dir::Right => res = (res + num).rem_euclid(100),
        };
        if res == 0 {
            n_zeros += 1;
        }
    });
    n_zeros
}

pub fn part_2(input: &[(Dir, i32)]) -> u32 {
    let mut res: i32 = 50;
    let mut n_zeros: u32 = 0;
    input.iter().for_each(|(dir, num)| {
        match dir {
            Dir::Left => {
                if res == 0 {
                    n_zeros += (num / 100) as u32;
                } else if *num >= res {
                    n_zeros += 1 + ((num - res) / 100) as u32;
                }
                res = (res - num).rem_euclid(100);
            }
            Dir::Right => {
                if res == 0 {
                    n_zeros += (num / 100) as u32;
                } else if *num >= 100 - res {
                    n_zeros += 1 + ((num - (100 - res)) / 100) as u32;
                }
                res = (res + num).rem_euclid(100);
            }
        };
    });
    n_zeros
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
"#;

    #[test]
    fn part_1_test() {
        let generator_output = generator(INPUT);
        let result = part_1(&generator_output);
        assert_eq!(result, 3);
    }

    #[test]
    fn part_2_test() {
        let generator_output = generator(INPUT);
        let result = part_2(&generator_output);
        assert_eq!(result, 6);
    }
}
