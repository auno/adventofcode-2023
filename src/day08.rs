use std::collections::HashMap;
use aoc_runner_derive::{aoc, aoc_generator};
use scan_fmt::scan_fmt;
use anyhow::{bail, Error};

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

#[aoc(day8, part1)]
fn part1(input: &Input) -> usize {
    let (instructions, map) = input;

    let mut location = &"AAA".to_string();
    let mut distance = 0;

    for instruction in instructions.iter().cycle() {
        if location == "ZZZ" {
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
}
