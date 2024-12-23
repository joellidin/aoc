use std::collections::{HashMap, HashSet};

pub fn generator(input: &str) -> HashMap<&str, HashSet<&str>> {
    input.lines().fold(HashMap::new(), |mut acc, line| {
        let (a, b) = line.split_once("-").unwrap();
        acc.entry(a).or_default().insert(b);
        acc.entry(b).or_default().insert(a);
        acc
    })
}

fn bron_kerbosch<'a>(
    r: &mut HashSet<&'a str>,
    p: &mut HashSet<&'a str>,
    x: &mut HashSet<&'a str>,
    adjacency: &'a HashMap<&'a str, HashSet<&'a str>>,
    output: &mut Vec<HashSet<&'a str>>,
) {
    if p.is_empty() && x.is_empty() {
        output.push(r.clone());
        return;
    }

    let u = p
        .union(x)
        .max_by_key(|node| node.len())
        .expect("All nodes should have neighbours");

    let neighbors_of_u = &adjacency[u];

    for v in &*p - neighbors_of_u {
        let neighbors_of_v = &adjacency[v];
        r.insert(v);

        let mut p_intersect = p
            .intersection(neighbors_of_v)
            .copied()
            .collect::<HashSet<_>>();

        let mut x_intersect = x
            .intersection(neighbors_of_v)
            .copied()
            .collect::<HashSet<_>>();

        bron_kerbosch(
            &mut *r,
            &mut p_intersect,
            &mut x_intersect,
            adjacency,
            output,
        );

        r.remove(v);
        p.remove(v);
        x.insert(v);
    }
}

fn find_maximal_cliques<'a>(map: &'a HashMap<&'a str, HashSet<&'a str>>) -> Vec<HashSet<&'a str>> {
    let mut r = HashSet::new();
    let mut p = map.keys().copied().collect();
    let mut x = HashSet::new();

    let mut cliques = Vec::new();
    bron_kerbosch(&mut r, &mut p, &mut x, map, &mut cliques);

    cliques
}

pub fn part_1(input: &HashMap<&str, HashSet<&str>>) -> u32 {
    let mut res = 0;
    for a in input.keys() {
        let a_neighbours = &input[a];
        for b in a_neighbours {
            if b > a {
                for c in &input[b] {
                    if c > b
                        && a_neighbours.contains(c)
                        && [a, b, c].iter().any(|node| node.starts_with('t'))
                    {
                        res += 1;
                    }
                }
            }
        }
    }
    res
}

pub fn part_2(input: &HashMap<&str, HashSet<&str>>) -> String {
    let mut largest_clique = find_maximal_cliques(input)
        .iter()
        .max_by_key(|a| a.len())
        .expect("Must find a largest clique")
        .iter()
        .copied()
        .collect::<Vec<_>>();
    largest_clique.sort_unstable();
    largest_clique.join(",")
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn
"#;

    #[test]
    fn part_1_test() {
        let generator_output = generator(INPUT);
        let result = part_1(&generator_output);
        assert_eq!(result, 7);
    }

    #[test]
    fn part_2_test() {
        let generator_output = generator(INPUT);
        let result = part_2(&generator_output);
        assert_eq!(result, "co,de,ka,ta".to_string());
    }
}
