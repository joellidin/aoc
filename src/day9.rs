use std::collections::HashSet;

fn update_position(first: (isize, isize), next: (isize, isize)) -> (isize, isize) {
    let (x, y) = match (first.0 - next.0, first.1 - next.1) {
        (-1 | 0 | 1, -1 | 0 | 1) => (0, 0),
        (1 | 2, 1 | 2) => (1, 1),
        (2, 0) => (1, 0),
        (1 | 2, -1 | -2) => (1, -1),
        (0, 2) => (0, 1),
        (0, -2) => (0, -1),
        (-2 | -1, 1 | 2) => (-1, 1),
        (-2, 0) => (-1, 0),
        (-2 | -1, -2 | -1) => (-1, -1),
        _ => unreachable!(),
    };
    return (next.0 + x, next.1 + y);
}

fn update_positions(knots: &mut Vec<(isize, isize)>, tail_set: &mut Vec<HashSet<(isize, isize)>>) {
    for i in 0..knots.len() - 1 {
        knots[i + 1] = update_position(knots[i], knots[i + 1]);
        tail_set[i + 1].insert(knots[i + 1]);
    }
}

pub fn solution() {
    let input = std::fs::read_to_string("data/day9.txt").unwrap();
    let mut knots = vec![(0, 0); 10];
    let mut tails_set = vec![HashSet::new(); 10];
    for line in input.trim().lines() {
        let (direction, steps) = line
            .split_once(" ")
            .map(|(d, s)| (d, s.parse::<usize>().unwrap()))
            .unwrap();
        for _ in 0..steps {
            match direction {
                "R" => knots[0].0 += 1,
                "L" => knots[0].0 -= 1,
                "U" => knots[0].1 += 1,
                "D" => knots[0].1 -= 1,
                _ => unreachable!(),
            }
            update_positions(&mut knots, &mut tails_set);
        }
    }
    println!(
        "The first tail has seen {:?} unique tiles",
        tails_set[1].len()
    );
    println!(
        "The last tail has seen {:?} unique tiles",
        tails_set[9].len()
    );
}
