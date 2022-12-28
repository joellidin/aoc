pub fn solution() {
    let input = std::fs::read_to_string("data/day10.txt").unwrap();
    let mut chars = input.trim().chars().collect::<Vec<_>>();
    let mut res = String::new();
    (0..50).for_each(|i| {
        let mut count = 1;
        res.clear();
        (0..chars.len()).for_each(|i| {
            let c = chars[i];
            if i == chars.len() - 1 || chars[i + 1] != c {
                res.push_str(&count.to_string());
                res.push(c);
                count = 1;
                return;
            }
            count += 1;
        });
        if i == 39 {
            println!("Length of string after 40 iterations: {}", res.len());
        }
        chars = res.chars().collect();
    });
    println!("Length of string after 50 iterations: {}", res.len());
}
