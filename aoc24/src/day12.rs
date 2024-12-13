use std::collections::HashSet;

pub fn generator(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn traverse(map: &[Vec<char>], node: (usize, usize)) -> (u32, u32, HashSet<(usize, usize)>) {
    let curr = map[node.0][node.1];
    let mut stack = vec![node];
    let mut seen = HashSet::from([node]);
    let mut area = 0;
    let mut perimiter = 0;
    while let Some((curr_i, curr_j)) = stack.pop() {
        area += 1;
        for (di, dj) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let new_i = (curr_i as isize + di) as usize;
            let new_j = (curr_j as isize + dj) as usize;

            if new_i < map.len() && new_j < map[0].len() && map[new_i][new_j] == curr {
                if !seen.contains(&(new_i, new_j)) {
                    stack.push((new_i, new_j));
                    seen.insert((new_i, new_j));
                }
            } else {
                perimiter += 1;
            }
        }
    }
    (area, perimiter, seen)
}

fn count_sides(seen: &HashSet<(usize, usize)>) -> u32 {
    let mut sides_seen = HashSet::new();
    let mut sides = 0;
    let moves = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    for &(y, x) in seen {
        for &(dy, dx) in &moves {
            let new_y = y as isize + dy;
            let new_x = x as isize + dx;

            // Check if the neighbor exists in the grid
            if seen.contains(&(new_y as usize, new_x as usize)) {
                continue;
            }

            // Find canonical side
            let (mut cy, mut cx) = (y, x);
            while seen.contains(&((cy as isize + dx) as usize, (cx as isize + dy) as usize))
                && !seen.contains(&((cy as isize + dy) as usize, (cx as isize + dx) as usize))
            {
                cy = (cy as isize + dx) as usize;
                cx = (cx as isize + dy) as usize;
            }

            if sides_seen.insert((cy, cx, dy, dx)) {
                sides += 1;
            }
        }
    }
    sides
}

pub fn part_1(input: &[Vec<char>]) -> u32 {
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let (height, width) = (input.len(), input[0].len());
    let mut score = 0;
    for i in 0..height {
        for j in 0..width {
            if visited.contains(&(i, j)) {
                continue;
            }
            let (area, perimiter, seen) = traverse(input, (i, j));
            score += area * perimiter;
            visited = visited.union(&seen).copied().collect();
        }
    }
    score
}

pub fn part_2(input: &[Vec<char>]) -> u32 {
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let (height, width) = (input.len(), input[0].len());
    let mut score = 0;
    for i in 0..height {
        for j in 0..width {
            if visited.contains(&(i, j)) {
                continue;
            }
            let (area, _, seen) = traverse(input, (i, j));
            let n_sides = count_sides(&seen);
            score += area * n_sides;
            visited = visited.union(&seen).copied().collect();
        }
    }
    score
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
"#;

    #[test]
    fn part_1_test() {
        let generator_output = generator(INPUT);
        let result = part_1(&generator_output);
        assert_eq!(result, 1930);
    }

    #[test]
    fn part_2_test() {
        let generator_output = generator(INPUT);
        let result = part_2(&generator_output);
        assert_eq!(result, 1206);
    }
}
