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
        .map(|seed| process_seed(seed, maps))
        .min()
        .unwrap()
}

pub fn part_2((seeds, maps): &(Vec<u64>, Vec<Map>)) -> u64 {
    seeds
        .chunks_exact(2)
        .flat_map(|chunk| chunk[0]..chunk[0] + chunk[1])
        .map(|seed| process_seed(&seed, maps))
        .min()
        .unwrap()
}

fn process_seed(seed: &u64, maps: &[Map]) -> u64 {
    maps.iter()
        .fold(*seed, |acc, map| find_destination(&acc, map))
}
