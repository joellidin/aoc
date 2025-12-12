use aoc_utils::prelude::*;

pub type Shape = Vec<(usize, usize)>;
pub type Region = (Vec<u32>, Vec<u32>); // (size [width, height], shapes_needed)

pub fn generator(input: &str) -> (Vec<Shape>, Vec<Region>) {
    let parts: Vec<_> = input.split("\n\n").collect();

    let shapes: Vec<Shape> = parts[..parts.len().saturating_sub(1)]
        .iter()
        .map(|shape| {
            shape
                .lines()
                .skip(1)
                .enumerate()
                .flat_map(|(y, line)| {
                    line.chars()
                        .enumerate()
                        .filter(|(_, c)| *c == '#')
                        .map(move |(x, _)| (y, x))
                })
                .collect()
        })
        .collect();

    let regions = parts
        .last()
        .expect("Invalid input")
        .lines()
        .map(|region| {
            let (size, shapes_needed) = region.split_once(':').expect("Invalid input");
            let size = extract_integers::<u32>(size);
            let shapes_needed = extract_integers::<u32>(shapes_needed);
            (size, shapes_needed)
        })
        .collect();
    (shapes, regions)
}

fn can_fit(
    width: u32,
    height: u32,
    shape_indices: &[usize],
    shape_orientations: &[Vec<Shape>],
) -> bool {
    let mut grid = vec![vec![false; width as usize]; height as usize];
    backtrack(&mut grid, shape_indices, shape_orientations, 0)
}

fn backtrack(
    grid: &mut Vec<Vec<bool>>,
    shape_indices: &[usize],
    shape_orientations: &[Vec<Shape>],
    index: usize,
) -> bool {
    if index == shape_indices.len() {
        return true;
    }

    // Early termination: count empty cells vs cells needed
    let empty_cells: usize = grid.iter().flatten().filter(|&&c| !c).count();
    let cells_needed: usize = shape_indices[index..]
        .iter()
        .map(|&idx| shape_orientations[idx][0].len()) // all orientations have same cell count
        .sum();

    if empty_cells < cells_needed {
        return false;
    }

    let shape_idx = shape_indices[index];
    let orientations = &shape_orientations[shape_idx];

    for oriented in orientations {
        for y in 0..grid.len() {
            for x in 0..grid[0].len() {
                if can_place(grid, oriented, x, y) {
                    place(grid, oriented, x, y, true);
                    if backtrack(grid, shape_indices, shape_orientations, index + 1) {
                        return true;
                    }
                    place(grid, oriented, x, y, false);
                }
            }
        }
    }
    false
}

fn all_orientations(shape: &Shape) -> Vec<Shape> {
    let mut orientations = Vec::new();
    let mut current = shape.clone();

    // 4 rotations
    for _ in 0..4 {
        let mut s = current.clone();
        s.sort(); // <- sort coordinates inside the shape
        orientations.push(s);
        current = flip_rows(&transpose(&current));
    }

    // flip and 4 more rotations
    current = flip_cols(shape);
    for _ in 0..4 {
        let mut s = current.clone();
        s.sort(); // <- sort coordinates here as well
        orientations.push(s);
        current = flip_rows(&transpose(&current));
    }

    orientations.sort();
    orientations.dedup();
    orientations
}

fn transpose(shape: &Shape) -> Shape {
    shape.iter().map(|(y, x)| (*x, *y)).collect()
}

fn flip_rows(shape: &Shape) -> Shape {
    let max_y = shape.iter().map(|(y, _)| *y).max().unwrap_or(0);
    shape.iter().map(|(y, x)| (max_y - y, *x)).collect()
}

fn flip_cols(shape: &Shape) -> Shape {
    let max_x = shape.iter().map(|(_, x)| *x).max().unwrap_or(0);
    shape.iter().map(|(y, x)| (*y, max_x - x)).collect()
}

fn can_place(grid: &[Vec<bool>], shape: &Shape, x: usize, y: usize) -> bool {
    shape.iter().all(|(sy, sx)| {
        let ny = y + sy;
        let nx = x + sx;
        ny < grid.len() && nx < grid[0].len() && !grid[ny][nx]
    })
}

fn place(grid: &mut [Vec<bool>], shape: &Shape, x: usize, y: usize, value: bool) {
    for (sy, sx) in shape {
        grid[y + sy][x + sx] = value;
    }
}

pub fn part_1(input: &(Vec<Shape>, Vec<Region>)) -> u32 {
    let (shapes, regions) = input;

    // Precompute orientations for each base shape once
    let shape_orientations: Vec<Vec<Shape>> = shapes.iter().map(all_orientations).collect();

    regions
        .iter()
        .filter(|(size, shape_counts)| {
            let shape_indices: Vec<usize> = shape_counts
                .iter()
                .enumerate()
                .flat_map(|(shape_idx, &count)| std::iter::repeat_n(shape_idx, count as usize))
                .collect();

            can_fit(size[0], size[1], &shape_indices, &shape_orientations)
        })
        .count() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2
"#;

    #[test]
    fn part_1_test() {
        let generator_output = generator(INPUT);
        let result = part_1(&generator_output);
        assert_eq!(result, 2);
    }
}
