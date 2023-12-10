use std::collections::HashMap;
use std::iter::successors;
use aoc_runner_derive::{aoc, aoc_generator};
use anyhow::{bail, Context, Error, Result};
use itertools::Itertools;

#[derive(Copy, Clone, Debug)]
enum Pipe {
    Vertical,
    Horizontal,
    BendNorthEast,
    BendNorthWest,
    BendSouthEast,
    BendSouthWest,
}

impl TryFrom<char> for Pipe {
    type Error = Error;

    fn try_from(value: char) -> Result<Self> {
        Ok(
            match value {
                '|' => Pipe::Vertical,
                '-' => Pipe::Horizontal,
                'L' => Pipe::BendNorthEast,
                'J' => Pipe::BendNorthWest,
                'F' => Pipe::BendSouthEast,
                '7' => Pipe::BendSouthWest,
                _ => bail!("Invalid pipe: {value}"),
            }
        )
    }
}

type Input = ((i32, i32), HashMap<(i32, i32), Pipe>);

fn determine_starting_pipe(starting_position: &(i32, i32), map: &HashMap<(i32, i32), Pipe>) -> Result<Pipe, Error> {
    match (
        map.get(&(starting_position.0, starting_position.1 - 1)),
        map.get(&(starting_position.0, starting_position.1 + 1)),
        map.get(&(starting_position.0 - 1, starting_position.1)),
        map.get(&(starting_position.0 + 1, starting_position.1)),
    ) {
        (Some(Pipe::Horizontal | Pipe::BendNorthEast | Pipe::BendSouthEast), Some(Pipe::Horizontal | Pipe::BendNorthWest | Pipe::BendSouthWest), _, _) => Some(Pipe::Horizontal),
        (_, _, Some(Pipe::Vertical | Pipe::BendSouthEast | Pipe::BendSouthWest), Some(Pipe::Vertical | Pipe::BendNorthEast | Pipe::BendNorthWest)) => Some(Pipe::Vertical),
        (_, Some(Pipe::Horizontal | Pipe::BendNorthWest | Pipe::BendSouthWest), Some(Pipe::Vertical | Pipe::BendSouthEast | Pipe::BendSouthWest), _) => Some(Pipe::BendNorthEast),
        (Some(Pipe::Horizontal | Pipe::BendNorthEast | Pipe::BendSouthEast), _, Some(Pipe::Vertical | Pipe::BendSouthEast | Pipe::BendSouthWest), _) => Some(Pipe::BendNorthWest),
        (_, Some(Pipe::Horizontal | Pipe::BendNorthWest | Pipe::BendSouthWest), _, Some(Pipe::Vertical | Pipe::BendNorthEast | Pipe::BendNorthWest)) => Some(Pipe::BendSouthEast),
        (Some(Pipe::Horizontal | Pipe::BendNorthEast | Pipe::BendSouthEast), _, _, Some(Pipe::Vertical | Pipe::BendNorthEast | Pipe::BendNorthWest)) => Some(Pipe::BendSouthWest),
        _ => None,
    }.context("No starting position found")
}

#[aoc_generator(day10)]
fn parse(input: &str) -> Result<Input> {
    let mut map = HashMap::new();
    let mut starting_position = None;

    for (j, line) in input.lines().enumerate() {
        for (i, c) in line.chars().enumerate() {
            let position = (j as i32, i as i32);
            match c {
                '.' => {},
                'S' => { starting_position = Some(position); },
                _ => { map.insert(position, c.try_into()?); },
            }
        }
    }

    let starting_position = starting_position.context("No starting position found")?;
    let starting_pipe = determine_starting_pipe(&starting_position, &map)?;
    map.insert(starting_position, starting_pipe);

    Ok((starting_position, map))
}

fn step(pos: (i32, i32), ppos: Option<(i32, i32)>, map: &HashMap<(i32, i32), Pipe>) -> ((i32, i32), Option<(i32, i32)>) {
    match (map.get(&pos), ppos) {
        (Some(Pipe::Vertical | Pipe::BendSouthEast | Pipe::BendSouthWest), None) => ((pos.0 + 1, pos.1), Some(pos)),
        (Some(Pipe::Horizontal | Pipe::BendNorthWest), None) => ((pos.0, pos.1 + 1), Some(pos)),
        (Some(Pipe::BendNorthEast), None) => ((pos.0, pos.1 - 1), Some(pos)),
        (Some(Pipe::Vertical | Pipe::BendSouthEast | Pipe::BendSouthWest), Some((pj, _))) if pj != pos.0 + 1 => ((pos.0 + 1, pos.1), Some(pos)),
        (Some(Pipe::Vertical | Pipe::BendNorthEast | Pipe::BendNorthWest), Some((pj, _))) if pj != pos.0 - 1 => ((pos.0 - 1, pos.1), Some(pos)),
        (Some(Pipe::Horizontal | Pipe::BendNorthWest | Pipe::BendSouthWest), Some((_, pi))) if pi != pos.1 - 1 => ((pos.0, pos.1 - 1), Some(pos)),
        (Some(Pipe::Horizontal | Pipe::BendNorthEast | Pipe::BendSouthEast), Some((_, pi))) if pi != pos.1 + 1 => ((pos.0, pos.1 + 1), Some(pos)),
        (pipe, _) => panic!("Nowhere to go from {:?}, coming from {:?}: {:?}", pos, ppos, pipe),
    }
}

fn find_path(starting_position: (i32, i32), map: &HashMap<(i32, i32), Pipe>) -> Vec<(i32, i32)> {
    successors(Some((starting_position, None)), |(position, previous_position)| {
        let (position, previous_position) = step(*position, *previous_position, map);

        if position == starting_position {
            return None;
        }

        Some((position, previous_position))
    }).map(|(pos, _)| pos).collect_vec()
}

#[aoc(day10, part1)]
fn part1(input: &Input) -> usize {
    let (starting_position, map) = input.clone();
    let path = find_path(starting_position, &map);

    path.len() / 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        assert_eq!(4, part1(&parse(include_str!("../test_input/day10.part1.4.txt")).unwrap()));
    }

    #[test]
    fn part1_example2() {
        assert_eq!(8, part1(&parse(include_str!("../test_input/day10.part1.8.txt")).unwrap()));
    }

    #[test]
    fn part1_input() {
        assert_eq!(6882, part1(&parse(include_str!("../input/2023/day10.txt")).unwrap()));
    }
}
