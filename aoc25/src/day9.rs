use aoc_utils::prelude::*;

pub fn generator(input: &str) -> Vec<Vec2<u64>> {
    input
        .lines()
        .map(|line| {
            let &[x, y, ..] = extract_integers::<u64>(line).as_slice() else {
                panic!("Could not parse integers")
            };
            (x, y).into()
        })
        .collect()
}

pub fn part_1(input: &[Vec2<u64>]) -> u64 {
    let mut max_area = 0;
    for i in 0..input.len() {
        let point_i = input[i];
        for point_j in input.iter().skip(i + 1) {
            let area = (point_i.x.abs_diff(point_j.x) + 1) * (point_i.y.abs_diff(point_j.y) + 1);
            max_area = max_area.max(area);
        }
    }
    max_area
}

pub fn part_2(input: &[Vec2<u64>]) -> u64 {
    let mut vertical_edges: Vec<(u64, u64, u64)> = Vec::new();
    let mut horizontal_edges: Vec<(u64, u64, u64)> = Vec::new();
    for i in 0..input.len() {
        let p1 = input[i];
        let p2 = input[(i + 1) % input.len()];

        if p1.x == p2.x {
            // Vertical edge: same x, different y
            let (min_y, max_y) = (p1.y.min(p2.y), p1.y.max(p2.y));
            vertical_edges.push((p1.x, min_y, max_y));
        } else {
            // Horizontal edge: same y, different x
            let (min_x, max_x) = (p1.x.min(p2.x), p1.x.max(p2.x));
            horizontal_edges.push((p1.y, min_x, max_x));
        }
    }

    // Ray casting. Odd numbers of crossings -> Inside
    let is_inside = |x: u64, y: u64| -> bool {
        let mut crossings = 0;
        for &(edge_x, min_y, max_y) in &vertical_edges {
            if edge_x > x && min_y <= y && y < max_y {
                crossings += 1;
            }
        }
        crossings % 2 == 1
    };

    let is_on_boundary = |x: u64, y: u64| -> bool {
        // Check vertical edges: x matches, y in [min_y, max_y]
        let on_vertical = vertical_edges
            .iter()
            .any(|&(edge_x, min_y, max_y)| edge_x == x && min_y <= y && y <= max_y);

        // Check horizontal edges: y matches, x in [min_x, max_x]
        let on_horizontal = horizontal_edges
            .iter()
            .any(|&(edge_y, min_x, max_x)| edge_y == y && min_x <= x && x <= max_x);

        on_vertical || on_horizontal
    };

    let is_valid_rect = |x1: u64, y1: u64, x2: u64, y2: u64| -> bool {
        let (x1, x2) = (x1.min(x2), x1.max(x2));
        let (y1, y2) = (y1.min(y2), y1.max(y2));

        // 1. Check all 4 corners are valid
        if !(is_on_boundary(x1, y2) || is_inside(x1, y2)) {
            return false;
        }
        if !(is_on_boundary(x2, y1) || is_inside(x2, y1)) {
            return false;
        }

        // 2. Check no vertical edge passes through the interior
        for &(edge_x, ey1, ey2) in &vertical_edges {
            if x1 < edge_x && edge_x < x2 && ey1 < y2 && ey2 > y1 {
                return false;
            }
        }

        // 3. Check no HORIZONTAL edge passes through the interior
        for &(edge_y, ex1, ex2) in &horizontal_edges {
            if y1 < edge_y && edge_y < y2 && ex1 < x2 && ex2 > x1 {
                return false;
            }
        }

        true
    };

    let mut max_area = 0;
    input.iter().enumerate().for_each(|(i, point_i)| {
        for point_j in input.iter().skip(i + 1) {
            if is_valid_rect(point_i.x, point_i.y, point_j.x, point_j.y) {
                let area =
                    (point_i.x.abs_diff(point_j.x) + 1) * (point_i.y.abs_diff(point_j.y) + 1);
                max_area = max_area.max(area);
            }
        }
    });
    max_area
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
"#;

    #[test]
    fn part_1_test() {
        let generator_output = generator(INPUT);
        let result = part_1(&generator_output);
        assert_eq!(result, 50);
    }

    #[test]
    fn part_2_test() {
        let generator_output = generator(INPUT);
        let result = part_2(&generator_output);
        assert_eq!(result, 24);
    }
}
