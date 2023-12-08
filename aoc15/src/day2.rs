pub fn solution() {
    let input = include_str!("../data/day2.txt");
    let (part1, part2) = input.lines().fold((0, 0), |(acc_paper, acc_string), line| {
        let mut v = line
            .split('x')
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        v.sort_unstable();
        let sides = [v[0] * v[1], v[1] * v[2], v[2] * v[0]];
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
