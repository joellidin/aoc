pub fn generator(input: &str) -> Vec<(u32, u32, u32, u32)> {
    input
        .trim()
        .lines()
        .map(|line| {
            let (a, b) = line.split_once(',').unwrap();
            let mut a = a.split('-').map(|num| num.parse::<u32>().unwrap());
            let mut b = b.split('-').map(|num| num.parse::<u32>().unwrap());
            let (a_min, a_max) = (a.next().unwrap(), a.next().unwrap());
            let (b_min, b_max) = (b.next().unwrap(), b.next().unwrap());
            (a_min, a_max, b_min, b_max)
        })
        .collect()
}

pub fn part_1(input: &[(u32, u32, u32, u32)]) -> u32 {
    input.iter().fold(0, |acc, (a_min, a_max, b_min, b_max)| {
        acc + (((a_min <= b_min) && (a_max >= b_max)) || ((b_min <= a_min) && (b_max >= a_max)))
            as u32
    })
}

pub fn part_2(input: &[(u32, u32, u32, u32)]) -> u32 {
    input.iter().fold(0, |acc, (a_min, a_max, b_min, b_max)| {
        acc + ((a_max >= b_min) && (b_max >= a_min)) as u32
    })
}
