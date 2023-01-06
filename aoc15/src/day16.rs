use std::str::FromStr;

#[derive(Debug)]
struct Sue {
    children: Option<u32>,
    cats: Option<u32>,
    samoyeds: Option<u32>,
    pomeranians: Option<u32>,
    akitas: Option<u32>,
    vizslas: Option<u32>,
    goldfish: Option<u32>,
    trees: Option<u32>,
    cars: Option<u32>,
    perfumes: Option<u32>,
}

fn get_number_from(item: &str, s: &str) -> Option<u32> {
    if s.contains(item) {
        let mut parts = s.split(item);
        parts.next();
        let number = parts
            .next()
            .unwrap()
            .trim_start_matches(": ")
            .split(',')
            .next()
            .unwrap()
            .parse()
            .unwrap();
        Some(number)
    } else {
        None
    }
}
impl FromStr for Sue {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let children = get_number_from("children", s);
        let cats = get_number_from("cats", s);
        let samoyeds = get_number_from("samoyeds", s);
        let pomeranians = get_number_from("pomeranians", s);
        let akitas = get_number_from("akitas", s);
        let vizslas = get_number_from("vizslas", s);
        let goldfish = get_number_from("goldfish", s);
        let trees = get_number_from("trees", s);
        let cars = get_number_from("cars", s);
        let perfumes = get_number_from("perfumes", s);
        Ok(Sue {
            children,
            cats,
            samoyeds,
            pomeranians,
            akitas,
            vizslas,
            goldfish,
            trees,
            cars,
            perfumes,
        })
    }
}

fn is_the_real_sue(real_sue: &Sue, other_sue: &Sue) -> bool {
    other_sue.children.unwrap_or(real_sue.children.unwrap()) == real_sue.children.unwrap()
        && other_sue.cats.unwrap_or(real_sue.cats.unwrap()) == real_sue.cats.unwrap()
        && other_sue.samoyeds.unwrap_or(real_sue.samoyeds.unwrap()) == real_sue.samoyeds.unwrap()
        && other_sue
            .pomeranians
            .unwrap_or(real_sue.pomeranians.unwrap())
            == real_sue.pomeranians.unwrap()
        && other_sue.akitas.unwrap_or(real_sue.akitas.unwrap()) == real_sue.akitas.unwrap()
        && other_sue.vizslas.unwrap_or(real_sue.vizslas.unwrap()) == real_sue.vizslas.unwrap()
        && other_sue.goldfish.unwrap_or(real_sue.goldfish.unwrap()) == real_sue.goldfish.unwrap()
        && other_sue.trees.unwrap_or(real_sue.trees.unwrap()) == real_sue.trees.unwrap()
        && other_sue.cars.unwrap_or(real_sue.cars.unwrap()) == real_sue.cars.unwrap()
        && other_sue.perfumes.unwrap_or(real_sue.perfumes.unwrap()) == real_sue.perfumes.unwrap()
}

fn is_the_real_real_sue(real_sue: &Sue, other_sue: &Sue) -> bool {
    other_sue.children.unwrap_or(real_sue.children.unwrap()) == real_sue.children.unwrap()
        && other_sue.cats.unwrap_or(u32::max_value()) > real_sue.cats.unwrap()
        && other_sue.samoyeds.unwrap_or(real_sue.samoyeds.unwrap()) == real_sue.samoyeds.unwrap()
        && other_sue.pomeranians.unwrap_or(u32::min_value()) < real_sue.pomeranians.unwrap()
        && other_sue.akitas.unwrap_or(real_sue.akitas.unwrap()) == real_sue.akitas.unwrap()
        && other_sue.vizslas.unwrap_or(real_sue.vizslas.unwrap()) == real_sue.vizslas.unwrap()
        && other_sue.goldfish.unwrap_or(u32::min_value()) < real_sue.goldfish.unwrap()
        && other_sue.trees.unwrap_or(u32::max_value()) > real_sue.trees.unwrap()
        && other_sue.cars.unwrap_or(real_sue.cars.unwrap()) == real_sue.cars.unwrap()
        && other_sue.perfumes.unwrap_or(real_sue.perfumes.unwrap()) == real_sue.perfumes.unwrap()
}

impl Default for Sue {
    fn default() -> Self {
        Sue {
            children: Some(3),
            cats: Some(7),
            samoyeds: Some(2),
            pomeranians: Some(3),
            akitas: Some(0),
            vizslas: Some(0),
            goldfish: Some(5),
            trees: Some(3),
            cars: Some(2),
            perfumes: Some(1),
        }
    }
}

pub fn solution() {
    let sues = include_str!("../data/day16.txt")
        .lines()
        .map(|line| line.parse::<Sue>().unwrap())
        .collect::<Vec<_>>();
    let the_sue = Sue::default();
    println!(
        "It was Sue number {} that got me the gift",
        sues.iter()
            .position(|sue| is_the_real_sue(&the_sue, sue))
            .unwrap()
            + 1
    );
    println!(
        "It was really Sue number {} that got me the gift",
        sues.iter()
            .position(|sue| is_the_real_real_sue(&the_sue, sue))
            .unwrap()
            + 1
    );
}
