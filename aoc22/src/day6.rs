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

pub fn part_1(input: &str) -> usize {
    get_marker(4, input)
}

pub fn part_2(input: &str) -> usize {
    get_marker(14, input)
}

