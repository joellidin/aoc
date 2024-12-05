use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    hash::Hash,
};

type OrderingRules = HashMap<u32, HashSet<u32>>;
type PageNumbers = Vec<Vec<u32>>;

pub fn generator(input: &str) -> (OrderingRules, PageNumbers) {
    let (ordering_rules_str, page_numbers_str) = input.split_once("\n\n").unwrap();

    let ordering_rules =
        ordering_rules_str
            .lines()
            .fold(HashMap::new(), |mut acc: HashMap<_, HashSet<_>>, line| {
                let (left, right) = line.split_once('|').unwrap();
                let n = left.parse::<u32>().unwrap();
                let m = right.parse::<u32>().unwrap();
                acc.entry(m).or_default().insert(n);
                acc
            });

    let page_numbers = page_numbers_str
        .lines()
        .map(|line| line.split(',').map(|n| n.parse().unwrap()).collect())
        .collect();

    (ordering_rules, page_numbers)
}

fn is_valid_update<T>(page_numbers: &[T], ordering_rules: &HashMap<T, HashSet<T>>) -> bool
where
    T: PartialEq + Eq + Hash,
{
    page_numbers.is_sorted_by(|n, m| {
        if let Some(m_values) = ordering_rules.get(n) {
            !m_values.contains(m)
        } else {
            true
        }
    })
}

fn reorder<T>(page_numbers: &mut [T], ordering_rules: &HashMap<T, HashSet<T>>)
where
    T: PartialOrd + Ord + Hash,
{
    page_numbers.sort_by(|n, m| {
        if let Some(m_values) = ordering_rules.get(n) {
            if m_values.contains(m) {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        } else {
            Ordering::Equal
        }
    });
}

pub fn part_1(input: &(OrderingRules, PageNumbers)) -> u32 {
    let (ordering_rules, page_numbers) = input;
    let filtered_page_numbers = page_numbers
        .iter()
        .filter(|nums| is_valid_update(nums, ordering_rules));
    filtered_page_numbers
        .map(|numbers| numbers.get(numbers.len() / 2).unwrap())
        .sum()
}

pub fn part_2(input: &(OrderingRules, PageNumbers)) -> u32 {
    let (ordering_rules, page_numbers) = input;
    page_numbers
        .iter()
        // Get all invalid page numbers
        .filter(|nums| !is_valid_update(nums, ordering_rules))
        .cloned()
        .fold(0, |acc, mut nums| {
            // Sort page numbers to the correct order
            reorder(&mut nums, ordering_rules);

            // Get the middle value of newly valid page numbers
            acc + nums[nums.len() / 2]
        })
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
