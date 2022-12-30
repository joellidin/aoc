use std::cmp::Ordering;

fn get_combinations(tot_eggnod: u16, containers: &[u16]) -> Vec<Vec<u16>> {
    let mut combinations = Vec::new();
    for i in 0..containers.len() {
        let container = containers[i];
        match tot_eggnod.cmp(&container) {
            Ordering::Equal => combinations.append(&mut vec![vec![container]]),
            Ordering::Greater => {
                let new_v = get_combinations(tot_eggnod - container, &containers[i + 1..]);
                for mut v in new_v {
                    v.push(container);
                    combinations.push(v);
                }
            }
            Ordering::Less => {}
        }
    }
    combinations
}

pub fn solution() {
    let containers = std::fs::read_to_string("data/day17.txt")
        .unwrap()
        .trim()
        .lines()
        .map(|line| line.parse().unwrap())
        .collect::<Vec<_>>();
    let combinations = get_combinations(150, &containers);
    println!("There are {} different combinations", combinations.len());
    let min_containers = combinations.iter().map(|v| v.len()).min().unwrap();
    println!(
        "We can fill the minimum amount of containers in {} ways",
        combinations
            .iter()
            .filter(|v| v.len() == min_containers)
            .count()
    );
}
