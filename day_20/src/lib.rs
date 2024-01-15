use std::collections::{HashMap, VecDeque};
use crate::Power::*;
use crate::Pulse::*;
use crate::Module::*;

#[derive(Debug)]
pub enum Module<'a> {
    Broadcaster,
    FlipFlop(Power),
    Conjunction(HashMap<&'a str, Pulse>)
}

#[derive(Debug, Clone, PartialEq)]
pub enum Pulse {
    High,
    Low
}

#[derive(Debug, PartialEq)]
pub enum Power {
    On,
    Off,
}

pub struct Setup<'a> {
    pub destinations: HashMap<&'a str, Vec<&'a str>>,
    pub modules: HashMap<&'a str, Module<'a>>,
    pub cycles: HashMap<&'a str, Option<usize>>,
    pub rx_connected: &'a str,
}

pub fn parse(data: &str) -> Setup {
    let parsed: Vec<_> = data.lines()
        .map(|line| {
            let (typelabel, connections) = line.split_once(" -> ").unwrap();
            let destinations: Vec<&str> = connections.split(',').map(str::trim).collect();
            let (name, module) = match &typelabel[0..1] {
                "%" => (&typelabel[1..], FlipFlop(Off)),
                "&" => (&typelabel[1..], Conjunction(HashMap::new())),
                "b" => (typelabel, Broadcaster),
                _ => panic!("Invalid input")
            };
            (name, destinations, module)
        })
        .collect();
    let mut modules: HashMap<&str, Module> = HashMap::new();
    let mut destinations: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut rx_connected = "";
    for (name, connections, module) in parsed {
        modules.insert(name, module);
        destinations.insert(name, connections);
    }
    for (name, connections) in &destinations {
        for conn in connections {
            if modules.get(conn).is_none() {
                rx_connected = name;
            }
        }
    }
    for (source, dests) in &destinations {
        for dest in dests {
            if let Some(Conjunction(state)) = modules.get_mut(dest) {
                state.insert(source, Low);
            }
        }
    }
    let cycles = if !rx_connected.is_empty() {
         match &modules[rx_connected] {
            Conjunction(m) => m.iter()
                .map(|(&m, _)| (m, None))
                .collect::<HashMap<_, _>>(),
            _ => unreachable!(),
        }
    } else {
       HashMap::new()
    };
    Setup{destinations, modules, cycles, rx_connected}
}

pub fn solve(setup: Setup) -> (usize, usize, HashMap<&str, Option<usize>>) {
    let destinations = setup.destinations;
    let mut modules = setup.modules;
    let mut cycles = setup.cycles;
    let rx_connector = setup.rx_connected;

    let mut low_count = 0;
    let mut high_count = 0;
    'outer: for button_presses in 1.. {
        let mut deque = VecDeque::from_iter([(Low, "button", "broadcaster")]);
        while !deque.is_empty() {
            let (pulse, prev, next) = deque.pop_front().unwrap();
            if button_presses <= 1000 {
                match pulse {
                    High => high_count += 1,
                    Low => low_count += 1,
                }
            } else if rx_connector.is_empty() {
                break 'outer;
            }
            if pulse == High && next == rx_connector {
                let v = cycles.get_mut(prev).unwrap();
                if v.is_none() {
                    *v = Some(button_presses as usize);
                    if cycles.values().all(|o| o.is_some()) {
                        break 'outer;
                    }
                }
            }
            let module = &mut modules.get_mut(&next);
            match module {
                Some(Broadcaster) => {
                    for dest in &destinations[&next] {
                        deque.push_back((pulse.clone(), &next, &dest))
                    }
                },
                Some(FlipFlop(p)) => {
                    match pulse {
                        Low => {
                            let next_pulse = if *p == On {
                                *p = Off;
                                Low
                            } else {
                                *p = On;
                                High
                            };
                            for dest in &destinations[&next] {
                                deque.push_back((next_pulse.clone(), &next, &dest))
                            }
                        },
                        _ => { /* Do nothing */ }
                    }
                },
                Some(Conjunction(ref mut state)) => {
                    state.insert(prev, pulse.clone());
                    let next_pulse = if state.iter().all(|(_, p)| *p == High) {
                        Low
                    } else {
                        High
                    };
                    for dest in &destinations[&next] {
                        deque.push_back((next_pulse.clone(), &next, &dest))
                    }
                }
                None => {}
            }
        }
    }
    (low_count, high_count, cycles)
}
