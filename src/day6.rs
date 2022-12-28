pub fn solution() {
    let mut grid1 = vec![vec![false; 1000]; 1000];
    let mut grid2 = vec![vec![0usize; 1000]; 1000];
    let input = std::fs::read_to_string("data/day6.txt").unwrap();
    input.lines().for_each(|line| {
        let (part, p2) = line.split_once(" through ").unwrap();
        let (action, p1) = part.rsplit_once(' ').unwrap();
        let (x1, y1) = p1.split_once(',').unwrap();
        let (x1, y1) = (x1.parse::<usize>().unwrap(), y1.parse::<usize>().unwrap());
        let (x2, y2) = p2.split_once(',').unwrap();
        let (x2, y2) = (x2.parse::<usize>().unwrap(), y2.parse::<usize>().unwrap());
        (x1..=x2).for_each(|x| {
            (y1..=y2).for_each(|y| match action {
                "turn on" => {
                    grid1[x][y] = true;
                    grid2[x][y] += 1;
                }
                "turn off" => {
                    grid1[x][y] = false;
                    grid2[x][y] = (grid2[x][y]).saturating_sub(1)
                }
                "toggle" => {
                    grid1[x][y] = !grid1[x][y];
                    grid2[x][y] += 2;
                }
                _ => (),
            });
        });
    });

    println!(
        "There are {} lights lit",
        grid1.iter().flatten().map(|x| *x as usize).sum::<usize>()
    );

    println!(
        "The total brightness is: {}",
        grid2.iter().flatten().sum::<usize>()
    );
}
