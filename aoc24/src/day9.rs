use std::collections::HashMap;

type Disk = Vec<Option<usize>>;
type FileLengths = HashMap<usize, u32>;
pub fn generator(input: &str) -> (Disk, FileLengths) {
    let mut disk_map = Vec::with_capacity(input.len());
    let mut file_lengths = HashMap::new();
    let numbers = input
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<_>>();
    let mut pointer = 0;
    numbers.chunks(2).enumerate().for_each(|(i, c)| {
        for _ in pointer..pointer + c[0] {
            disk_map.push(Some(i));
        }
        pointer += c[0];
        file_lengths.insert(i, c[0]);
        if c.len() == 2 {
            for _ in pointer..pointer + c[1] {
                disk_map.push(None);
            }
            pointer += c[1];
        }
    });
    (disk_map, file_lengths)
}

pub fn part_1(input: &(Disk, FileLengths)) -> u64 {
    let (mut disk, _) = input.clone();
    let mut i = 0;
    for j in (0..disk.len()).rev() {
        if disk[j].is_some() {
            while i < j {
                if disk[i].is_none() {
                    disk.swap(i, j);
                    i += 1;
                    break;
                }
                i += 1;
            }
        }
    }

    // Calculate the final checksum
    disk.iter()
        .enumerate()
        .filter_map(|(i, &n)| n.map(|fid| fid as u64 * i as u64))
        .sum()
}

pub fn part_2(input: &(Disk, FileLengths)) -> u64 {
    let (mut disk, file_lengths) = input.clone();
    let highest_file_number = *file_lengths.keys().max().unwrap();

    // Move files in descending order by file ID, as per Part Two rules.
    for fid in (0..=highest_file_number).rev() {
        let file_len = file_lengths[&fid];

        let file_start = disk
            .iter()
            .position(|&block| block == Some(fid))
            .expect("We must find the start of the file.");

        // Find free space to put the file
        let mut run_start = 0;
        let mut run_length = 0;
        let mut best_start = None;

        for (i, block) in disk.iter().enumerate().take(file_start) {
            if block.is_none() {
                run_length += 1;
            } else {
                run_length = 0;
                run_start = i + 1;
            }

            if run_length == file_len {
                best_start = Some(run_start);
                break;
            }
        }

        if let Some(new_start) = best_start {
            for offset in 0..file_len {
                // Clear the old file location
                disk[file_start + offset as usize] = None;

                // Place the file at the new location
                disk[new_start + offset as usize] = Some(fid);
            }
        }
    }

    // Calculate the final checksum
    disk.iter()
        .enumerate()
        .filter_map(|(i, &n)| n.map(|fid| fid as u64 * i as u64))
        .sum()
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
