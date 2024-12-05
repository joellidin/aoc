use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    hash::Hash,
    rc::Rc,
};

#[derive(Clone, Eq)]
pub struct OrderedItem<T>
where
    T: Hash,
{
    value: T,
    ordering_rules: Rc<HashMap<T, HashSet<T>>>,
}

impl<T> PartialEq for OrderedItem<T>
where
    T: Eq + Hash,
{
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<T> PartialOrd for OrderedItem<T>
where
    T: Eq + Hash,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> Ord for OrderedItem<T>
where
    T: Eq + Hash,
{
    fn cmp(&self, other: &Self) -> Ordering {
        if let Some(m_values) = self.ordering_rules.get(&self.value) {
            if m_values.contains(&other.value) {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        } else {
            Ordering::Equal
        }
    }
}

type Updates<'a, T> = Vec<Vec<OrderedItem<T>>>;

pub fn generator(input: &str) -> Updates<u32> {
    let (ordering_rules_str, update_str) = input.split_once("\n\n").unwrap();

    let ordering_rules = Rc::new(ordering_rules_str.lines().fold(
        HashMap::new(),
        |mut acc: HashMap<_, HashSet<_>>, line| {
            let (left, right) = line.split_once('|').unwrap();
            let n = left.parse::<u32>().unwrap();
            let m = right.parse::<u32>().unwrap();
            acc.entry(m).or_default().insert(n);
            acc
        },
    ));

    let updates = update_str
        .lines()
        .map(|line| {
            line.split(',')
                .map(|n| OrderedItem {
                    value: n.parse::<u32>().unwrap(),
                    ordering_rules: Rc::clone(&ordering_rules),
                })
                .collect()
        })
        .collect();
    updates
}

pub fn part_1(input: &Updates<u32>) -> u32 {
    let filtered_updates = input.iter().filter(|nums| nums.is_sorted());
    filtered_updates
        .map(|numbers| numbers.get(numbers.len() / 2).unwrap().value)
        .sum()
}

pub fn part_2(input: &Updates<u32>) -> u32 {
    input
        .iter()
        // Get all invalid page numbers
        .filter(|nums| !nums.is_sorted())
        .cloned()
        .fold(0, |acc, mut nums| {
            // Sort page numbers to the correct order
            nums.sort_unstable();

            // Get the middle value of newly valid page numbers
            acc + nums[nums.len() / 2].value
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
