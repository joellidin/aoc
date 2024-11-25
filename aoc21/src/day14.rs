use core::str;
use std::collections::HashMap;

pub fn generator(input: &str) -> (String, HashMap<&str, &str>) {
    let (polymer_template, pair_insertions) = input.split_once("\n\n").expect("Invalid input");
    let pairs = pair_insertions
        .lines()
        .map(|line| line.split_once(" -> ").expect("Could not parse pairs"))
        .collect();
    (polymer_template.to_owned(), pairs)
}

fn count_pairs(
    polymer_template: &str,
    pair_insertions: &HashMap<&str, &str>,
    iterations: u32,
) -> HashMap<String, u64> {
    let mut pair_counts = HashMap::new();
    polymer_template.as_bytes().windows(2).for_each(|pair| {
        let pair_string = std::str::from_utf8(pair).expect("Pair should be parsable to string");
        *pair_counts.entry(pair_string.to_owned()).or_default() += 1;
    });

    for _ in 0..iterations {
        let mut new_pair_counts = HashMap::new();
        pair_counts.iter().for_each(|(pair, count)| {
            let insertion = pair_insertions
                .get(pair.as_str())
                .expect("Pair must exist in map");
            let first_new_pair = format!("{}{}", &pair[0..1], insertion);
            let second_new_pair = format!("{}{}", insertion, &pair[1..2]);
            *new_pair_counts.entry(first_new_pair).or_default() += count;
            *new_pair_counts.entry(second_new_pair).or_default() += count;
        });
        pair_counts = new_pair_counts;
    }
    pair_counts
}

fn find_element_count(
    pair_count: &HashMap<String, u64>,
    original_template: &str,
) -> HashMap<char, u64> {
    let mut elements_count = HashMap::new();
    pair_count.iter().for_each(|(pair, count)| {
        // Only the first of the pair to avoid double count
        let ch = pair
            .chars()
            .next()
            .expect("First character in pair must exist");
        *elements_count.entry(ch).or_default() += count
    });
    let last_char = original_template.chars().last().expect("Original template should not be empty");
    *elements_count.entry(last_char).or_default() += 1;
    elements_count
}

pub fn part_1(input: &(String, HashMap<&str, &str>)) -> u64 {
    let (polymer_template, pair_insertions) = input.clone();
    let pair_counts = count_pairs(&polymer_template, &pair_insertions, 10);
    let element_counts = find_element_count(&pair_counts, &polymer_template);
    let max_count = element_counts
        .values()
        .max()
        .expect("Element count should not be empty");
    let min_count = element_counts
        .values()
        .min()
        .expect("Element count should not be empty");
    max_count - min_count
}

pub fn part_2(input: &(String, HashMap<&str, &str>)) -> u64 {
    let (polymer_template, pair_insertions) = input.clone();
    let pair_counts = count_pairs(&polymer_template, &pair_insertions, 40);
    let element_counts = find_element_count(&pair_counts, &polymer_template);
    let max_count = element_counts
        .values()
        .max()
        .expect("Element count should not be empty");
    let min_count = element_counts
        .values()
        .min()
        .expect("Element count should not be empty");
    max_count - min_count
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C
"#;

    #[test]
    fn part_1_test() {
        let generator_output = generator(INPUT);
        let result = part_1(&generator_output);
        assert_eq!(result, 1588);
    }

    #[test]
    fn part_2_test() {
        let generator_output = generator(INPUT);
        let result = part_2(&generator_output);
        assert_eq!(result, 2188189693529);
    }
}
