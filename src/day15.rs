use std::str::FromStr;
use aoc_runner_derive::{aoc, aoc_generator};
use anyhow::{bail, Error, Result};

enum Step {
    Set(String, u32),
    Remove(String),
}

impl FromStr for Step {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((label, focal_length)) = s.split_once('=') else {
            if !s.ends_with('-') {
                bail!("Invalid step: {s}");
            }

            return Ok(Step::Remove(s[0..(s.len() - 1)].to_string()));
        };

        Ok(Step::Set(label.to_string(), focal_length.parse()?))
    }
}

type Input = (Vec<String>, Vec<Step>);

#[aoc_generator(day15)]
fn parse(input: &str) -> Result<Input> {
    let part1 = input
        .trim()
        .split(',')
        .map(|step| step.to_string())
        .collect();

    let part2 = input
        .trim()
        .split(',')
        .map(str::parse)
        .collect::<Result<_>>()?;

    Ok((part1, part2))
}

fn hash(s: &str) -> u32 {
    s.chars().fold(0, |acc, c| ((acc + c as u32) * 17) % 256)
}

#[aoc(day15, part1)]
fn part1((steps, _): &Input) -> u32 {
    steps
        .iter()
        .map(String::as_str)
        .map(hash)
        .sum()
}

#[aoc(day15, part2)]
fn part2((_, steps): &Input) -> u32 {
    let mut boxes: Vec<Vec<(&str, u32)>> = vec![vec![]; 256];

    for step in steps {
        match step {
            Step::Set(label, focal_length) => {
                let box_num = hash(label) as usize;

                if let Some(i) = boxes[box_num].iter().position(|(l, _)| l == label) {
                    boxes[box_num][i] = (label.as_str(), *focal_length);
                } else {
                    boxes[box_num].push((label.as_str(), *focal_length));
                }
            },
            Step::Remove(label) => {
                let box_num = hash(label) as usize;
                boxes[box_num].retain(|(l, _)| l != label);
            },
        }
    }

    boxes
        .iter()
        .enumerate()
        .flat_map(|(box_num, b)| b
            .iter()
            .enumerate()
            .map(move |(slot, (_, fl))| (box_num as u32 + 1) * (slot as u32 + 1) * fl)
        )
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        assert_eq!(1320, part1(&parse(include_str!("../test_input/day15.part1.1320.txt")).unwrap()));
    }

    #[test]
    fn part1_input() {
        assert_eq!(505379, part1(&parse(include_str!("../input/2023/day15.txt")).unwrap()));
    }

    #[test]
    fn part2_example1() {
        assert_eq!(145, part2(&parse(include_str!("../test_input/day15.part2.145.txt")).unwrap()));
    }

    #[test]
    fn part2_input() {
        assert_eq!(263211, part2(&parse(include_str!("../input/2023/day15.txt")).unwrap()));
    }
}
