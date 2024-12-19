use std::cmp::Ordering;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum DiskEntry {
    File { id: usize, size: usize },
    Blank { size: usize },
}

pub fn generator(input: &str) -> Vec<DiskEntry> {
    input
        .char_indices()
        .map(|(i, c)| {
            let size = c.to_digit(10).unwrap() as usize;
            if i % 2 == 0 {
                let id = i / 2;
                DiskEntry::File { id, size }
            } else {
                DiskEntry::Blank { size }
            }
        })
        .collect::<Vec<_>>()
}

fn calculate_checksum(memory: &[DiskEntry]) -> u64 {
    memory
        .iter()
        .fold((0, 0), |mut acc, &disk| match disk {
            DiskEntry::File { id, size } => {
                for i in 0..size {
                    acc.0 += id as u64 * (i as u64 + acc.1 as u64)
                }
                acc.1 += size;
                acc
            }
            DiskEntry::Blank { size } => {
                acc.1 += size;
                acc
            }
        })
        .0
}

pub fn part_1(input: &[DiskEntry]) -> u64 {
    let mut memory = input
        .iter()
        .flat_map(|&entry| {
            let mut flattened_entry = Vec::new();
            match entry {
                DiskEntry::File { id, size } => {
                    (0..size).for_each(|_| flattened_entry.push(DiskEntry::File { id, size: 1 }));
                }
                DiskEntry::Blank { size } => {
                    (0..size).for_each(|_| flattened_entry.push(DiskEntry::Blank { size: 1 }));
                }
            }
            flattened_entry
        })
        .collect::<Vec<_>>();

    let mut i = 0;
    for j in (0..memory.len()).rev() {
        if let DiskEntry::File { .. } = memory[j] {
            while i < j {
                if let DiskEntry::Blank { .. } = memory[i] {
                    memory.swap(i, j);
                    i += 1;
                    break;
                }
                i += 1;
            }
        }
    }

    calculate_checksum(&memory)
}

pub fn part_2(input: &[DiskEntry]) -> u64 {
    let mut memory = input.to_vec();
    for i in (0..memory.len()).rev() {
        if let DiskEntry::File {
            id,
            size: file_size,
        } = memory[i]
        {
            for j in 0..i {
                if let DiskEntry::Blank { size: blank_size } = memory[j] {
                    let file_compare = blank_size.cmp(&file_size);
                    if let Ordering::Equal | Ordering::Greater = file_compare {
                        memory[i] = DiskEntry::Blank { size: file_size };
                        memory[j] = DiskEntry::File {
                            id,
                            size: file_size,
                        };
                        if let Ordering::Greater = file_compare {
                            memory.insert(
                                j + 1,
                                DiskEntry::Blank {
                                    size: blank_size - file_size,
                                },
                            );
                        }
                        break;
                    }
                }
            }
        }
    }
    calculate_checksum(&memory)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"2333133121414131402"#;

    #[test]
    fn part_1_test() {
        let generator_output = generator(INPUT);
        let result = part_1(&generator_output);
        assert_eq!(result, 1928);
    }

    #[test]
    fn part_2_test() {
        let generator_output = generator(INPUT);
        let result = part_2(&generator_output);
        assert_eq!(result, 2858);
    }
}
