use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Hlf(Reg),
    Tpl(Reg),
    Inc(Reg),
    Jmp(i32),
    Jie(Reg, i32),
    Jio(Reg, i32),
}

#[derive(Debug, Clone, Copy)]
enum Reg {
    A,
    B,
}

impl FromStr for Instruction {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (inst, rest) = s.split_once(' ').unwrap();
        let reg = if rest.contains('a') { Reg::A } else { Reg::B };

        match inst {
            "hlf" => Ok(Instruction::Hlf(reg)),
            "tpl" => Ok(Instruction::Tpl(reg)),
            "inc" => Ok(Instruction::Inc(reg)),
            "jmp" => Ok(Instruction::Jmp(rest.trim().parse().unwrap())),
            "jie" => {
                let (_, offset) = rest.split_once(", ").unwrap();
                Ok(Instruction::Jie(reg, offset.trim().parse().unwrap()))
            }
            "jio" => {
                let (_, offset) = rest.split_once(", ").unwrap();
                Ok(Instruction::Jio(reg, offset.trim().parse().unwrap()))
            }
            _ => unreachable!(),
        }
    }
}

fn run(instructions: &[Instruction], start: u32) -> u32 {
    let mut a = start;
    let mut b = 0;
    let mut idx = 0i32;
    while (idx as usize) < instructions.len() {
        match instructions[idx as usize] {
            Instruction::Hlf(reg) => match reg {
                Reg::A => a /= 2,
                Reg::B => b /= 2,
            },
            Instruction::Tpl(reg) => match reg {
                Reg::A => a *= 3,
                Reg::B => b *= 3,
            },
            Instruction::Inc(reg) => match reg {
                Reg::A => a += 1,
                Reg::B => b += 1,
            },
            Instruction::Jmp(i) => {
                idx += i;
                continue;
            }
            Instruction::Jie(reg, i) => match reg {
                Reg::A => {
                    if a % 2 == 0 {
                        idx += i;
                        continue;
                    };
                }
                Reg::B => {
                    if b % 2 == 0 {
                        idx += i;
                        continue;
                    };
                }
            },
            Instruction::Jio(reg, i) => match reg {
                Reg::A => {
                    if a == 1 {
                        idx += i;
                        continue;
                    };
                }
                Reg::B => {
                    if b % 2 == 1 {
                        idx += i;
                        continue;
                    };
                }
            },
        }
        idx += 1;
    }
    b
}

pub fn solution() {
    let instructions = std::fs::read_to_string("data/day23.txt")
        .unwrap()
        .lines()
        .map(|l| l.parse().unwrap())
        .collect::<Vec<Instruction>>();
    println!(
        "After the instructions, register b has the value {}",
        run(&instructions, 0)
    );
    println!(
        "After the instructions, when register a starts at 1, register b has the value {}",
        run(&instructions, 1)
    );
}
