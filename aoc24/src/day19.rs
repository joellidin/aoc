use std::collections::HashMap;

pub fn generator(input: &str) -> (Vec<&str>, Vec<&str>) {
    let (patterns_str, designs_str) = input.split_once("\n\n").expect("Invalid input");
    let patterns = patterns_str.split(", ").collect();
    let designs = designs_str.lines().collect();
    (patterns, designs)
}

fn find_combinations<'a>(
    patterns: &[&'a str],
    design: &'a str,
    index: usize,
    memo: &mut HashMap<usize, u64>,
) -> u64 {
    if let Some(res) = memo.get(&index) {
        return *res;
    }
    let res = patterns
        .iter()
        .filter(|&&pattern| design[index..].starts_with(pattern))
        .map(|pattern| {
            if index + pattern.len() == design.len() {
                1
            } else {
                find_combinations(patterns, design, index + pattern.len(), memo)
            }
        })
        .sum();
    memo.insert(index, res);
    res
}

pub fn part_1(input: &(Vec<&str>, Vec<&str>)) -> u64 {
    let (patterns, designs) = input;

    designs
        .iter()
        .filter(|design| {
            let mut memo = HashMap::new();
            find_combinations(patterns, design, 0, &mut memo) > 0
        })
        .count() as u64
}

pub fn part_2(input: &(Vec<&str>, Vec<&str>)) -> u64 {
    let (patterns, designs) = input;

    designs
        .iter()
        .map(|design| {
            let mut memo = HashMap::new();
            find_combinations(patterns, design, 0, &mut memo)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb
"#;

    #[test]
    fn part_1_test() {
        let generator_output = generator(INPUT);
        let result = part_1(&generator_output);
        assert_eq!(result, 6);
    }

    #[test]
    fn part_2_test() {
        let generator_output = generator(INPUT);
        let result = part_2(&generator_output);
        assert_eq!(result, 16);
    }
}
