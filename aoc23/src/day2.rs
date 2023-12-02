use std::str::FromStr;

#[derive(Clone, Debug)]
pub struct Game {
    id: u32,
    reveals: Vec<(u32, u32, u32)>,
}

impl FromStr for Game {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (game_id, reveals_str) = s.split_once(':').unwrap();
        let id = game_id.strip_prefix("Game ").unwrap().parse().unwrap();
        let reveals = reveals_str
            .trim_end()
            .split(';')
            .map(|x| {
                let mut cubes_revealed = (0, 0, 0);
                x.split(',').for_each(|colors_str| {
                    let (s1, color) = colors_str.trim().split_once(' ').unwrap();
                    let n_cubes = s1.parse().unwrap_or(0);
                    if color == "red" {
                        cubes_revealed.0 = n_cubes;
                    } else if color == "green" {
                        cubes_revealed.1 = n_cubes;
                    } else {
                        cubes_revealed.2 = n_cubes;
                    }
                });
                cubes_revealed
            })
            .collect();
        Ok(Game { id, reveals })
    }
}
pub fn generator(input: &str) -> Vec<Game> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

pub fn part_1(input: &[Game]) -> u32 {
    let max_cubes = (12, 13, 14);
    input
        .iter()
        .filter(|game| {
            game.reveals
                .iter()
                .all(|r| r.0 <= max_cubes.0 && r.1 <= max_cubes.1 && r.2 <= max_cubes.2)
        })
        .fold(0, |acc, game| acc + game.id)
}

pub fn part_2(input: &[Game]) -> u32 {
    input.iter().map(|game| {
        let (min_r, min_g, min_b) = game.reveals
            .iter()
            .fold((1u32, 1u32, 1u32), |(min_r, min_g, min_b), (r, g, b)| {
                (
                    std::cmp::max(min_r, *r),
                    std::cmp::max(min_g, *g),
                    std::cmp::max(min_b, *b),
                )
            });
        min_r * min_g * min_b
    }).sum()
}
