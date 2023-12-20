use std::collections::{HashMap, VecDeque};
use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;
use anyhow::{Context, Result};
use itertools::Itertools;

#[derive(Clone)]
enum Module {
    FlipFlop(State, Vec<String>),
    Conjunction(HashMap<String, Pulse>, Vec<String>),
    Broadcaster(Vec<String>),
    Sink(),
}

#[derive(Copy, Clone)]
enum State {
    On,
    Off,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Pulse {
    High,
    Low,
}

type Input = HashMap<String, Module>;

#[aoc_generator(day20)]
fn parse(input: &str) -> Result<Input> {
    let pattern = Regex::new(r"^([%&]?)([a-z]+) -> (.*)$")?;

    let modules = input
        .lines()
        .map(|line| {
            let caps = pattern.captures(line).context(format!("Invalid input: {line}"))?;
            let (_, [type_indicator, name, targets]) = caps.extract();
            let targets = targets.split(", ").map(str::to_string).collect_vec();

            Ok((type_indicator, name, targets))
        })
        .collect::<Result<Vec<_>>>()?;

    let sources: HashMap<&str, Vec<&str>> = modules
        .iter()
        .fold(HashMap::new(), |mut acc, (_, name, targets)| {
            for target in targets {
                let sources = acc.entry(target.as_str()).or_default();
                sources.push(name);
            }

            acc
        });

    Ok(
        modules
            .iter()
            .map(|(type_indicator, name, targets)| {
                match *type_indicator {
                    "" => (name.to_string(), Module::Broadcaster(targets.clone())),
                    "%" => (name.to_string(), Module::FlipFlop(State::Off, targets.clone())),
                    "&" => (
                        name.to_string(),
                        Module::Conjunction(
                            sources.get(name).unwrap().iter().map(|source| (source.to_string(), Pulse::Low)).collect(),
                            targets.clone(),
                        ),
                    ),
                    _ => unreachable!(),
                }
            })
            .collect()
    )
}

fn simulate(modules: &mut HashMap<String, Module>) -> Vec<(String, String, Pulse)> {
    let mut queue = VecDeque::from([("button".to_string(), "broadcaster".to_string(), Pulse::Low)]);
    let mut processed_pulses = vec![];

    while let Some((source, target, pulse)) = queue.pop_front() {
        match modules.entry(target.to_string()).or_insert(Module::Sink()) {
            Module::FlipFlop(state, targets) => {
                if pulse == Pulse::Low {
                    let new_pulse = match state {
                        State::Off => {
                            *state = State::On;
                            Pulse::High
                        },
                        State::On => {
                            *state = State::Off;
                            Pulse::Low
                        },
                    };

                    for new_target in targets {
                        queue.push_back((target.to_string(), new_target.to_string(), new_pulse));
                    }
                }
            }
            Module::Conjunction(memory, targets) => {
                *memory.get_mut(&source).unwrap() = pulse;

                let new_pulse = if memory.values().all(|&p| p == Pulse::High) {
                    Pulse::Low
                } else {
                    Pulse::High
                };

                for new_target in targets {
                    queue.push_back((target.to_string(), new_target.to_string(), new_pulse));
                }
            }
            Module::Broadcaster(targets) => {
                for new_target in targets {
                    queue.push_back((target.to_string(), new_target.to_string(), pulse));
                }
            }
            Module::Sink() => {}
        }

        processed_pulses.push((source, target, pulse));
    }

    processed_pulses
}

fn gcd(a: usize, b: usize) -> usize {
    let (mut a, mut b) = (a, b);

    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }

    a
}

fn lcm(a: usize, b: usize) -> usize {
    (a * b) / gcd(a, b)
}

#[aoc(day20, part1)]
fn part1(modules: &Input) -> usize {
    let mut modules = modules.clone();
    let mut count_high = 0;
    let mut count_low = 0;

    for _ in 0..1000 {
        for (_, _, pulse) in simulate(&mut modules) {
            match pulse {
                Pulse::High => { count_high += 1; }
                Pulse::Low => { count_low += 1; }
            }
        }
    }

    count_high * count_low
}

#[aoc(day20, part2)]
fn part2(modules: &Input) -> usize {
    let mut modules = modules.clone();

    let mut rx_source_cycles = modules
        .iter()
        .find(|(_, module)| match module {
            Module::Conjunction(_, targets) => targets.contains(&"rx".to_string()),
            _ => false,
        })
        .map(|(_, module)| match module {
            Module::Conjunction(sources, _) => sources.keys().map(|source| (source.to_string(), None)),
            _ => unimplemented!(),
        })
        .unwrap()
        .collect::<HashMap<_, _>>();

    for button_presses in 1.. {
        for (source, _, pulse) in simulate(&mut modules) {
            for (rx_source, cycle_count) in &mut rx_source_cycles {
                if cycle_count.is_none() && &source == rx_source && pulse == Pulse::High {
                    *cycle_count = Some(button_presses);
                }
            }
        }

        if rx_source_cycles.values().all(|cycle_count| cycle_count.is_some()) {
            break;
        }
    }

    rx_source_cycles
        .values()
        .copied()
        .map(Option::unwrap)
        .reduce(lcm)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        assert_eq!(32000000, part1(&parse(include_str!("../test_input/day20.part1.32000000.txt")).unwrap()));
    }

    #[test]
    fn part1_example2() {
        assert_eq!(11687500, part1(&parse(include_str!("../test_input/day20.part1.11687500.txt")).unwrap()));
    }

    #[test]
    fn part1_input() {
        assert_eq!(856482136, part1(&parse(include_str!("../input/2023/day20.txt")).unwrap()));
    }

    #[test]
    fn part2_input() {
        assert_eq!(224046542165867, part2(&parse(include_str!("../input/2023/day20.txt")).unwrap()));
    }
}
