use itertools::Itertools;

fn get_lowest_qe(nums: &Vec<u32>, parts: u32) -> Option<u64> {
    let part_sum = nums.iter().sum::<u32>() / parts;
    for i in 1..nums.len() {
        for combination in nums.iter().combinations(i) {
            if combination.iter().map(|x| **x).sum::<u32>() == part_sum {
                return Some(combination.iter().map(|x| **x as u64).product());
            }
        }
    }
    None
}
pub fn solution() {
    let input = std::fs::read_to_string("data/day24.txt")
        .unwrap()
        .lines()
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<_>>();
    println!(
        "The quantum entanglement for the ideal configuration with 3 parts is {}",
        get_lowest_qe(&input, 3).unwrap()
    );
    println!(
        "The quantum entanglement for the ideal configuration with 4 parts is {}",
        get_lowest_qe(&input, 4).unwrap()
    );
}
