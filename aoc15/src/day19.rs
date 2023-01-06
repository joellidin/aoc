use std::collections::HashSet;

fn parse_input(input: &str) -> (Vec<(&str, &str)>, &str) {
    let (part, molecule) = input.split_once("\n\n").unwrap();
    let replacement_rules = part
        .lines()
        .map(|line| {
            let (from, to) = line.split_once(" => ").unwrap();
            (from, to)
        }).collect();
    (replacement_rules, molecule.trim_end())
}

fn get_all_possible_molecules(
    molecule: &str,
    replacement_rules: &[(&str, &str)],
) -> HashSet<String> {
    let mut molecules = HashSet::new();
    for (from, to) in replacement_rules {
        for (i, _) in molecule.match_indices(from) {
            let mut new_molecule = molecule.to_string();
            new_molecule.replace_range(i..i + from.len(), to);
            molecules.insert(new_molecule);
        }
    }
    molecules
}

fn backtrack(molecule: &str, replacement_rules: &[(&str, &str)], target: &str) -> Option<usize> {
    if molecule == target {
        return Some(0);
    }
    for (from, to) in replacement_rules {
        for (i, _) in molecule.match_indices(to) {
            let mut new_molecule = molecule.to_owned();
            new_molecule.replace_range(i..i + to.len(), from);
            if let Some(steps) = backtrack(&new_molecule, replacement_rules, target) {
                return Some(steps + 1);
            }
        }
    }
    None
}

pub fn solution() {
    let input = include_str!("../data/day19.txt");
    let (replacement_rules, molecule) = parse_input(input);

    println!(
        "We can create {} distinct molecules",
        get_all_possible_molecules(molecule, &replacement_rules).len()
    );
    println!(
        "The fewest numbers of steps we can take are {}",
        backtrack(molecule, &replacement_rules, "e").unwrap()
    );
}
