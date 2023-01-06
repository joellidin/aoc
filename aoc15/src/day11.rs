use std::collections::HashSet;

fn is_valid_password(s: &str) -> bool {
    let chars = s.chars().collect::<Vec<_>>();
    let contains_iol = s.contains(|c| c == 'i' || c == 'o' || c == 'l');
    let valid_pairs: HashSet<_> = HashSet::from_iter(chars.windows(2).filter(|w| w[0] == w[1]));
    let has_two_pairs = valid_pairs.len() >= 2;
    let has_straight = chars
        .windows(3)
        .any(|w| w[0] as u8 + 1 == w[1] as u8 && w[1] as u8 + 1 == w[2] as u8);
    !contains_iol && has_two_pairs && has_straight
}

fn increment_password(s: &str) -> String {
    let mut chars = s.chars().collect::<Vec<_>>();
    let mut i = chars.len() - 1;
    loop {
        if chars[i] == 'z' {
            chars[i] = 'a';
            i -= 1;
        } else {
            chars[i] = (chars[i] as u8 + 1) as char;
            break;
        }
    }
    chars.into_iter().collect()
}

pub fn solution() {
    let mut curr_password = include_str!("../data/day11.txt")
        .trim()
        .to_string();
    for i in 0..2 {
        while !is_valid_password(&curr_password) {
            curr_password = increment_password(&curr_password);
        }
        if i == 0 {
            println!("Santas new password is (part 1): {curr_password}");
            curr_password = increment_password(&curr_password);
        }
    }
    println!("Santas new password is (part 2): {curr_password}");
}
