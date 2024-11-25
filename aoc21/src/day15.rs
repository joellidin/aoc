use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};

#[derive(Clone)]
pub struct Map {
    grid: Vec<Vec<u8>>,
}

impl Map {
    fn find_lowest_risk_path(&self) -> Option<u32> {
        let mut heap = BinaryHeap::new();
        let mut distances = HashMap::new();
        let start = (0, 0);
        let end = (self.grid[0].len() - 1, self.grid.len() - 1);
        distances.insert(start, 0);
        heap.push(Reverse((0, start)));

        while let Some(Reverse((risk_level, point))) = heap.pop() {
            if point == end {
                return Some(risk_level);
            }
            for neighbor in self.find_neighbors(point) {
                let next_risk = risk_level + self.grid[neighbor.1][neighbor.0] as u32;
                if next_risk < *distances.get(&neighbor).unwrap_or(&u32::MAX) {
                    distances.insert(neighbor, next_risk);
                    heap.push(Reverse((next_risk, neighbor)));
                }
            }
        }
        None
    }

    fn find_neighbors(&self, point: (usize, usize)) -> Vec<(usize, usize)> {
        let mut neighbors = Vec::new();
        let (x, y) = point;
        let dirs = [(-1, 0), (1, 0), (0, -1), (0, 1)];

        for (dx, dy) in dirs.iter() {
            let new_x = x as isize + dx;
            let new_y = y as isize + dy;
            if new_x >= 0
                && new_x < self.grid[0].len() as isize
                && new_y >= 0
                && new_y < self.grid.len() as isize
            {
                neighbors.push((new_x as usize, new_y as usize));
            }
        }
        neighbors
    }

    pub fn extend_with_risk(&mut self, factor: usize) {
        let original_grid = self.grid.clone();
        let original_height = original_grid.len();
        let original_width = if original_height > 0 {
            original_grid[0].len()
        } else {
            0
        };

        if original_height == 0 || original_width == 0 {
            return;
        }

        // Create a new grid for the extended map
        let mut extended_grid = vec![vec![0; original_width * factor]; original_height * factor];

        // Populate the extended grid
        for tile_row in 0..factor {
            for tile_col in 0..factor {
                let risk_increase = (tile_row + tile_col) as u8; // Calculate risk increase for this tile
                for row in 0..original_height {
                    for col in 0..original_width {
                        let original_risk = original_grid[row][col];
                        let new_risk = (original_risk + risk_increase - 1) % 9 + 1; // Wrap risk to 1-9
                        extended_grid[tile_row * original_height + row]
                            [tile_col * original_width + col] = new_risk;
                    }
                }
            }
        }

        self.grid = extended_grid; // Update the map's grid
    }
}

pub fn generator(input: &str) -> Map {
    let grid = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).expect("Map must be numbers") as u8)
                .collect()
        })
        .collect();
    Map { grid }
}

pub fn part_1(input: &Map) -> u32 {
    input.find_lowest_risk_path().expect("Must find the end")
}

pub fn part_2(input: &Map) -> u32 {
    let mut map = input.clone();
    map.extend_with_risk(5);
    map.find_lowest_risk_path().expect("Must find end")
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581
"#;

    #[test]
    fn part_1_test() {
        let generator_output = generator(INPUT);
        let result = part_1(&generator_output);
        assert_eq!(result, 40);
    }

    #[test]
    fn part_2_test() {
        let generator_output = generator(INPUT);
        let result = part_2(&generator_output);
        assert_eq!(result, 315);
    }
}
