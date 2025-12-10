use aoc_utils::prelude::*;
use std::collections::HashMap;

pub fn generator(input: &str) -> Vec<Vec3<u32>> {
    input
        .lines()
        .map(|line| {
            let &[x, y, z, ..] = extract_integers::<u32>(line).as_slice() else {
                panic!("Could not parse integers")
            };
            (x, y, z).into()
        })
        .collect()
}

fn find(parent: &mut [usize], i: usize) -> usize {
    if parent[i] != i {
        parent[i] = find(parent, parent[i]);
    }
    parent[i]
}

fn calculate_pairs(junction_boxes: &[Vec3<u32>]) -> Vec<(u64, usize, usize)> {
    let mut pairs = Vec::new();
    for i in 0..junction_boxes.len() {
        let point_i = junction_boxes[i];
        for (j, point_j) in junction_boxes.iter().enumerate().skip(i + 1) {
            let dx = point_i.x.abs_diff(point_j.x) as u64;
            let dy = point_i.y.abs_diff(point_j.y) as u64;
            let dz = point_i.z.abs_diff(point_j.z) as u64;
            let distance = dx * dx + dy * dy + dz * dz;
            pairs.push((distance, i, j));
        }
    }
    pairs.sort_by_key(|&(d, ..)| d);
    pairs
}

pub fn part_1(input: &[Vec3<u32>]) -> u32 {
    let pairs = calculate_pairs(input);

    let mut parents: Vec<usize> = (0..input.len()).collect();

    let connections = if input.len() <= 20 { 10 } else { 1000 };
    for &(_, i, j) in pairs.iter().take(connections) {
        let pi = find(&mut parents, i);
        let pj = find(&mut parents, j);
        if pi != pj {
            parents[pi] = pj;
        }
    }

    let mut circuits = HashMap::new();
    for i in 0..input.len() {
        let root = find(&mut parents, i);
        *circuits.entry(root).or_default() += 1;
    }

    // Multiply 3 largest
    let mut sizes: Vec<u32> = circuits.values().copied().collect();
    sizes.sort_by(|a, b| b.cmp(a));
    sizes.iter().take(3).product()
}

pub fn part_2(input: &[Vec3<u32>]) -> u32 {
    let pairs = calculate_pairs(input);
    let mut parents: Vec<usize> = (0..input.len()).collect();

    let mut num_circuits = input.len();
    for &(_, i, j) in pairs.iter() {
        let pi = find(&mut parents, i);
        let pj = find(&mut parents, j);
        if pi != pj {
            parents[pi] = pj;
            num_circuits -= 1;
        }
        if num_circuits == 1 {
            return input[i].x * input[j].x;
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
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
        assert_eq!(result, 25272);
    }
}
