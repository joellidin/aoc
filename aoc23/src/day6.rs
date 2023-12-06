pub fn generator(input: &str) -> Vec<(u64, u64)> {
    let mut number_lines = input.lines().map(|line| {
        line.split_once(':')
            .unwrap()
            .1
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect::<Vec<u64>>()
    });
    number_lines
        .next()
        .unwrap()
        .iter()
        .zip(number_lines.next().unwrap().iter())
        .map(|(&n, &m)| (n, m))
        .collect()
}

pub fn is_winning(time: u64, max_time: u64, max_score: u64) -> bool {
    (max_time - time) * time > max_score
}

fn binary_search_first_win(max_time: u64, max_score: u64) -> u64 {
    let mut low = 0;
    let mut high = max_time;
    while low < high {
        let mid = low + (high - low) / 2;
        if is_winning(mid, max_time, max_score) {
            high = mid;
        } else {
            low = mid + 1;
        }
    }
    low
}

fn binary_search_last_win(max_time: u64, max_score: u64) -> u64 {
    let mut low = 0;
    let mut high = max_time;
    while low < high {
        let mid = low + (high - low + 1) / 2;
        if is_winning(mid, max_time, max_score) {
            low = mid;
        } else {
            high = mid - 1;
        }
    }
    high
}

pub fn part_1(input: &[(u64, u64)]) -> u64 {
    input
        .iter()
        .map(|&(max_time, max_score)| {
            let first_win = binary_search_first_win(max_time, max_score);
            let last_win = binary_search_last_win(max_time, max_score);
            last_win.saturating_sub(first_win) + 1
        })
        .product()
}

pub fn part_2(input: &[(u64, u64)]) -> u64 {
    let (max_time_str, max_score_str) = input
        .iter()
        .fold(("".to_owned(), "".to_owned()), |(acc_n, acc_m), (n, m)| {
            (acc_n + &n.to_string(), acc_m + &m.to_string())
        });
    let (max_time, max_score) = (
        max_time_str.parse::<u64>().unwrap(),
        max_score_str.parse::<u64>().unwrap(),
    );
    let first_win = binary_search_first_win(max_time, max_score);
    let last_win = binary_search_last_win(max_time, max_score);
    last_win.saturating_sub(first_win) + 1
}
