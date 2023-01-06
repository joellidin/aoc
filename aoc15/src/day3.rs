use std::collections::HashSet;

pub fn solution() {
    let input = include_str!("../data/day3.txt");
    let mut seen_houses = HashSet::new();
    input.chars().fold((0, 0), |(x, y), c| {
        seen_houses.insert((x, y));
        match c {
            '^' => (x, y + 1),
            'v' => (x, y - 1),
            '>' => (x + 1, y),
            '<' => (x - 1, y),
            _ => (x, y),
        }
    });
    println!("Santa visits {} houses", seen_houses.len());

    let mut seen_houses = HashSet::new();
    input
        .chars()
        .enumerate()
        .fold(((0, 0), (0, 0)), |((xs, ys), (xr, yr)), (i, c)| {
            if i % 2 == 0 {
                seen_houses.insert((xs, ys));
                match c {
                    '^' => ((xs, ys + 1), (xr, yr)),
                    'v' => ((xs, ys - 1), (xr, yr)),
                    '>' => ((xs + 1, ys), (xr, yr)),
                    '<' => ((xs - 1, ys), (xr, yr)),
                    _ => ((xs, ys), (xr, yr)),
                }
            } else {
                seen_houses.insert((xr, yr));
                match c {
                    '^' => ((xs, ys), (xr, yr + 1)),
                    'v' => ((xs, ys), (xr, yr - 1)),
                    '>' => ((xs, ys), (xr + 1, yr)),
                    '<' => ((xs, ys), (xr - 1, yr)),
                    _ => ((xs, ys), (xr, yr)),
                }
            }
        });
    println!("Santa and Robo-Santa visits {} houses", seen_houses.len());
}
