fn hash(s: &str) -> usize {
    s.chars().fold(0, |acc, c| ((acc + c as usize) * 17) % 256)
}

fn calculate_focus_power<T>(boxes: &[Vec<(T, usize)>]) -> usize {
    boxes
        .iter()
        .enumerate()
        .flat_map(|(i, v)| {
            v.iter()
                .enumerate()
                .map(move |(j, (_, n))| (i + 1) * (j + 1) * n)
        })
        .sum()
}
pub fn generator(input: &str) -> Vec<&str> {
    input.split(',').collect()
}

pub fn part_1(input: &[&str]) -> usize {
    input.iter().map(|s| hash(s)).sum()
}

pub fn part_2(input: &[&str]) -> usize {
    let mut boxes = vec![Vec::new(); 256];

    input.iter().for_each(|s| {
        let operation_pos = s.find(['-', '=']).expect("Needs to have either - or =");
        let label = &s[..operation_pos];
        let idx = hash(label);
        let box_vec = boxes.get_mut(idx).unwrap();
        match s.chars().nth(operation_pos) {
            Some('=') => {
                let num = s[operation_pos + 1..]
                    .parse::<usize>()
                    .expect("This part of the string has to be a number.");
                if let Some(replace_idx) = box_vec.iter().position(|(l, _)| &label == l) {
                    box_vec[replace_idx] = (label, num);
                } else {
                    box_vec.push((label, num));
                }
            }
            Some('-') => {
                if let Some(rm_idx) = box_vec.iter().position(|(l, _)| &label == l) {
                    box_vec.remove(rm_idx);
                }
            }
            _ => panic!("Should not be possible with another character at this position."),
        };
    });
    calculate_focus_power(&boxes)
}
