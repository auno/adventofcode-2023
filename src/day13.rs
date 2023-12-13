use std::cmp::min;
use std::collections::HashMap;
use aoc_runner_derive::{aoc, aoc_generator};
use anyhow::{bail, Context, Error, Result};
use itertools::Itertools;

#[derive(Eq, PartialEq)]
enum Tile {
    Ash,
    Rock,
}

impl TryFrom<char> for Tile {
    type Error = Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(
            match value {
                '.' => Tile::Ash,
                '#' => Tile::Rock,
                _ => bail!("Invalid tile: {value}"),
            }
        )
    }
}

enum Orientation {
    Vertical,
    Horizontal,
}

type Input = Vec<(usize, usize, HashMap<(usize, usize), Tile>)>;

#[aoc_generator(day13)]
fn parse(input: &str) -> Result<Input> {
    input
        .split("\n\n")
        .map(|pattern| {
            let height = pattern.lines().count();
            let width = pattern.lines().next().context("Unexpected empty line")?.len();
            let pattern = pattern
                .lines()
                .enumerate()
                .flat_map(|(j, line)| {
                    line.chars()
                        .enumerate()
                        .map(move |(i, c)| Ok(((j, i), Tile::try_from(c)?)))
                })
                .collect::<Result<HashMap<(usize, usize), Tile>>>();

            Ok((height, width, pattern?))
        })
        .collect()
}

fn find_mirror(height: usize, width: usize, pattern: &HashMap<(usize, usize), Tile>) -> Option<(Orientation, usize)> {
    for k in 1..(width) {
        let reflection_width = min(k, width - k);

        if (0..height)
            .cartesian_product(0..reflection_width)
            .all(|(j, l)| pattern.get(&(j, k - l - 1)) == pattern.get(&(j, k + l)))
        {
            return Some((Orientation::Vertical, k));
        }
    }

    for k in 1..(height) {
        let reflection_height = min(k, height - k);

        if (0..width)
            .cartesian_product(0..reflection_height)
            .all(|(i, l)| pattern.get(&(k - l - 1, i)) == pattern.get(&(k + l, i)))
        {
            return Some((Orientation::Horizontal, k));
        }
    }

    None
}

#[aoc(day13, part1)]
fn part1(input: &Input) -> usize {
    input
        .iter()
        .map(|(height, width, pattern)| {
            match find_mirror(*height, *width, pattern) {
                Some((Orientation::Vertical, k)) => k,
                Some((Orientation::Horizontal, k)) => k * 100,
                None => panic!("No solution found"),
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        assert_eq!(405, part1(&parse(include_str!("../test_input/day13.part1.405.txt")).unwrap()));
    }

    #[test]
    fn part1_input() {
        assert_eq!(34889, part1(&parse(include_str!("../input/2023/day13.txt")).unwrap()));
    }
}
