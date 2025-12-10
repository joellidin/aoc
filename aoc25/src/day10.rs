use std::collections::{HashSet, VecDeque};
use z3::ast::Int;
use z3::{Optimize, SatResult};

use aoc_utils::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LightState {
    On,
    Off,
}

impl std::ops::Not for LightState {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            LightState::On => LightState::Off,
            LightState::Off => LightState::On,
        }
    }
}

pub type MachineData = (Vec<Vec<LightState>>, Vec<Vec<Vec<u32>>>, Vec<Vec<u32>>);

pub fn generator(input: &str) -> MachineData {
    let mut light_diagrams = Vec::new();
    let mut wirings = Vec::new();
    let mut requirements = Vec::new();
    input.lines().for_each(|line| {
        let diagram_end_idx = line.find(']').expect("Invalid input");
        let requirement_start_idx = line.find('{').expect("Invalid input");
        let diagram = line[1..diagram_end_idx]
            .chars()
            .map(|c| match c {
                '.' => LightState::Off,
                '#' => LightState::On,
                _ => panic!("Invalid input"),
            })
            .collect::<Vec<LightState>>();

        let wiring = line[diagram_end_idx + 1..requirement_start_idx]
            .split_whitespace()
            .map(extract_integers::<u32>)
            .collect::<Vec<Vec<u32>>>();

        let requirement = extract_integers::<u32>(&line[requirement_start_idx..]);
        light_diagrams.push(diagram);
        wirings.push(wiring);
        requirements.push(requirement);
    });

    (light_diagrams, wirings, requirements)
}

pub fn bts(start: &[LightState], target: &[LightState], buttons: &[Vec<u32>]) -> u32 {
    if start == target {
        return 0;
    }
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();

    visited.insert(start.to_vec());
    queue.push_back((start.to_vec(), 0));
    while let Some((state, n_presses)) = queue.pop_front() {
        for button in buttons {
            let mut new_state = state.to_vec();
            for &idx in button {
                new_state[idx as usize] = !new_state[idx as usize];
            }

            if new_state == target {
                return n_presses + 1;
            }

            if visited.insert(new_state.clone()) {
                queue.push_back((new_state, n_presses + 1));
            }
        }
    }
    u32::MAX
}

fn solve_z3(target: &[u32], buttons: &[Vec<u32>]) -> u32 {
    let opt = Optimize::new();

    let n_buttons = buttons.len();

    // x_i = number of times button i is pressed
    let presses: Vec<Int> = (0..n_buttons)
        .map(|i| Int::new_const(format!("x_{i}")))
        .collect();

    // x_i >= 0
    for p in &presses {
        opt.assert(&p.ge(0_i64));
    }

    // For each counter j: sum of affecting buttons = target[j]
    for (j, &t) in target.iter().enumerate() {
        let terms = buttons
            .iter()
            .enumerate()
            .filter(|(_, btn)| btn.contains(&(j as u32)))
            .map(|(i, _)| &presses[i])
            .collect::<Vec<&Int>>();

        let sum = Int::add(&terms);
        let target_val: Int = (t as i64).into();
        opt.assert(&sum.eq(&target_val));
    }

    // Minimize total number of presses
    let total = Int::add(&presses.iter().collect::<Vec<_>>());
    opt.minimize(&total);

    match opt.check(&[]) {
        SatResult::Sat => {
            let model = opt.get_model().unwrap();
            presses
                .iter()
                .map(|p| model.eval(p, true).unwrap().as_i64().unwrap() as u32)
                .sum()
        }
        _ => u32::MAX,
    }
}

pub fn part_1(input: &MachineData) -> u32 {
    let (light_diagrams, wirings, _) = input;
    (0..light_diagrams.len())
        .map(|i| {
            let start = vec![LightState::Off; light_diagrams[i].len()];
            bts(&start, &light_diagrams[i], &wirings[i])
        })
        .sum()
}

pub fn part_2(input: &MachineData) -> u32 {
    let (_, wirings, requirements) = input;
    (0..requirements.len())
        .map(|i| solve_z3(&requirements[i], &wirings[i]))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
"#;

    #[test]
    fn part_1_test() {
        let generator_output = generator(INPUT);
        let result = part_1(&generator_output);
        assert_eq!(result, 7);
    }

    #[test]
    fn part_2_test() {
        let generator_output = generator(INPUT);
        let result = part_2(&generator_output);
        assert_eq!(result, 33);
    }
}
