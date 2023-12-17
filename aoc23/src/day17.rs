use std::collections::{BinaryHeap, HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    position: Position,
    direction: Direction,
    steps_in_dir: usize,
    heat_loss: u32,
}

impl State {
    fn new(position: Position, direction: Direction, steps_in_dir: usize, heat_loss: u32) -> Self {
        State {
            position,
            direction,
            steps_in_dir,
            heat_loss,
        }
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .heat_loss
            .cmp(&self.heat_loss)
            .then_with(|| other.steps_in_dir.cmp(&self.steps_in_dir))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn dijkstra(map: &[Vec<u32>], min_steps: usize, max_steps: usize) -> u32 {
    let mut open_set = BinaryHeap::new();
    let mut visited = HashSet::new();
    let directions = [
        Direction::North,
        Direction::East,
        Direction::South,
        Direction::West,
    ];
    let goal = Position {
        y: map.len() - 1,
        x: map[0].len() - 1,
    };

    // Initialize starting directions
    for &dir in &directions {
        open_set.push(State::new(Position { x: 0, y: 0 }, dir, 0, 0));
    }

    while let Some(State {
        position,
        direction,
        steps_in_dir,
        heat_loss,
    }) = open_set.pop()
    {
        if position == goal {
            return heat_loss;
        }
        if !visited.insert((position, direction, steps_in_dir)) {
            continue;
        }

        for &new_direction in &directions {
            if new_direction == opposite_direction(direction)
                || (new_direction == direction && steps_in_dir == max_steps)
                || (new_direction != direction && steps_in_dir < min_steps)
            {
                continue;
            }

            let (new_x, new_y) = match new_direction {
                Direction::North if position.y > 0 => (position.x, position.y - 1),
                Direction::East if position.x < map[0].len() - 1 => (position.x + 1, position.y),
                Direction::South if position.y < map.len() - 1 => (position.x, position.y + 1),
                Direction::West if position.x > 0 => (position.x - 1, position.y),
                _ => continue,
            };

            let new_steps_in_dir = if new_direction == direction {
                steps_in_dir + 1
            } else {
                1
            };
            let new_position = Position { x: new_x, y: new_y };
            let new_heat_loss = heat_loss + map[new_y][new_x];

            open_set.push(State::new(
                new_position,
                new_direction,
                new_steps_in_dir,
                new_heat_loss,
            ));
        }
    }

    u32::MAX
}

fn opposite_direction(dir: Direction) -> Direction {
    match dir {
        Direction::North => Direction::South,
        Direction::East => Direction::West,
        Direction::South => Direction::North,
        Direction::West => Direction::East,
    }
}

pub fn generator(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|l| l.chars().map(|n| n.to_digit(10).unwrap()).collect())
        .collect()
}

pub fn part_1(input: &[Vec<u32>]) -> u32 {
    dijkstra(input, 0, 3)
}

pub fn part_2(input: &[Vec<u32>]) -> u32 {
    dijkstra(input, 4, 10)
}
