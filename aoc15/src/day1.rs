pub fn solution() {
    let input = include_str!("../data/day1.txt");
    println!(
        "The instructions takes santa to floor: {}",
        input.chars().fold(0, |acc, c| {
            match c {
                '(' => acc + 1,
                ')' => acc - 1,
                _ => acc,
            }
        })
    );

    let mut floor = 0;
    println!(
        "The {}th character makes santa enter the basement",
        input
            .chars()
            .position(|c| {
                match c {
                    '(' => floor += 1,
                    ')' => floor -= 1,
                    _ => (),
                };
                floor == -1
            })
            .unwrap()
            + 1
    );
}
