use std::collections::HashSet;

pub fn part_1(input: &str) -> u32 {
    input.lines().fold(0, |acc, line| {
        let char_count = &line.chars().count();
        let mut first_compartment_set = HashSet::new();
        let mut score = 0;
        for (i, char) in line.chars().enumerate() {
            if i < char_count / 2 {
                first_compartment_set.insert(char.to_owned());
            } else if first_compartment_set.contains(&char) {
                score += if char.is_lowercase() {
                    char as u32 - 96
                } else {
                    char as u32 - 38
                };
                break;
            }
        }
        acc + score
    })
}

pub fn part_2(input: &str) -> u32 {
    let mut i: u8 = 0;
    let mut intersection_set = HashSet::new();
    input.lines().fold(0, |acc, line| {
        let mut score = 0;
        if i == 0 {
            intersection_set = line.chars().collect();
        } else {
            let mut _next_set = HashSet::new();
            _next_set = line.chars().collect();
            intersection_set = intersection_set.intersection(&_next_set).copied().collect();
        }
        i += 1;
        if i == 3 {
            let char = intersection_set.iter().next().unwrap();
            score += if char.is_lowercase() {
                *char as u32 - 96
            } else {
                *char as u32 - 38
            };
            i = 0;
        }
        acc + score
    })
}
