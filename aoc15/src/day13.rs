use itertools::Itertools;
use std::collections::HashMap;

fn get_max_happiness(graph: &HashMap<String, HashMap<String, isize>>) -> isize {
    let mut max_happiness = isize::min_value();
    let possible_seating_arrangements = graph.keys().permutations(graph.len());
    for seating_arrangement in possible_seating_arrangements {
        let mut happiness = 0;
        for i in 0..seating_arrangement.len() {
            let left = seating_arrangement
                [(i + seating_arrangement.len() - 1) % seating_arrangement.len()];
            let right = seating_arrangement[(i + 1) % seating_arrangement.len()];
            happiness += graph[seating_arrangement[i]][left];
            happiness += graph[seating_arrangement[i]][right];
        }
        max_happiness = max_happiness.max(happiness);
    }
    max_happiness
}

pub fn solution() {
    let mut happiness = HashMap::new();
    let input = include_str!("../data/day13.txt");
    input.lines().for_each(|line| {
        let parts = line.split_whitespace().collect::<Vec<_>>();
        let name = parts[0].to_string();
        let other_name = parts[10].trim_end_matches('.').to_string();
        let happiness_score = if parts[2] == "gain" {
            parts[3].parse().unwrap()
        } else {
            -parts[3].parse::<isize>().unwrap()
        };
        happiness
            .entry(name)
            .or_insert_with(HashMap::new)
            .insert(other_name, happiness_score);
    });
    println!(
        "Total change of happiness: {}",
        get_max_happiness(&happiness)
    );

    for name in happiness.clone().keys() {
        happiness
            .entry("me".to_string())
            .or_insert_with(HashMap::new)
            .insert(name.clone(), 0);
        happiness.get_mut(name).unwrap().insert("me".to_string(), 0);
    }

    println!(
        "Total change of happiness with me included: {}",
        get_max_happiness(&happiness)
    );
}
