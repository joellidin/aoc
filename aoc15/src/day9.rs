use std::collections::HashMap;

fn get_distances(graph: &HashMap<&str, HashMap<&str, usize>>) -> Vec<usize> {
    let mut queue = Vec::new();
    let mut visited = HashMap::new();
    for node in graph.keys() {
        visited.insert(node, false);
    }
    for node in graph.keys() {
        queue.push((0, node, visited.clone()));
    }
    let mut distances = Vec::new();
    while let Some((distance, node, mut visited)) = queue.pop() {
        visited.insert(node, true);
        if visited.values().all(|&v| v) {
            distances.push(distance);
            continue;
        }
        for (neighbor, weight) in graph.get(node).unwrap() {
            if *visited.get(neighbor).unwrap() {
                continue;
            }
            queue.push((distance + weight, neighbor, visited.clone()));
        }
    }
    distances
}

pub fn solution() {
    let mut routes = HashMap::new();
    let input = include_str!("../data/day9.txt");
    input.lines().for_each(|line| {
        let mut parts = line.split(" = ");
        let (from, to) = parts.next().unwrap().split_once(" to ").unwrap();
        let distance = parts.next().unwrap().parse::<usize>().unwrap();
        routes
            .entry(from)
            .or_insert_with(HashMap::new)
            .insert(to, distance);
        routes
            .entry(to)
            .or_insert_with(HashMap::new)
            .insert(from, distance);
    });
    println!(
        "Min distance: {}",
        get_distances(&routes).iter().min().unwrap()
    );
    println!(
        "Max distance: {}",
        get_distances(&routes).iter().max().unwrap()
    );
}
