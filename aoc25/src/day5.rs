use aoc_utils::prelude::*;

pub fn generator(input: &str) -> (Vec<(u64, u64)>, Vec<u64>) {
    let (all_ranges, all_ingredients) = input.split_once("\n\n").expect("Invalid format");

    let ranges = all_ranges
        .lines()
        .map(|range| {
            let &[x, y, ..] = extract_integers::<u64>(range).as_slice() else {
                panic!("Could not parse integers")
            };
            (x, y)
        })
        .collect();

    let ingredients = all_ingredients
        .lines()
        .map(|ingredient_id| ingredient_id.parse().expect("Not parsable to a number"))
        .collect();

    (ranges, ingredients)
}

pub fn part_1(input: &(Vec<(u64, u64)>, Vec<u64>)) -> u64 {
    let (ranges, ingredients) = input;
    ingredients
        .iter()
        .filter(|&&id| {
            for range in ranges {
                if id >= range.0 && id <= range.1 {
                    return true;
                }
            }
            false
        })
        .count() as u64
}

pub fn part_2(input: &(Vec<(u64, u64)>, Vec<u64>)) -> u64 {
    let (ranges, _) = input;
    let mut sorted = ranges.clone();
    sorted.sort_by_key(|r| r.0);

    let mut merged: Vec<(u64, u64)> = Vec::new();
    for (start, end) in sorted {
        if let Some(last) = merged.last_mut() {
            if start <= last.1 + 1 {
                // Overlapping or adjacent - extend
                last.1 = last.1.max(end);
            } else {
                merged.push((start, end));
            }
        } else {
            merged.push((start, end));
        }
    }
    merged.iter().map(|(start, end)| end - start + 1).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"3-5
10-14
16-20
12-18

1
5
8
11
17
32
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
        assert_eq!(result, 14);
    }
}
