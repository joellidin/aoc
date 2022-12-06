use std::collections::HashSet;

fn get_marker(n_unique: usize, input: &str) -> usize {
    let mut tot_chars = 0;
    let line_vec: Vec<char> = input.chars().collect();
    for i in n_unique - 1..line_vec.len() {
        let mut set = HashSet::new();
        for c in line_vec[i - (n_unique - 1)..=i].iter() {
            set.insert(c);
        }
        if set.len() == n_unique {
            tot_chars += i + 1;
            break;
        }
    }
    tot_chars
}

pub fn solution() {
    let input = std::fs::read_to_string("data/day6.txt").unwrap();
    println!(
        "You need to process {} character you get 4 unique chars",
        get_marker(4, &input)
    );
    println!(
        "You need to process {} character you get 14 unique chars",
        get_marker(14, &input)
    );
}
