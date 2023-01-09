pub fn generator(input: &str) -> Vec<u32> {
        let mut calories_list = input.split("\n\n")
        .map(|x| x.lines().map(|x| x.parse::<u32>().unwrap()).sum::<u32>())
        .collect::<Vec<u32>>();
    calories_list.sort_by(|a, b| b.cmp(a));
    calories_list
}

pub fn part_1(input: &[u32]) -> u32 {
    input[0]
}

pub fn part_2(input: &[u32]) -> u32 {
    input.iter().take(3).sum::<u32>()
}
