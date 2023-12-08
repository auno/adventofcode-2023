use std::collections::HashMap;
use aoc_runner_derive::{aoc, aoc_generator};
use scan_fmt::scan_fmt;
use anyhow::{bail, Error};
use itertools::Itertools;

enum Instruction {
    Left,
    Right,
}

impl TryFrom<char> for Instruction {
    type Error = Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'L' => Ok(Instruction::Left),
            'R' => Ok(Instruction::Right),
            _ => bail!("Invalid instruction: {value}"),
        }
    }
}

type Input = (Vec<Instruction>, HashMap<String, (String, String)>);

#[aoc_generator(day8)]
fn parse(input: &str) -> Input {
    let (instructions, map) = input.split_once("\n\n").unwrap();
    let instructions = instructions.chars().map(Instruction::try_from).map(Result::unwrap).collect();

    let map = map.lines()
        .map(|line| {
            let (from, l, r) = scan_fmt!(line, "{} = ({}, {})", String, String, String).unwrap();
            (from, (l, r))
        })
        .collect();

    (instructions, map)
}

fn distance(start: &str, end: &str, input: &Input) -> usize {
    let (instructions, map) = input;

    let mut location = start;
    let mut distance = 0;

    for instruction in instructions.iter().cycle() {
        if location.ends_with(end) {
            break;
        }

        location = match instruction {
            Instruction::Left => &map.get(location).unwrap().0,
            Instruction::Right => &map.get(location).unwrap().1,
        };

        distance += 1;
    }

    distance
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

#[aoc(day8, part1)]
fn part1(input: &Input) -> usize {
    distance("AAA", "ZZZ", input)
}

#[aoc(day8, part2)]
fn part2(input: &Input) -> usize {
    let (_, map) = input;

    let distances = map
        .keys()
        .filter(|k| k.ends_with('A'))
        .map(|start| distance(start, "Z", input))
        .collect_vec();

    distances
        .into_iter()
        .reduce(lcm)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        assert_eq!(2, part1(&parse(include_str!("../test_input/day08.part1.2.txt"))));
    }

    #[test]
    fn part1_example2() {
        assert_eq!(6, part1(&parse(include_str!("../test_input/day08.part1.6.txt"))));
    }

    #[test]
    fn part1_input() {
        assert_eq!(21883, part1(&parse(include_str!("../input/2023/day8.txt"))));
    }

    #[test]
    fn part2_example1() {
        assert_eq!(6, part2(&parse(include_str!("../test_input/day08.part2.6.txt"))));
    }

    #[test]
    fn part2_input() {
        assert_eq!(12833235391111, part2(&parse(include_str!("../input/2023/day8.txt"))));
    }
}
