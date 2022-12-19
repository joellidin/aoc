use std::collections::HashMap;

#[derive(Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

enum Shape {
    Horizontal,
    Plus,
    InverseL,
    Vertical,
    Square,
}

impl Shape {
    const ORDER: &[Shape] = &[
        Shape::Horizontal,
        Shape::Plus,
        Shape::InverseL,
        Shape::Vertical,
        Shape::Square,
    ];
    fn get_rocks(&self, starting_point: &Point) -> impl Iterator<Item = Point> {
        let Point { x, y } = starting_point;
        match self {
            Shape::Horizontal => vec![
                Point { x: *x, y: *y },
                Point { x: x + 1, y: *y },
                Point { x: x + 2, y: *y },
                Point { x: x + 3, y: *y },
            ],
            Shape::Plus => vec![
                Point { x: *x, y: y + 1 },
                Point { x: x + 1, y: y + 1 },
                Point { x: x + 2, y: y + 1 },
                Point { x: x + 1, y: y + 2 },
                Point { x: x + 1, y: *y },
            ],
            Shape::InverseL => vec![
                Point { x: *x, y: *y },
                Point { x: x + 1, y: *y },
                Point { x: x + 2, y: *y },
                Point { x: x + 2, y: y + 1 },
                Point { x: x + 2, y: y + 2 },
            ],
            Shape::Vertical => vec![
                Point { x: *x, y: *y },
                Point { x: *x, y: y + 1 },
                Point { x: *x, y: y + 2 },
                Point { x: *x, y: y + 3 },
            ],
            Shape::Square => vec![
                Point { x: *x, y: *y },
                Point { x: *x, y: y + 1 },
                Point { x: x + 1, y: *y },
                Point { x: x + 1, y: y + 1 },
                Point { x: *x, y: *y },
            ],
        }
        .into_iter()
    }

    fn width(&self) -> usize {
        match self {
            Shape::Horizontal => 4,
            Shape::Plus => 3,
            Shape::InverseL => 3,
            Shape::Vertical => 1,
            Shape::Square => 2,
        }
    }

    fn height(&self) -> usize {
        match self {
            Shape::Horizontal => 1,
            Shape::Plus => 3,
            Shape::InverseL => 3,
            Shape::Vertical => 4,
            Shape::Square => 2,
        }
    }

    fn is_colliding(&self, point: &Point, cave: &[Vec<bool>]) -> bool {
        self.get_rocks(point).any(|Point { x, y }| cave[y][x])
    }

    fn move_from_stream(&self, point: &Point, direction: char, cave: &[Vec<bool>]) -> Point {
        let new_point = match direction {
            '<' => Point {
                x: point.x.saturating_sub(1),
                y: point.y,
            },
            '>' => Point {
                x: (point.x + 1).clamp(0, 7 - self.width()),
                y: point.y,
            },
            _ => unreachable!(),
        };
        if self.is_colliding(&new_point, cave) {
            *point
        } else {
            new_point
        }
    }

    fn place_rock(&self, point: &Point, cave: &mut [Vec<bool>]) {
        self.get_rocks(point).for_each(|Point { x, y }| {
            cave[y][x] = true;
        })
    }
}

fn drop_rock<'a>(
    rock: &mut impl Iterator<Item = (usize, &'a Shape)>,
    stream: &mut impl Iterator<Item = (usize, char)>,
    cave: &mut [Vec<bool>],
    drop_height: usize,
) -> (usize, usize, usize) {
    let mut point = Point {
        x: 2,
        y: drop_height + 3,
    };
    let (shape_idx, shape) = rock.next().unwrap();
    let stream_idx = loop {
        let (stream_idx, direction) = stream.next().unwrap();
        point = shape.move_from_stream(&point, direction, cave);
        if shape.is_colliding(
            &Point {
                x: point.x,
                y: point.y - 1,
            },
            cave,
        ) {
            shape.place_rock(&point, cave);
            break stream_idx;
        } else {
            point.y -= 1
        }
    };
    (
        shape_idx,
        stream_idx,
        drop_height.max(shape.height() + point.y),
    )
}

// This state struct I saw somewhere on reddit
#[derive(Hash, Clone, Copy, PartialEq, Eq, Debug)]
struct State([usize; 7], usize, usize);

impl State {
    fn ceiling_map(cave: &[Vec<bool>], height: usize) -> [usize; 7] {
        let mut result = [0; 7];
        for (idx, h) in (0..7)
            .map(|x| {
                let mut height = height;
                let mut count = 0;
                while !cave[height][x] {
                    height -= 1;
                    count += 1;
                }

                count
            })
            .enumerate()
        {
            result[idx] = h;
        }
        result
    }
}

fn simulate<'a>(
    n_rocks: usize,
    stream: &mut impl Iterator<Item = (usize, char)>,
    rocks: &mut impl Iterator<Item = (usize, &'a Shape)>,
) -> usize {
    let mut cave = vec![vec![false; 7]; (4 * n_rocks).min(250_000)];
    (0..7).for_each(|x| cave[0][x] = true);
    let mut height = 1;
    let mut state_map: HashMap<State, (usize, usize)> = HashMap::new();
    for i in 0..n_rocks {
        let (rock_idx, stream_idx, new_height) = drop_rock(rocks, stream, &mut cave, height);
        height = new_height;
        let ceiling = State::ceiling_map(&cave, height);
        let state = State(ceiling, rock_idx, stream_idx);
        if let Some((blocks_placed, prev_height)) = state_map.get(&state) {
            let n_blocks = i + 1 - blocks_placed;
            let d_height = height - prev_height;
            let repeats = (n_rocks - (i + 1)) / n_blocks;
            let n_left = (n_rocks - (i + 1)) % n_blocks;
            let mut tot_height = repeats * d_height;
            (0..n_left).for_each(|_| {
                let (_, _, height_now) = drop_rock(rocks, stream, &mut cave, height);
                height = height_now;
            });

            tot_height += height;
            return tot_height - 1;
        }
        state_map.insert(state, (i + 1, height));
    }
    height - 1
}

pub fn solution() {
    let pattern = std::fs::read_to_string("data/day17.txt").unwrap();
    let mut stream = pattern.trim().chars().enumerate().cycle();
    let mut rocks = Shape::ORDER.iter().enumerate().cycle();
    println!(
        "After 2022 rocks the tower will be {} units tall",
        simulate(2022, &mut stream, &mut rocks)
    );
    let mut stream = pattern.trim().chars().enumerate().cycle();
    let mut rocks = Shape::ORDER.iter().enumerate().cycle();
    println!(
        "After 1e12 rocks the tower will be {} units tall",
        simulate(1_000_000_000_000, &mut stream, &mut rocks,)
    );
}
