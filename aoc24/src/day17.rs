use aoc_utils::prelude::*;

pub fn generator(input: &str) -> (Vec<u64>, Vec<u64>) {
    let (registers, program) = input.split_once("\n\n").unwrap();
    let program = extract_integers::<u64>(program);
    (
        registers
            .lines()
            .map(|line| {
                let &[x, ..] = extract_integers::<u64>(line).as_slice() else {
                    panic!("Could not parse integers")
                };
                x
            })
            .collect(),
        program,
    )
}

fn run_program(register: &[u64], program: &[u64]) -> Vec<u64> {
    let mut i = 0;
    let mut outputs = Vec::new();
    let mut a_value = register[0];
    let mut b_value = register[1];
    let mut c_value = register[2];
    while i < program.len() - 1 {
        let opcode = program[i];
        let literal_operand = program[i + 1];
        let combo_operand = match literal_operand {
            0..=3 => literal_operand,
            4 => a_value,
            5 => b_value,
            6 => c_value,
            _ => unreachable!(),
        };
        match opcode {
            0 => {
                a_value /= 1u64 << combo_operand;
            }
            1 => {
                b_value ^= literal_operand;
            }
            2 => {
                b_value = combo_operand % 8;
            }
            3 => {
                if a_value != 0 {
                    i = literal_operand as usize;
                    continue;
                }
            }
            4 => {
                b_value ^= c_value;
            }
            5 => {
                outputs.push(combo_operand % 8);
            }
            6 => {
                b_value = a_value / (1u64 << combo_operand);
            }
            7 => {
                c_value = a_value / (1u64 << combo_operand);
            }
            _ => unreachable!(),
        };
        i += 2;
    }
    outputs
}

fn find_inital_value(register: &[u64], program: &[u64]) -> u64 {
    let mut valid = vec![0];
    for n in (0..program.len()).rev() {
        let old_valid = valid.clone();
        valid = Vec::new();
        for num in old_valid {
            for offset in 0..8 {
                let new_num = 8 * num + offset;
                let output = run_program(&[new_num, register[1], register[2]], program);
                if output.as_slice() == &program[n..] {
                    valid.push(new_num)
                }
            }
        }
    }
    valid.into_iter().min().unwrap()
}

pub fn part_1(input: &(Vec<u64>, Vec<u64>)) -> String {
    let (register, program) = input;
    let outputs = run_program(register, program);
    outputs
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

pub fn part_2(input: &(Vec<u64>, Vec<u64>)) -> u64 {
    let (register, program) = input;
    find_inital_value(register, program)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_1: &str = r#"Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0
"#;

    const INPUT_2: &str = r#"Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0
"#;

    #[test]
    fn part_1_test() {
        let generator_output = generator(INPUT_1);
        let result = part_1(&generator_output);
        assert_eq!(result, "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn part_2_test() {
        let generator_output = generator(INPUT_2);
        let result = part_2(&generator_output);
        assert_eq!(result, 117440);
    }
}
