pub fn solution() {
    let input = std::fs::read_to_string("data/day8.txt").unwrap();
    let part1 = input.lines().fold(0, |acc, line| {
        let tot_chars = line.len();
        // Calculate the number of charcaters in memory
        let memory_str = line.replace("\\\\", "a").replace("\\\"", "a");
        let chars = memory_str.chars().collect::<Vec<char>>();
        let memory_chars =
            memory_str.len() - 3 * chars.windows(2).filter(|w| w == &['\\', 'x']).count() - 2;
        acc + tot_chars - memory_chars
    });
    println!("Characters of code - characters of memory = {part1}");

    let part2 = input.lines().fold(0, |acc, line| {
        let encoded_string = line.replace('\\', "\\\\").replace('\"', "\\\"");
        acc + encoded_string.len() + 2 - line.len()
    });
    println!("Characters of encoded string - characters of code = {part2}");
}
