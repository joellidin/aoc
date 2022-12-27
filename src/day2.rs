pub fn solution() {
    let input = std::fs::read_to_string("data/day2.txt").unwrap();
    let (part1, part2) = input.lines().fold((0, 0), |(acc_paper, acc_string), line| {
        let mut v = line
            .split('x')
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        v.sort_unstable();
        let sides = vec![v[0] * v[1], v[1] * v[2], v[2] * v[0]];
        (
            acc_paper
                + sides.iter().fold(0, |acc, &side| acc + 2 * side)
                + sides.iter().min().unwrap(),
            acc_string
                + v.iter().take(2).fold(0, |acc, &side| acc + 2 * side)
                + v.iter().product::<usize>(),
        )
    });
    println!("The elves should order {part1} square feet of wrapping paper");
    println!("The elves should order {part2} feet of ribbon");
}
