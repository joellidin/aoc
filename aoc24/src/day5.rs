use std::{collections::HashMap, hash::Hash};

type OrderingRules = HashMap<u32, Vec<u32>>;
type PageNumbers = Vec<Vec<u32>>;

pub fn generator(input: &str) -> (OrderingRules, PageNumbers) {
    let (ordering_rules_str, page_numbers_str) = input.split_once("\n\n").unwrap();
    let mut ordering_rules = HashMap::new();
    ordering_rules_str.lines().for_each(|line| {
        let (left, right) = line.split_once('|').unwrap();
        let (n, m) = (left.parse::<u32>().unwrap(), right.parse::<u32>().unwrap());
        ordering_rules.entry(m).or_insert_with(Vec::new).push(n);
    });
    let page_numbers = page_numbers_str
        .lines()
        .map(|line| line.split(',').map(|n| n.parse().unwrap()).collect())
        .collect();
    (ordering_rules, page_numbers)
}

fn get_invalid_indices<T>(
    page_numbers: &[T],
    ordering_rules: &HashMap<T, Vec<T>>,
) -> Option<(usize, usize)>
where
    T: PartialEq + Eq + Hash,
{
    for (i, n) in page_numbers.iter().enumerate().rev() {
        if let Some(order_numbers) = ordering_rules.get(n) {
            for m in order_numbers.iter() {
                if let Some(found_idx) = page_numbers.iter().skip(i).position(|k| m == k) {
                    return Some((i, found_idx + i));
                }
            }
        }
    }
    None
}

pub fn part_1(input: &(OrderingRules, PageNumbers)) -> u32 {
    let (ordering_rules, page_numbers) = input;
    let filtered_page_numbers = page_numbers
        .iter()
        .filter(|nums| get_invalid_indices(nums, ordering_rules).is_none());
    filtered_page_numbers
        .map(|numbers| numbers.get(numbers.len() / 2).unwrap())
        .sum()
}

pub fn part_2(input: &(OrderingRules, PageNumbers)) -> u32 {
    let (ordering_rules, page_numbers) = input;
    let mut incorrect_page_numbers = page_numbers
        .iter()
        .filter(|nums| get_invalid_indices(nums, ordering_rules).is_some())
        .cloned()
        .collect::<Vec<_>>();

    incorrect_page_numbers.iter_mut().for_each(|nums| {
        while let Some((i, j)) = get_invalid_indices(nums, ordering_rules) {
            nums.swap(i, j);
        }
    });

    incorrect_page_numbers
        .iter()
        .filter_map(|numbers| numbers.get(numbers.len() / 2))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
"#;

    #[test]
    fn part_1_test() {
        let generator_output = generator(INPUT);
        let result = part_1(&generator_output);
        assert_eq!(result, 143);
    }

    #[test]
    fn part_2_test() {
        let generator_output = generator(INPUT);
        let result = part_2(&generator_output);
        assert_eq!(result, 123);
    }
}
