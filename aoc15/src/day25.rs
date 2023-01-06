fn find_num(row: u64, col: u64) -> u64 {
    // The nth number is given by finding the triangle number for the row and
    // adding the column number. (The triangle number is the sum of the first
    // n natural numbers, so the nth row has a triangle number of n * (n + 1) / 2.)
    let mut num = 20151125;
    for _ in 0..(row + col - 2) * (row + col - 1) / 2 + col - 1 {
        num = (num * 252533) % 33554393;
    }
    num
}
pub fn solution() {
    let input = include_str!("../data/day25.txt");
    let row = input
        .split("row ")
        .nth(1)
        .unwrap()
        .split(',')
        .next()
        .unwrap()
        .parse::<u64>()
        .unwrap();
    let column = input
        .trim()
        .split("column ")
        .nth(1)
        .unwrap()
        .split('.')
        .next()
        .unwrap()
        .parse::<u64>()
        .unwrap();
    println!(
        "The code at row {row}, column {column} is {}",
        find_num(row, column)
    );
}
