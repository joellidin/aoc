pub fn solution() {
    let input = std::fs::read_to_string("data/day4.txt").unwrap();
    let lines = input.trim().split("\n");

    let result = lines.fold((0, 0), |acc, line| {
        let (a, b) = line.split_once(",").unwrap();
        let mut a = a.split("-").map(|num| num.parse::<u32>().unwrap());
        let mut b = b.split("-").map(|num| num.parse::<u32>().unwrap());
        let (a_min, a_max) = (a.next().unwrap(), a.next().unwrap());
        let (b_min, b_max) = (b.next().unwrap(), b.next().unwrap());
        let point1 =
            if ((a_min <= b_min) && (a_max >= b_max)) || ((b_min <= a_min) && (b_max >= a_max)) {
                1
            } else {
                0
            };
        let point2 = if (a_max >= b_min) && (b_max >= a_min) {
            1
        } else {
            0
        };
        (acc.0 + point1, acc.1 + point2)
    });
    println!("Complete overlaps: {}", result.0);
    println!("Partial overlaps {}", result.1);
}
