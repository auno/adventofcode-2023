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

#[derive(Copy, Clone, Eq, PartialEq)]
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

#[aoc(day20, part1)]
fn part1(modules: &Input) -> usize {
    let mut modules = modules.clone();
    let mut queue = VecDeque::new();
    let mut count_high = 0;
    let mut count_low = 0;

    for _ in 0..1000 {
        queue.push_back(("button".to_string(), "broadcaster".to_string(), Pulse::Low));

        while let Some((source, target, pulse)) = queue.pop_front() {
            match pulse {
                Pulse::High => { count_high += 1; }
                Pulse::Low => { count_low += 1; }
            }

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
        }
    }

    count_high * count_low
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
}
