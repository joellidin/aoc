use std::{collections::HashMap, fmt::Display};

#[derive(Clone, Copy)]
pub enum Operator {
    And,
    Or,
    Xor,
}

impl Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operator::And => write!(f, " & "),
            Operator::Or => write!(f, " | "),
            Operator::Xor => write!(f, " ^ "),
        }
    }
}

type CircuitGates<'a> = HashMap<&'a str, (&'a str, Operator, &'a str)>;
type WireValues<'a> = HashMap<&'a str, u8>;

pub fn generator(input: &str) -> (WireValues, CircuitGates) {
    let (values, operators) = input.split_once("\n\n").unwrap();
    let values = values
        .lines()
        .map(|line| {
            let (name, value) = line.split_once(": ").unwrap();
            let value = value.parse().unwrap();
            (name, value)
        })
        .collect();
    let operations = operators
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let first_operand = parts.next().unwrap();
            let operator = match parts.next().unwrap() {
                "AND" => Operator::And,
                "OR" => Operator::Or,
                "XOR" => Operator::Xor,
                _ => unreachable!(),
            };
            let second_operand = parts.next().unwrap();
            parts.next();
            let assign = parts.next().unwrap();
            (assign, (first_operand, operator, second_operand))
        })
        .collect();
    (values, operations)
}

fn compute_value<'a>(node: &'a str, values: &mut WireValues<'a>, gates: &CircuitGates<'a>) -> u8 {
    if let Some(&val) = values.get(node) {
        return val;
    }

    let (left, op, right) = gates[node];

    let left_val = compute_value(left, values, gates);
    let right_val = compute_value(right, values, gates);

    let val = match op {
        Operator::And => left_val & right_val,
        Operator::Or => left_val | right_val,
        Operator::Xor => left_val ^ right_val,
    };

    values.insert(node, val);
    val
}

fn resolve_all_gates<'a>(values: &mut WireValues<'a>, gates: &CircuitGates<'a>) {
    for node in gates.keys() {
        compute_value(node, values, gates);
    }
}

fn swap_map_entries<K, V>(map: &mut HashMap<K, V>, k1: K, k2: K)
where
    K: Eq + std::hash::Hash,
    V: Clone,
{
    if let (Some(v1), Some(v2)) = (map.remove(&k1), map.remove(&k2)) {
        map.insert(k1, v2);
        map.insert(k2, v1);
    }
}

fn evaluate_circuit<'a>(
    x: Option<u64>,
    y: Option<u64>,
    values: &WireValues<'a>,
    gates: &CircuitGates<'a>,
) -> u64 {
    let mut values = values.clone();
    if let Some(x) = x {
        values
            .iter_mut()
            .filter(|(k, _)| k.starts_with("x"))
            .for_each(|(k, v)| {
                let idx = k.strip_prefix("x").unwrap().parse::<u64>().unwrap();
                *v = ((x >> idx) & 1) as u8;
            })
    };
    if let Some(y) = y {
        values
            .iter_mut()
            .filter(|(k, _)| k.starts_with("y"))
            .for_each(|(k, v)| {
                let idx = k.strip_prefix("y").unwrap().parse::<u64>().unwrap();
                *v = ((y >> idx) & 1) as u8;
            })
    };

    resolve_all_gates(&mut values, gates);
    values
        .iter()
        .filter(|(k, _)| k.starts_with("z"))
        .fold(0u64, |acc, (k, v)| {
            let raise = k.strip_prefix('z').unwrap().parse::<u64>().unwrap();
            if *v == 1 {
                acc + (1 << raise)
            } else {
                acc
            }
        })
}

fn trace_wire<'a>(
    node: &str,
    values: &WireValues<'a>,
    gates: &CircuitGates<'a>,
    depth: usize,
) -> String {
    if depth >= 4 {
        return "".to_string();
    }
    if values.contains_key(node) {
        return format!("{:>depth$}{}\n", "", node, depth = depth * 4);
    }

    let (left, operator, right) = gates[node];
    let op1 = trace_wire(left, values, gates, depth + 1);
    let op2 = trace_wire(right, values, gates, depth + 1);

    let operator_str = match operator {
        Operator::And => "&",
        Operator::Or => "|",
        Operator::Xor => "^",
    };

    // Ensure every argument is properly formatted and included
    format!(
        "{:>depth$}{}: {} {} {}\n{}{}",
        "",
        node,
        left,
        operator_str,
        right,
        op1,
        op2,
        depth = depth * 4
    )
}

pub fn part_1((values, gates): &(WireValues, CircuitGates)) -> u64 {
    evaluate_circuit(None, None, values, gates)
}

pub fn part_2((values, gates): &(WireValues, CircuitGates)) -> String {
    let mut gates = gates.clone();
    // swap_map_entries(&mut gates, "z10", "vcf");
    swap_map_entries(&mut gates, "z17", "fhg");
    swap_map_entries(&mut gates, "dvb", "fsq");
    swap_map_entries(&mut gates, "z39", "tnc");
    for i in 0..45 {
        for j in 0..45 {
            let x = 1 << i;
            let y = 1 << j;
            let z = evaluate_circuit(Some(x), Some(y), values, &gates);
            assert_eq!(
                z,
                x + y,
                "Faulty gate z{j:0>2}:\n{}\n{}\n{}",
                trace_wire(&format!("z{:0>2}", j - 1), values, &gates, 0),
                trace_wire(&format!("z{:0>2}", j), values, &gates, 0),
                trace_wire(&format!("z{:0>2}", j + 1), values, &gates, 0),
            );
        }
    }
    let mut a = ["z10", "vcf", "z17", "fhg", "fsq", "dvb", "z39", "tnc"];
    a.sort_unstable();
    a.join(",")
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_1: &str = r#"x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02
"#;

    const INPUT_2: &str = r#"x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj
"#;

    const INPUT_3: &str = r#"x00: 0
x01: 1
x02: 0
x03: 1
x04: 0
x05: 1
y00: 0
y01: 0
y02: 1
y03: 1
y04: 0
y05: 1

x00 AND y00 -> z05
x01 AND y01 -> z02
x02 AND y02 -> z01
x03 AND y03 -> z03
x04 AND y04 -> z04
x05 AND y05 -> z00
"#;
    #[test]
    fn part_1_test() {
        let generator_output = generator(INPUT_1);
        let result = part_1(&generator_output);
        assert_eq!(result, 4);

        let generator_output = generator(INPUT_2);
        let result = part_1(&generator_output);
        assert_eq!(result, 2024);
    }

    #[test]
    fn part_2_test() {
        let generator_output = generator(INPUT_3);
        let result = part_2(&generator_output);
        assert_eq!(result, "z00,z01,z02,z05".to_string());
    }
}
