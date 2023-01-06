use std::str::FromStr;

#[derive(Debug)]
struct Ingridient {
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

impl FromStr for Ingridient {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        let capacity = parts.nth(2).unwrap().trim_end_matches(',').parse().unwrap();
        let durability = parts.nth(1).unwrap().trim_end_matches(',').parse().unwrap();
        let flavor = parts.nth(1).unwrap().trim_end_matches(',').parse().unwrap();
        let texture = parts.nth(1).unwrap().trim_end_matches(',').parse().unwrap();
        let calories = parts.nth(1).unwrap().parse().unwrap();
        Ok(Ingridient {
            capacity,
            durability,
            flavor,
            texture,
            calories,
        })
    }
}

fn get_splits(number: u32, split_size: u32) -> Vec<Vec<u32>> {
    if split_size == 1 {
        return vec![vec![number]];
    }

    (0..=number)
        .flat_map(|x| {
            let mut v = get_splits(number - x, split_size - 1);
            for combination in &mut v {
                combination.push(x);
            }
            v
        })
        .collect::<Vec<_>>()
}

fn get_best_total_score(
    ingridients: &[Ingridient],
    tot_ingridients: u32,
    filter: Option<u32>,
) -> u32 {
    let splits = get_splits(tot_ingridients, ingridients.len() as u32);
    let mut best_score = 0;
    splits.iter().for_each(|permutation| {
        let mut capacity = 0;
        let mut durability = 0;
        let mut flavor = 0;
        let mut texture = 0;
        let mut calories = 0;
        ingridients.iter().enumerate().for_each(|(i, ingridient)| {
            capacity += ingridient.capacity * permutation[i] as i32;
            durability += ingridient.durability * permutation[i] as i32;
            flavor += ingridient.flavor * permutation[i] as i32;
            texture += ingridient.texture * permutation[i] as i32;
            calories += ingridient.calories * permutation[i] as i32;
        });
        let score = capacity.max(0) * durability.max(0) * flavor.max(0) * texture.max(0);
        if filter.is_none() || calories == filter.unwrap() as i32 {
            best_score = best_score.max(score as u32);
        }
    });
    best_score
}

pub fn solution() {
    let ingridients = include_str!("../data/day15.txt")
        .lines()
        .map(|line| line.parse::<Ingridient>().unwrap())
        .collect::<Vec<_>>();
    println!(
        "Total score of highest-scoring cookie: {}",
        get_best_total_score(&ingridients, 100, None)
    );
    println!(
        "Total score of the highest-scoring cookie with 500 calories: {}",
        get_best_total_score(&ingridients, 100, Some(500))
    );
}
