use std::collections::HashMap;
use aoc_runner_derive::{aoc, aoc_generator};
use anyhow::{bail, Context, Error, Result};

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

    Ok((starting_position.unwrap(), map))
}

#[aoc(day10, part1)]
fn part1(input: &Input) -> Result<usize> {
    let (spos, mut map) = input.clone();

    let starting_pipe = match (map.get(&(spos.0, spos.1 - 1)), map.get(&(spos.0, spos.1 + 1)), map.get(&(spos.0 - 1, spos.1)), map.get(&(spos.0 + 1, spos.1))) {
        (Some(Pipe::Horizontal | Pipe::BendNorthEast | Pipe::BendSouthEast), Some(Pipe::Horizontal | Pipe::BendNorthWest | Pipe::BendSouthWest), _, _) => Some(Pipe::Horizontal),
        (_, _, Some(Pipe::Vertical | Pipe::BendSouthEast | Pipe::BendSouthWest), Some(Pipe::Vertical | Pipe::BendNorthEast | Pipe::BendNorthWest)) => Some(Pipe::Vertical),
        (_, Some(Pipe::Horizontal | Pipe::BendNorthWest | Pipe::BendSouthWest), Some(Pipe::Vertical | Pipe::BendSouthEast | Pipe::BendSouthWest), _) => Some(Pipe::BendNorthEast),
        (Some(Pipe::Horizontal | Pipe::BendNorthEast | Pipe::BendSouthEast), _, Some(Pipe::Vertical | Pipe::BendSouthEast | Pipe::BendSouthWest), _) => Some(Pipe::BendNorthWest),
        (_, Some(Pipe::Horizontal | Pipe::BendNorthWest | Pipe::BendSouthWest), _, Some(Pipe::Vertical | Pipe::BendNorthEast | Pipe::BendNorthWest)) => Some(Pipe::BendSouthEast),
        (Some(Pipe::Horizontal | Pipe::BendNorthEast | Pipe::BendSouthEast), _, _, Some(Pipe::Vertical | Pipe::BendNorthEast | Pipe::BendNorthWest)) => Some(Pipe::BendSouthWest),
        _ => None,
    }.context("No starting position found")?;

    map.insert(spos, starting_pipe);

    let mut ppos = None;
    let mut pos = spos;
    let mut distance = 0usize;

    loop {
        (pos, ppos) = match (map.get(&pos), ppos) {
            (Some(Pipe::Vertical | Pipe::BendSouthEast | Pipe::BendSouthWest), None) => ((pos.0 + 1, pos.1), Some(pos)),
            (Some(Pipe::Horizontal | Pipe::BendNorthWest), None) => ((pos.0, pos.1 + 1), Some(pos)),
            (Some(Pipe::BendNorthEast), None) => ((pos.0, pos.1 - 1), Some(pos)),
            (Some(Pipe::Vertical | Pipe::BendSouthEast | Pipe::BendSouthWest), Some((pj, _))) if pj != pos.0 + 1 => ((pos.0 + 1, pos.1), Some(pos)),
            (Some(Pipe::Vertical | Pipe::BendNorthEast | Pipe::BendNorthWest), Some((pj, _))) if pj != pos.0 - 1 => ((pos.0 - 1, pos.1), Some(pos)),
            (Some(Pipe::Horizontal | Pipe::BendNorthWest | Pipe::BendSouthWest), Some((_, pi))) if pi != pos.1 - 1 => ((pos.0, pos.1 - 1), Some(pos)),
            (Some(Pipe::Horizontal | Pipe::BendNorthEast | Pipe::BendSouthEast), Some((_, pi))) if pi != pos.1 + 1 => ((pos.0, pos.1 + 1), Some(pos)),
            (pipe, _) => bail!("Nowhere to go from {:?}, coming from {:?}: {:?}", pos, ppos, pipe),
        };

        distance += 1;

        if pos == spos {
            break;
        }
    }

    Ok(distance / 2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        assert_eq!(4, part1(&parse(include_str!("../test_input/day10.part1.4.txt")).unwrap()).unwrap());
    }

    #[test]
    fn part1_example2() {
        assert_eq!(8, part1(&parse(include_str!("../test_input/day10.part1.8.txt")).unwrap()).unwrap());
    }

    #[test]
    fn part1_input() {
        assert_eq!(6882, part1(&parse(include_str!("../input/2023/day10.txt")).unwrap()).unwrap());
    }
}
