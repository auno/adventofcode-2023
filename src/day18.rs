use std::str::FromStr;
use aoc_runner_derive::{aoc, aoc_generator};
use anyhow::{bail, Context, Error, Result};

#[derive(Copy, Clone, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl FromStr for Direction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(
            match s {
                "U" => Direction::Up,
                "D" => Direction::Down,
                "L" => Direction::Left,
                "R" => Direction::Right,
                _ => bail!("Invalid direction: {s}"),
            }
        )
    }
}

type Input = (Vec<(Direction, isize)>, Vec<(Direction, isize)>);

#[aoc_generator(day18)]
fn parse(input: &str) -> Result<Input> {
    let part1: Vec<(Direction, isize)> = input
        .lines()
        .map(|line| {
            let mut parts = line.split_ascii_whitespace();

            let direction = parts.next().context("Missing direction")?.parse()?;
            let amount = parts.next().context("Missing amount")?.parse()?;

            Ok((direction, amount))
        })
        .collect::<Result<_>>()?;

    let part2: Vec<(Direction, isize)> = input
        .lines()
        .map(|line| {
            let hex_code = line.split_ascii_whitespace().nth(2).context(format!("Invalid input: {line}"))?;

            let direction = match &hex_code[7..8] {
                "0" => Direction::Right,
                "1" => Direction::Down,
                "2" => Direction::Left,
                "3" => Direction::Up,
                _ => bail!("Invalid hex code: {hex_code}"),
            };

            let amount = isize::from_str_radix(&hex_code[2..7], 16)?;

            eprintln!("direction: {:?}, amount: {amount}", direction);

            Ok((direction, amount))
        })
        .collect::<Result<_>>()?;

    Ok((part1, part2))
}

fn solve(input: &Vec<(Direction, isize)>) -> isize {
    let mut level = i32::MAX as isize;
    let mut area = 1;

    for &(direction, amount) in input {
        match direction {
            Direction::Up => {
                area += amount;
                level += amount;
            },
            Direction::Down => { level -= amount; },
            Direction::Right => { area += amount * level; }
            Direction::Left => { area -= amount * (level - 1); }
        }
    }

    area
}

#[aoc(day18, part1)]
fn part1((input, _): &Input) -> isize {
    solve(input)
}

#[aoc(day18, part2)]
fn part2((_, input): &Input) -> isize {
    solve(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        assert_eq!(62, part1(&parse(include_str!("../test_input/day18.part1.62.txt")).unwrap()));
    }

    #[test]
    fn part1_input() {
        assert_eq!(92758, part1(&parse(include_str!("../input/2023/day18.txt")).unwrap()));
    }

    #[test]
    fn part2_example1() {
        assert_eq!(952408144115, part2(&parse(include_str!("../test_input/day18.part2.952408144115.txt")).unwrap()));
    }

    #[test]
    fn part2_input() {
        assert_eq!(62762509300678, part2(&parse(include_str!("../input/2023/day18.txt")).unwrap()));
    }
}
