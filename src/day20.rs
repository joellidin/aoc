fn n_presents_slow(house_number: u32, start: u32, factor: u32) -> u32 {
    (1..=(house_number as f32).sqrt() as u32)
        .filter(|&i| house_number % i == 0)
        .map(|i| {
            if i >= start && i != (house_number / i) {
                i * factor + (house_number / i) * factor
            } else if (house_number / i) >= start {
                (house_number / i) * factor
            } else {
                0
            }
        })
        .sum()
}

fn get_min_house_fast(min_num: u32, max_visits: Option<u32>, factor: u32) -> Option<usize> {
    let mut v = vec![0; (min_num / 10) as usize];
    let max_visits = max_visits.unwrap_or(u32::max_value());
    for i in 1..=min_num {
        let mut num = i;
        let mut num_vistits = 0;
        while num < v.len() as u32 && num_vistits < max_visits {
            v[num as usize] += i * factor;
            num += i;
            num_vistits += 1
        }
    }
    v.iter().position(|num| *num >= min_num)
}

pub fn solution() {
    let num = std::fs::read_to_string("data/day20.txt")
        .unwrap()
        .trim()
        .parse::<u32>()
        .unwrap();

    let mut i = 1;
    loop {
        if n_presents_slow(i, 1, 10) >= num {
            println!("Slow: Lowest house number (part 1): {i}");
            break;
        }
        i += 1;
    }

    i = 1;
    loop {
        if n_presents_slow(i, i / 50 + 1, 11) >= num {
            println!("Slow: Lowest house number (part 2): {i}");
            break;
        }
        i += 1;
    }

    println!(
        "Fast: Lowest house number (part 1): {}",
        get_min_house_fast(num, None, 10).unwrap()
    );
    println!(
        "Fast: Lowest house number (part 2): {}",
        get_min_house_fast(num, Some(50), 11).unwrap()
    );
}
