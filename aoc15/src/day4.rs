use md5::{Digest, Md5};

pub fn solution() {
    let input = include_str!("../data/day4.txt");
    let mut part1_done = false;
    let mut hasher = Md5::new();
    for i in 0.. {
        hasher.update(format!("{}{i}", input.trim()));
        let hash = hasher.finalize_reset();
        if !part1_done && hash[..2] == [0, 0] && &hash[2] & 0xf0 == 0 {
            part1_done = true;
            println!("Part 1: {i}");
        }
        if hash[..3] == [0, 0, 0] {
            println!("Part 2: {i}");
            break;
        }
    }
}
