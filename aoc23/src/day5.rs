#[derive(Clone, Debug)]
pub struct Map {
    destination_range_start: Vec<u64>,
    source_range_start: Vec<u64>,
    range_length: Vec<u64>,
}

pub fn generator(input: &str) -> (Vec<u64>, Vec<Map>) {
    if let Some((seeds_str, maps_str)) = input.split_once('\n') {
        let seeds = seeds_str
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();
        let maps = ("\n".to_owned() + maps_str)
            .split("\n\n")
            .skip(1)
            .map(|m| {
                let (mut destination_range_start, mut source_range_start, mut range_length) =
                    (vec![], vec![], vec![]);
                m.lines().skip(1).for_each(|numbers| {
                    let number_vec: Vec<u64> = numbers
                        .split_whitespace()
                        .map(|n| n.parse().unwrap())
                        .collect();
                    destination_range_start.push(number_vec[0]);
                    source_range_start.push(number_vec[1]);
                    range_length.push(number_vec[2]);
                });
                Map {
                    destination_range_start,
                    source_range_start,
                    range_length,
                }
            })
            .collect();
        return (seeds, maps);
    }
    (vec![], vec![])
}

fn find_destination(source: &u64, map: &Map) -> u64 {
    if let Some((i, _)) = map
        .source_range_start
        .iter()
        .enumerate()
        .find(|&(i, &x)| (x..x + map.range_length[i]).contains(source))
    {
        return ((map.destination_range_start[i] as i64 - map.source_range_start[i] as i64)
            + *source as i64) as u64;
    }
    *source
}

pub fn part_1((seeds, maps): &(Vec<u64>, Vec<Map>)) -> u64 {
    seeds
        .iter()
        .map(|seed| {
            maps.iter()
                .fold(*seed, |acc, map| find_destination(&acc, map))
        })
        .min()
        .unwrap()
}

pub fn part_2((seeds, maps): &(Vec<u64>, Vec<Map>)) -> u64 {
    let mut ranges = seeds
        .chunks_exact(2)
        .filter_map(|c| {
            if let &[start, range] = c {
                Some((start, start + range - 1))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    maps.iter().for_each(|map| {
        let mut destination_ranges = Vec::new();

        ranges.iter().cloned().for_each(|(start, end)| {
            let mut current_ranges = vec![(start, end)];

            'outer: while let Some((current_start, current_end)) = current_ranges.pop() {
                for i in 0..map.source_range_start.len() {
                    let source_start = map.source_range_start[i];
                    let source_end = source_start + map.range_length[i] - 1;

                    if current_start > source_end || current_end < source_start {
                        continue;
                    }

                    let new_start = u64::max(current_start, source_start);
                    let new_end = u64::min(current_end, source_end);

                    destination_ranges.push((
                        find_destination(&new_start, map),
                        find_destination(&new_end, map),
                    ));

                    if new_start > current_start {
                        current_ranges.push((current_start, new_start - 1));
                    }

                    if new_end < current_end {
                        current_ranges.push((new_end + 1, current_end));
                    }
                    continue 'outer;
                }
                destination_ranges.push((current_start, current_end));
            }
        });
        ranges = destination_ranges;
    });
    *ranges.iter().map(|(start, _)| start).min().unwrap()
}
