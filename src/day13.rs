use std::str::FromStr;
#[derive(PartialEq, Eq)]
enum Node {
    List(Vec<Node>),
    Leaf(usize),
}

impl FromStr for Node {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut stack: Vec<Vec<Node>> = Vec::new();
        let mut current_list: Vec<Node> = Vec::new();
        let mut chars = s.trim().chars();
        while let Some(c) = chars.next() {
            match c {
                '[' => {
                    stack.push(current_list);
                    current_list = Vec::new();
                }
                ']' => {
                    let mut node = stack.pop().unwrap();
                    node.push(Node::List(current_list));
                    current_list = node;
                }
                ',' => (),
                '0'..='9' => {
                    let mut n = c.to_digit(10).unwrap();
                    for number in chars.by_ref() {
                        match number {
                            '0'..='9' => n = n * 10 + number.to_digit(10).unwrap(),
                            ']' => {
                                current_list.push(Node::Leaf(n as usize));
                                let mut node = stack.pop().unwrap();
                                node.push(Node::List(current_list));
                                current_list = node;
                                break;
                            }
                            ',' => {
                                current_list.push(Node::Leaf(n as usize));
                                break;
                            }
                            _ => unreachable!(),
                        }
                    }
                }
                _ => unreachable!(),
            }
        }
        Ok(current_list.pop().unwrap())
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Node::List(left), Node::List(right)) => left.cmp(right),
            (Node::Leaf(left), Node::List(right)) => vec![Node::Leaf(*left)].cmp(right),
            (Node::List(left), Node::Leaf(right)) => left.cmp(&vec![Node::Leaf(*right)]),
            (Node::Leaf(left), Node::Leaf(right)) => left.cmp(right),
        }
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub fn solution() {
    let pairs = std::fs::read_to_string("data/day13.txt").unwrap()
        .split("\n\n")
        .map(|pair| {
            pair.split_once('\n')
                .map(|(node_x, node_y)| {
                    (
                        node_x.parse::<Node>().unwrap(),
                        node_y.parse::<Node>().unwrap(),
                    )
                })
                .unwrap()
        })
        .collect::<Vec<_>>();

    // Part 1
    let sum_pairs = pairs
        .iter()
        .enumerate()
        .filter(|(_, (left, right))| left < right)
        .map(|(i, _)| i + 1)
        .sum::<usize>();
    println!(
        "Sum of par indices that have the right order: {}",
        sum_pairs
    );

    // Part 2
    let mut nodes = pairs
        .iter()
        .flat_map(|(left, right)| vec![left, right])
        .collect::<Vec<_>>();
    let node_2 = "[[2]]".parse::<Node>().unwrap();
    let node_6 = "[[6]]".parse::<Node>().unwrap();
    nodes.push(&node_2);
    nodes.push(&node_6);
    nodes.sort_unstable();
    let i_2 = nodes.iter().position(|node| node == &&node_2).unwrap() + 1;
    let i_6 = nodes.iter().position(|node| node == &&node_6).unwrap() + 1;
    println!("Decoder key: {}", i_2 * i_6);
}
