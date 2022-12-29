use serde_json::Value;

pub fn solution() {
    let input = std::fs::read_to_string("data/day12.txt").unwrap();
    let v = serde_json::from_str::<serde_json::Value>(&input).unwrap();
    println!("Sum of all numbers: {}", sum_numbers(&v, ""));
    println!("Sum of all numbers without red objects: {}", sum_numbers(&v, "red"));
}

fn sum_numbers(v: &Value, filter: &str) -> i64 {
    match v {
        Value::Number(n) => n.as_i64().unwrap(),
        Value::Array(a) => a.iter().map(|v| sum_numbers(&v.clone(), filter)).sum(),
        Value::Object(o) => {
            if o.values().any(|v| v == filter) {
                0
            } else {
                o.values().map(|v| sum_numbers(&v.clone(), filter)).sum()
            }
        }
        _ => 0,
    }
}
