pub fn solution() {
    let mut calories_list = include_str!("../data/day1.txt")
        .split("\n\n")
        .map(|x| x.lines().map(|x| x.parse::<u32>().unwrap()).sum::<u32>())
        .collect::<Vec<u32>>();
    calories_list.sort_by(|a, b| b.cmp(a));

    println!(
        "The elf with max amount of calories has {} calories",
        calories_list[0]
    );
    println!(
        "The top three elfs with max amount of calories has {} calories",
        calories_list.iter().take(3).sum::<u32>()
    );
}
