pub fn solution() {
    let mut dirs = vec![("/", 0usize)];
    let mut final_dirs = vec![];
    let mut sum_directories = 0;

    let input = include_str!("../data/day7.txt");
    input
        .trim()
        .split('\n')
        .skip(1)
        .filter(|line| *line != "$ ls" && !line.starts_with("dir"))
        .map(|line| {
            if let Some(dir) = line.strip_prefix("$ cd ") {
                let dir = dir;
                if dir == ".." {
                    let (name, size) = dirs.pop().unwrap();
                    if size < 100_000 {
                        sum_directories += size;
                    }
                    dirs.last_mut().unwrap().1 += size;
                    final_dirs.push((name, size))
                } else {
                    dirs.push((dir, 0));
                }
                return;
            }
            let (size, _) = line.split_once(' ').unwrap();
            dirs.last_mut().unwrap().1 += size.parse::<usize>().unwrap();
        })
        .for_each(drop);

    while !dirs.is_empty() {
        let (name, size) = dirs.pop().unwrap();
        final_dirs.push((name, size));
        if let Some((_, _)) = dirs.last() {
            dirs.last_mut().unwrap().1 += size;
        }
    }

    let needed_space = 30_000_000 - (70_000_000 - final_dirs.last().unwrap().1);
    let smallest_dir = final_dirs
        .iter()
        .filter(|(_, size)| size > &needed_space)
        .map(|(_, size)| size)
        .min()
        .unwrap();
    println!("Total sum of directories below 100_000: {sum_directories}");
    println!("Smallest directory needed to be deleted: {smallest_dir}");
}
