use std::collections::VecDeque;

// https://www.geeksforgeeks.org/dijkstras-shortest-path-algorithm-using-priority_queue-stl/
fn dijkstra(end: &(usize, usize), heightmap: &[Vec<u8>], distances: &mut [Vec<u32>]) {
    let mut search_queue = VecDeque::with_capacity(heightmap.len() * heightmap[0].len());
    search_queue.push_back(*end);
    while !search_queue.is_empty() {
        let (curr_i, curr_j) = search_queue.pop_front().unwrap();
        let new_dist = distances[curr_i][curr_j] + 1;

        for (di, dj) in &[(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let (i, j) = (curr_i as isize + di, curr_j as isize + dj);
            // New pos inside grid
            if (i >= 0 && i < heightmap.len() as isize && j >= 0 && j < heightmap[0].len() as isize)
                // Is valid step
                && (heightmap[curr_i][curr_j] <= heightmap[i as usize][j as usize] + 1)
                // New distance is smaller
                && (distances[i as usize][j as usize] > new_dist)
            {
                distances[i as usize][j as usize] = new_dist;
                search_queue.push_back((i as usize, j as usize))
            }
        }
    }
}

type Heightmap = Vec<Vec<u8>>;
type Distances = Vec<Vec<u32>>;
type Pos = (usize, usize);

pub fn generator(input: &str) -> (Heightmap, Distances, Pos) {
    let (mut start, mut end) = ((0, 0), (0, 0));
    let heightmap = input
        .trim()
        .split('\n')
        .enumerate()
        .map(|(i, line)| {
            line.as_bytes()
                .iter()
                .enumerate()
                .map(|(j, c)| {
                    if c == &b'S' {
                        start = (i, j);
                        b'a'
                    } else if c == &b'E' {
                        end = (i, j);
                        b'z'
                    } else {
                        *c
                    }
                })
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<_>>();
    let mut distances = heightmap
        .iter()
        .map(|r| r.iter().map(|_| u32::MAX).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    distances[end.0][end.1] = 0;
    dijkstra(&end, &heightmap, &mut distances);
    (heightmap, distances, start)
}

pub fn part_1((_, distances, start): &(Heightmap, Distances, Pos)) -> u32 {
    distances[start.0][start.1]
}

pub fn part_2((heightmap, distances, _): &(Heightmap, Distances, Pos)) -> u32 {
    let pot_start = heightmap
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, c)| c == &&b'a')
                .map(move |(j, _)| (i, j))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    pot_start
        .iter()
        .map(|start| distances[start.0][start.1])
        .min()
        .unwrap()
}
