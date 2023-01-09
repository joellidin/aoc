use std::collections::{HashMap, HashSet, VecDeque};
use std::str::FromStr;

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct Valve {
    flow_rate: u32,
    leading_valves: Vec<String>,
}

impl FromStr for Valve {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (flow_rate, line) = s
            .strip_prefix("has flow rate=")
            .unwrap()
            .split_once(';')
            .unwrap();
        let valve_names = line
            .strip_prefix(" tunnels lead to valves ")
            .unwrap_or_else(|| line.strip_prefix(" tunnel leads to valve ").unwrap())
            .split(", ")
            .map(|name| name.to_owned())
            .collect::<Vec<_>>();
        let flow_rate = flow_rate.parse().unwrap();
        Ok(Valve {
            flow_rate,
            leading_valves: valve_names,
        })
    }
}

fn bfs(valves: &HashMap<String, Valve>, start_valve: Valve, time_left: i16) -> u32 {
    let mut q = VecDeque::new();
    let flow = 0;
    let open_valves: HashSet<Valve> = HashSet::new();
    q.push_front((&start_valve, time_left, flow, open_valves));
    let mut max_flow = 0;
    let mut visited_states = HashSet::new();
    while let Some((current_valve, time, flow, open_valves)) = q.pop_front() {
        if time <= 0 || visited_states.get(&(current_valve, time, flow)).is_some() {
            max_flow = std::cmp::max(flow, max_flow);
            continue;
        }
        if current_valve.flow_rate > 0 && open_valves.get(current_valve).is_none() {
            let mut new_open_valves = open_valves.clone();
            new_open_valves.insert(current_valve.to_owned());
            q.push_front((
                current_valve,
                time - 1,
                flow + open_valves
                    .iter()
                    .fold(0, |acc, valve| acc + valve.flow_rate),
                new_open_valves,
            ));
            visited_states.insert((current_valve, time, flow));
        }

        for leading_valve in current_valve.to_owned().leading_valves {
            let new_open_valves = open_valves.clone();
            let next_valve = valves.get(&leading_valve).unwrap();
            q.push_front((
                next_valve,
                time - 1,
                flow + open_valves
                    .iter()
                    .fold(0, |acc, valve| acc + valve.flow_rate),
                new_open_valves,
            ));
            visited_states.insert((current_valve, time, flow));
        }
    }
    max_flow
}

fn bfs2(valves: &HashMap<String, Valve>, start_valve: Valve, time_left: i16) -> u32 {
    let mut q = VecDeque::new();
    let flow = 0;
    let open_valves: HashSet<Valve> = HashSet::new();
    q.push_front((&start_valve, &start_valve, time_left, flow, open_valves));
    let mut max_flow = 0;
    let mut visited_states = HashSet::new();
    while let Some((elf_valve, elephant_valve, time, flow, open_valves)) = q.pop_front() {
        if time <= 0
            || visited_states
                .get(&(elf_valve, elephant_valve, time, flow))
                .is_some()
        {
            max_flow = std::cmp::max(flow, max_flow);
            continue;
        }

        if elf_valve.flow_rate > 0 && open_valves.get(elf_valve).is_none() {
            if elephant_valve.flow_rate > 0 && open_valves.get(elephant_valve).is_none() {
                let mut new_open_valves = open_valves.clone();
                new_open_valves.insert(elf_valve.to_owned());
                new_open_valves.insert(elephant_valve.to_owned());
                q.push_front((
                    elf_valve,
                    elephant_valve,
                    time - 1,
                    flow + open_valves
                        .iter()
                        .fold(0, |acc, valve| acc + valve.flow_rate),
                    new_open_valves,
                ));
            }
            elephant_valve
                .to_owned()
                .leading_valves
                .into_iter()
                .for_each(|leading_valve| {
                    let mut new_open_valves = open_valves.clone();
                    new_open_valves.insert(elf_valve.to_owned());
                    let next_valve = valves.get(&leading_valve).unwrap();
                    q.push_front((
                        elf_valve,
                        next_valve,
                        time - 1,
                        flow + open_valves
                            .iter()
                            .fold(0, |acc, valve| acc + valve.flow_rate),
                        new_open_valves,
                    ));
                });
        }

        if elephant_valve.flow_rate > 0 && open_valves.get(elephant_valve).is_none() {
            if elf_valve.flow_rate > 0 && open_valves.get(elf_valve).is_none() {
                let mut new_open_valves = open_valves.clone();
                new_open_valves.insert(elf_valve.to_owned());
                new_open_valves.insert(elephant_valve.to_owned());
                q.push_front((
                    elf_valve,
                    elephant_valve,
                    time - 1,
                    flow + open_valves
                        .iter()
                        .fold(0, |acc, valve| acc + valve.flow_rate),
                    new_open_valves,
                ));
            }

            elf_valve
                .to_owned()
                .leading_valves
                .into_iter()
                .for_each(|leading_valve| {
                    let mut new_open_valves = open_valves.clone();
                    new_open_valves.insert(elephant_valve.to_owned());
                    let next_valve = valves.get(&leading_valve).unwrap();
                    q.push_front((
                        next_valve,
                        elephant_valve,
                        time - 1,
                        flow + open_valves
                            .iter()
                            .fold(0, |acc, valve| acc + valve.flow_rate),
                        new_open_valves,
                    ));
                });
        }

        elf_valve
            .to_owned()
            .leading_valves
            .into_iter()
            .for_each(|leading_elf_valve| {
                elephant_valve
                    .to_owned()
                    .leading_valves
                    .into_iter()
                    .for_each(|leading_elpehant_valve| {
                        let new_open_valves = open_valves.clone();
                        let next_elf_valve = valves.get(&leading_elf_valve).unwrap();
                        let next_elelphant_valve = valves.get(&leading_elpehant_valve).unwrap();
                        q.push_front((
                            next_elf_valve,
                            next_elelphant_valve,
                            time - 1,
                            flow + open_valves
                                .iter()
                                .fold(0, |acc, valve| acc + valve.flow_rate),
                            new_open_valves,
                        ));
                    });
            });

        visited_states.insert((elf_valve, elephant_valve, time, flow));
    }
    max_flow
}

pub fn generator(input: &str) -> HashMap<String, Valve> {
    let mut valves = HashMap::new();
    input
        .trim_end()
        .split('\n')
        .map(|line| {
            let (valve_name, line) = line
                .strip_prefix("Valve ")
                .unwrap()
                .split_once(' ')
                .unwrap();
            valves.insert(valve_name.to_owned(), line.parse::<Valve>().unwrap());
        })
        .for_each(drop);
    valves
}

pub fn part_1(input: &HashMap<String, Valve>) -> u32 {
    bfs(input, input.get(&"AA".to_owned()).unwrap().to_owned(), 30)
}

pub fn part_2(input: &HashMap<String, Valve>) -> u32 {
    bfs2(input, input.get(&"AA".to_owned()).unwrap().to_owned(), 26)
}
