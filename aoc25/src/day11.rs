use std::collections::HashMap;

pub fn generator(input: &str) -> HashMap<&str, Vec<&str>> {
    let mut connections: HashMap<&str, Vec<&str>> = HashMap::new();
    input.lines().for_each(|line| {
        let from_idx = line.find(':').expect("Invalid input");
        let start_device = &line[..from_idx];
        line[from_idx + 2..]
            .split_whitespace()
            .for_each(|o| connections.entry(start_device).or_default().push(o));
    });
    connections
}

fn count_paths<'a>(
    current: &'a str,
    end: &str,
    graph: &HashMap<&'a str, Vec<&'a str>>,
    required: &[&str],
    mut seen: usize,
    memo: &mut HashMap<(&'a str, usize), u64>,
) -> u64 {
    // Update seen bitmask for required nodes
    for (i, &req) in required.iter().enumerate() {
        if current == req {
            seen |= 1 << i;
        }
    }

    if current == end {
        return if seen == (1 << required.len()) - 1 {
            1
        } else {
            0
        };
    }

    let key = (current, seen);
    if let Some(&cached) = memo.get(&key) {
        return cached;
    }

    let count = graph
        .get(current)
        .map(|neighbors| {
            neighbors
                .iter()
                .map(|&n| count_paths(n, end, graph, required, seen, memo))
                .sum()
        })
        .unwrap_or(0);

    memo.insert(key, count);
    count
}

pub fn part_1(input: &HashMap<&str, Vec<&str>>) -> u64 {
    let mut memo = HashMap::new();
    count_paths("you", "out", input, &[], 0, &mut memo)
}

pub fn part_2(input: &HashMap<&str, Vec<&str>>) -> u64 {
    let mut memo = HashMap::new();
    count_paths("svr", "out", input, &["fft", "dac"], 0, &mut memo)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out
"#;

    const INPUT2: &str = r#"svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out
"#;

    #[test]
    fn part_1_test() {
        let generator_output = generator(INPUT);
        let result = part_1(&generator_output);
        assert_eq!(result, 5);
    }

    #[test]
    fn part_2_test() {
        let generator_output = generator(INPUT2);
        let result = part_2(&generator_output);
        assert_eq!(result, 2);
    }
}
