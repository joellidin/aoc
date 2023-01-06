fn neighbours_on(lights: &Vec<Vec<bool>>, (i, j): (usize, usize)) -> u8 {
    let mut count = 0;
    (-1..=1).for_each(|di| {
        (-1..=1).for_each(|dj| {
            let (new_i, new_j) = (di + i as isize, dj + j as isize);
            if new_i >= 0
                && new_i < lights.len() as isize
                && new_j >= 0
                && new_j < lights[0].len() as isize
                && (di, dj) != (0, 0)
                && lights[new_i as usize][new_j as usize]
            {
                count += 1
            }
        })
    });
    count
}

fn update_lights(lights: &mut [Vec<bool>], corners_on: bool) {
    let copy_lights = lights.to_owned();
    let (w, h) = (lights[0].len(), lights.len());
    lights.iter_mut().enumerate().for_each(|(i, r)| {
        r.iter_mut().enumerate().for_each(|(j, c)| {
            let on_neigh = neighbours_on(&copy_lights, (i, j));
            match *c {
                true => *c &= on_neigh == 2 || on_neigh == 3,
                false => *c |= on_neigh == 3,
            }
            if corners_on && (i == 0 || i == h - 1) && (j == 0 || j == w - 1) {
                *c = true
            }
        });
    });
}

pub fn solution() {
    let mut lights = include_str!("../data/day18.txt")
        .lines()
        .map(|line| line.chars().map(|c| matches!(c, '#')).collect())
        .collect::<Vec<Vec<_>>>();
    let mut lights2 = lights.clone();
    (0..100).for_each(|_| update_lights(&mut lights, false));
    println!("After 100 steps {} lights are on", lights.iter().flatten().filter(|x| **x).count());

    // Part 2
    let (w, h) = (lights2[0].len(), lights2.len());
    for (i, j) in [(0, 0), (0, w - 1), (h - 1, 0), (h - 1, w - 1)].iter() {
        lights2[*i][*j] = true
    }
    (0..100).for_each(|_| update_lights(&mut lights2, true));
    println!(
        "With all corners always on, {} lights are on after 100 steps",
        lights2.iter().flatten().filter(|x| **x).count()
    );
}
