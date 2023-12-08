pub fn generator(input: &str) -> Vec<(&str, u32)> {
    let mut dirs = vec![("/", 0u32)];
    let mut final_dirs = Vec::new();

    input
        .trim()
        .lines()
        .skip(1)
        .filter(|line| *line != "$ ls" && !line.starts_with("dir"))
        .map(|line| {
            if let Some(dir) = line.strip_prefix("$ cd ") {
                if dir == ".." {
                    let (name, size) = dirs.pop().unwrap();
                    dirs.last_mut().unwrap().1 += size;
                    final_dirs.push((name, size))
                } else {
                    dirs.push((dir, 0));
                }
                return;
            }
            let (size, _) = line.split_once(' ').unwrap();
            dirs.last_mut().unwrap().1 += size.parse::<u32>().unwrap();
        })
        .for_each(drop);

    while let Some((name, size)) = dirs.pop() {
        final_dirs.push((name, size));
        if let Some((_, _)) = dirs.last() {
            dirs.last_mut().unwrap().1 += size;
        }
    }
    final_dirs
}

pub fn part_1(input: &[(&str, u32)]) -> u32 {
    input
        .iter()
        .filter(|(_, size)| size < &100_000)
        .fold(0, |acc, (_, size)| acc + size)
}

pub fn part_2(input: &[(&str, u32)]) -> u32 {
    let needed_space = 30_000_000 - (70_000_000 - input.last().unwrap().1);
    *input
        .iter()
        .filter(|(_, size)| size > &needed_space)
        .map(|(_, size)| size)
        .min()
        .unwrap()
}
