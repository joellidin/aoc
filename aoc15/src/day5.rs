use std::collections::HashMap;

pub fn solution() {
    let input = include_str!("../data/day5.txt");
    println!(
        "Part 1 has {} nice strings",
        input.lines().fold(0, |acc, line| {
            let contains_3_vowels = line.matches(|c| "aeiou".contains(c)).count() >= 3;
            let no_disallowed_chars = !line.contains("xy")
                && !line.contains("ab")
                && !line.contains("cd")
                && !line.contains("pq");
            let chars = line.chars().collect::<Vec<char>>();
            let occuring_letter = chars.windows(2).any(|w| w[0] == w[1]);
            acc + (contains_3_vowels && no_disallowed_chars && occuring_letter) as usize
        })
    );
    println!(
        "Part 2 has {} nice strings",
        input.lines().fold(0, |acc, line| {
            let chars = line.chars().collect::<Vec<char>>();
            let mut seen_pairs = HashMap::new();
            let occuring_pair = chars.windows(2).enumerate().any(|(i, w)| {
                let seen = seen_pairs.entry(w).or_insert(i);
                *seen != i && *seen + 2 <= i
            });
            let occuring_letter = chars.windows(3).any(|w| w[0] == w[2]);
            acc + (occuring_pair && occuring_letter) as usize
        })
    );
}
