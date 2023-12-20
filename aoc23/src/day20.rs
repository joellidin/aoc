use std::collections::{HashMap, VecDeque};
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub enum Module {
    FlipFlop {
        name: String,
        is_on: bool,
        destination_modules: Vec<String>,
    },
    Conjuction {
        name: String,
        connected_modules: HashMap<String, Pulse>,
        destination_modules: Vec<String>,
    },
    Broadcaster {
        name: String,
        destination_modules: Vec<String>,
    },
    Button {
        name: String,
        destination_modules: Vec<String>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pulse {
    Low,
    High,
}

impl Module {
    fn receive_signal(&mut self, input_pulse: Pulse, sender_name: String) -> Option<Pulse> {
        match self {
            Module::FlipFlop { is_on, .. } => match input_pulse {
                Pulse::Low => {
                    *is_on = !*is_on;
                    if *is_on {
                        Some(Pulse::High)
                    } else {
                        Some(Pulse::Low)
                    }
                }
                Pulse::High => None,
            },

            Module::Conjuction {
                connected_modules, ..
            } => {
                connected_modules.insert(sender_name, input_pulse);
                if connected_modules
                    .values()
                    .all(|pulse| matches!(pulse, Pulse::High))
                {
                    Some(Pulse::Low)
                } else {
                    Some(Pulse::High)
                }
            }
            Module::Broadcaster { .. } => Some(input_pulse),
            Module::Button { .. } => Some(Pulse::Low),
        }
    }

    fn get_name(&self) -> String {
        match self {
            Module::FlipFlop { name, .. }
            | Module::Conjuction { name, .. }
            | Module::Broadcaster { name, .. }
            | Module::Button { name, .. } => name.to_owned(),
        }
    }

    fn get_destination_modules(&self) -> &Vec<String> {
        match self {
            Module::FlipFlop {
                destination_modules,
                ..
            }
            | Module::Conjuction {
                destination_modules,
                ..
            }
            | Module::Broadcaster {
                destination_modules,
                ..
            }
            | Module::Button {
                destination_modules,
                ..
            } => destination_modules,
        }
    }
}

fn initialize_conjuctions_with_low_pulse(module_network: &mut HashMap<String, Module>) {
    // First, collect the updates in a vector
    let mut updates = Vec::new();
    for (module_name, module) in module_network.iter() {
        let destination_modules = module.get_destination_modules();
        for dest_module_name in destination_modules {
            updates.push((module_name.to_string(), dest_module_name.clone()));
        }
    }

    // Then, apply the updates
    for (module_name, dest_module_name) in updates {
        if let Some(Module::Conjuction {
            connected_modules, ..
        }) = module_network.get_mut(&dest_module_name)
        {
            connected_modules.insert(module_name, Pulse::Low);
        }
    }
}

fn simulate_button_press<F>(
    module_network: &mut HashMap<String, Module>,
    mut callback: F,
) -> (u32, u32)
where
    F: FnMut(&str, Pulse) -> bool,
{
    let mut signals_queue = VecDeque::new();
    let (mut n_low_pulses, mut n_high_pulses) = (0, 0);

    // Initial signal from the button
    let module_name = module_network.get("button").unwrap().get_name().to_string();
    // We know that button only have one destination
    let destination = module_network
        .get("button")
        .unwrap()
        .get_destination_modules()[0]
        .clone();
    let input_signal = Pulse::Low;
    if let Some(output_signal) = module_network
        .get_mut("button")
        .unwrap()
        .receive_signal(input_signal, "".to_owned())
    {
        signals_queue.push_back((module_name, output_signal, destination));
    }

    while let Some((sender_name, input_signal, destination)) = signals_queue.pop_front() {
        if callback(&sender_name, input_signal) {
            break;
        }
        match input_signal {
            Pulse::Low => n_low_pulses += 1,
            Pulse::High => n_high_pulses += 1,
        }

        if let Some(output_signal) = module_network
            .get_mut(&destination)
            .and_then(|module| module.receive_signal(input_signal, sender_name))
        {
            if let Some(module) = module_network.get(&destination) {
                let destination_modules = module.get_destination_modules();
                for dest in destination_modules {
                    signals_queue.push_back((destination.clone(), output_signal, dest.to_owned()));
                }
            }
        }
    }
    (n_low_pulses, n_high_pulses)
}

pub fn calculate_pulse_loop_count(
    input: &HashMap<String, Module>,
    target_module: &str,
    target_pulse: Pulse,
) -> u64 {
    let mut module_network = input.clone();
    let mut button_press_count = 0;

    let mut is_target_reached = false;

    while !is_target_reached {
        simulate_button_press(&mut module_network, |module_name, pulse| {
            if module_name == target_module && pulse == target_pulse {
                is_target_reached = true;
                true
            } else {
                false
            }
        });

        button_press_count += 1;
    }
    button_press_count
}

pub fn find_parent_modules(
    module_network: &HashMap<String, Module>,
    target_module: &str,
) -> Vec<String> {
    let mut parents = Vec::new();

    // Iterate over all modules in the map
    for (module_name, module) in module_network.iter() {
        // Check if the current module's destination_modules contains the target_module
        if module
            .get_destination_modules()
            .contains(&target_module.to_string())
        {
            parents.push(module_name.clone());
        }
    }
    parents
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    a / gcd(a, b) * b
}

pub fn generator(input: &str) -> HashMap<String, Module> {
    let mut module_network: HashMap<String, Module> = input
        .lines()
        .map(|l| {
            let (name_and_type, destination_modules_str) =
                l.split_once(" -> ").expect("Wrong input format");
            let destination_modules = destination_modules_str
                .split(", ")
                .map(String::from)
                .collect::<Vec<String>>();
            let module = match name_and_type.chars().next().unwrap() {
                '%' => Module::FlipFlop {
                    name: name_and_type.trim_start_matches('%').to_owned(),
                    is_on: false,
                    destination_modules,
                },

                '&' => Module::Conjuction {
                    name: name_and_type.trim_start_matches('&').to_owned(),
                    connected_modules: HashMap::new(),
                    destination_modules,
                },

                _ => Module::Broadcaster {
                    name: "broadcaster".to_owned(),
                    destination_modules,
                },
            };
            (module.get_name(), module)
        })
        .collect();

    module_network.insert(
        "button".to_owned(),
        Module::Button {
            name: "button".to_owned(),
            destination_modules: vec!["broadcaster".to_owned()],
        },
    );

    initialize_conjuctions_with_low_pulse(&mut module_network);

    module_network
}

pub fn part_1(input: &HashMap<String, Module>) -> u32 {
    let mut module_network = input.clone();
    let (mut n_low_pulses, mut n_high_pulses) = (0, 0);
    for _ in 0..1000 {
        let (low_pulses, high_pulses) = simulate_button_press(&mut module_network, |_, _| false);
        n_low_pulses += low_pulses;
        n_high_pulses += high_pulses;
    }
    n_low_pulses * n_high_pulses
}

pub fn part_2(input: &HashMap<String, Module>) -> u64 {
    let parent_of_rx = find_parent_modules(input, "rx");
    if parent_of_rx.len() != 1 {
        panic!("This solution only works for when 'rx' have one parent");
    }
    // Assuming that all parents here is a Conjuction
    find_parent_modules(input, &parent_of_rx[0])
        .iter()
        .fold(1, |acc, target| {
            lcm(acc, calculate_pulse_loop_count(input, target, Pulse::High))
        })
}
