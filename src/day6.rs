fn get_marker(n_unique: usize, input: &str) -> usize {
    input
        .as_bytes()
        .windows(n_unique)
        .position(|bytes| {
            bytes
                .iter()
                .try_fold(0usize, |mut set, char| {
                    let prev = set;
                    set |= 1 << (char - b'a');
                    if prev == set {
                        return None;
                    }
                    Some(set)
                })
                .is_some()
        })
        .unwrap()
        + n_unique
}

pub fn solution() {
    let input = std::fs::read_to_string("data/day6.txt").unwrap();
    println!(
        "You need to process {} character before you get 4 unique chars",
        get_marker(4, &input)
    );
    println!(
        "You need to process {} character before you get 14 unique chars",
        get_marker(14, &input)
    );
}
