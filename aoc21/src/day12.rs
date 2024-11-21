use std::collections::HashMap;

pub struct CaveSystem {
    graph: Vec<Vec<usize>>,   // Graph represented as adjacency lists
    start_idx: usize,         // Index of the start cave
    end_idx: usize,           // Index of the end cave
    is_small_cave: Vec<bool>, // Flags indicating if a cave is small
    small_cave_indices: HashMap<usize, usize>, // Mapping of small cave indices to bit positions
}

pub fn generator(input: &str) -> CaveSystem {
    let mut cave_indices = HashMap::new();
    let mut caves = Vec::new();
    let mut is_small_cave = Vec::new();

    // Assign indices to caves and determine if they're small
    for line in input.lines() {
        let (start, end) = line.split_once('-').expect("Invalid input.");
        for cave in [start, end] {
            if !cave_indices.contains_key(cave) {
                let index = caves.len();
                cave_indices.insert(cave, index);
                caves.push(cave);
                is_small_cave.push(cave.chars().all(char::is_lowercase));
            }
        }
    }

    // Build the graph as an adjacency list
    let mut graph = vec![Vec::new(); caves.len()];
    for line in input.lines() {
        let (start, end) = line.split_once('-').expect("Invalid input.");
        let start_idx = cave_indices[start];
        let end_idx = cave_indices[end];
        graph[start_idx].push(end_idx);
        graph[end_idx].push(start_idx);
    }

    let start_idx = cave_indices["start"];
    let end_idx = cave_indices["end"];

    // Map small caves (excluding "start" and "end") to bit positions
    let mut small_cave_indices = HashMap::new();
    let mut bit_position = 0;
    for (idx, &small) in is_small_cave.iter().enumerate() {
        let cave_name = caves[idx];
        if small && cave_name != "start" && cave_name != "end" {
            small_cave_indices.insert(idx, bit_position);
            bit_position += 1;
        }
    }

    CaveSystem {
        graph,
        start_idx,
        end_idx,
        is_small_cave,
        small_cave_indices,
    }
}

fn count_paths(
    cave_system: &CaveSystem,
    current: usize,
    end: usize,
    visited: u64,
    visited_twice: bool,
    memo: &mut HashMap<(usize, u64, bool), u32>,
) -> u32 {
    if current == end {
        return 1;
    }

    let key = (current, visited, visited_twice);

    // Return cached result if available
    if let Some(&cached) = memo.get(&key) {
        return cached;
    }

    let mut total_paths = 0;

    for &neighbor in &cave_system.graph[current] {
        if neighbor == cave_system.start_idx {
            continue; // Never revisit the "start" cave
        }

        if cave_system.is_small_cave[neighbor] {
            if neighbor == end {
                // Proceed to "end" cave
                total_paths +=
                    count_paths(cave_system, neighbor, end, visited, visited_twice, memo);
            } else {
                let idx_in_small = match cave_system.small_cave_indices.get(&neighbor) {
                    Some(&idx) => idx,
                    None => continue,
                };
                let mask = 1 << idx_in_small;
                let has_visited = (visited & mask) != 0;

                if !has_visited {
                    // First visit to this small cave
                    total_paths += count_paths(
                        cave_system,
                        neighbor,
                        end,
                        visited | mask,
                        visited_twice,
                        memo,
                    );
                } else if !visited_twice {
                    // Visiting a small cave for the second time
                    total_paths += count_paths(cave_system, neighbor, end, visited, true, memo);
                }
                // If the small cave has been visited and we've already visited another small cave twice, skip
            }
        } else {
            // Large cave, can visit any number of times
            total_paths += count_paths(cave_system, neighbor, end, visited, visited_twice, memo);
        }
    }

    // Cache the result
    memo.insert(key, total_paths);

    total_paths
}

pub fn part_1(cave_system: &CaveSystem) -> u32 {
    let visited = 0u64; // No small caves visited initially
    let visited_twice = true; // Prevent any small cave from being visited twice

    let mut memo = HashMap::new();

    count_paths(
        cave_system,
        cave_system.start_idx,
        cave_system.end_idx,
        visited,
        visited_twice,
        &mut memo,
    )
}

pub fn part_2(cave_system: &CaveSystem) -> u32 {
    let visited = 0u64; // No small caves visited initially
    let visited_twice = false; // Allow one small cave to be visited twice

    let mut memo = HashMap::new();

    count_paths(
        cave_system,
        cave_system.start_idx,
        cave_system.end_idx,
        visited,
        visited_twice,
        &mut memo,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"start-A
start-b
A-c
A-b
b-d
A-end
b-end
"#;

    #[test]
    fn part_1_test() {
        let generator_output = generator(INPUT);
        let result = part_1(&generator_output);
        assert_eq!(result, 10);
    }

    #[test]
    fn part_2_test() {
        let generator_output = generator(INPUT);
        let result = part_2(&generator_output);
        assert_eq!(result, 36);
    }
}
