use std::cmp::{max, min};
use std::collections::{HashSet, VecDeque};
use std::str::FromStr;
use aoc_runner_derive::{aoc, aoc_generator};
use anyhow::{bail, Context, Error, Result};

#[derive(Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn step(&self, (j, i): (isize, isize)) -> (isize, isize) {
        match self {
            Direction::Up => (j - 1, i),
            Direction::Down => (j + 1, i),
            Direction::Left => (j, i - 1),
            Direction::Right => (j, i + 1),
        }
    }
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

type Input = Vec<(Direction, isize, String)>;

#[aoc_generator(day18)]
fn parse(input: &str) -> Result<Input> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split_ascii_whitespace();

            let direction = parts.next().context("Missing direction")?.parse()?;
            let amount = parts.next().context("Missing amount")?.parse()?;
            let color = parts.next().context("Missing color")?.parse()?;

            Ok((direction, amount, color))
        })
        .collect()
}

fn fill_outside(path: &HashSet<(isize, isize)>, (min_j, max_j, min_i, max_i): (isize, isize, isize, isize)) -> HashSet<(isize, isize)> {
    let mut outside = HashSet::from([(min_j, min_i)]);
    let mut queue = VecDeque::from([(min_j, min_i)]);

    while let Some(position) = queue.pop_front() {
        for neighbor in [
            (position.0 - 1, position.1 - 1), (position.0 - 1, position.1), (position.0 - 1, position.1 + 1),
            (position.0, position.1 - 1), (position.0, position.1 + 1),
            (position.0 + 1, position.1 - 1), (position.0 + 1, position.1), (position.0 + 1, position.1 + 1),
        ] {
            if !outside.contains(&neighbor) && !path.contains(&neighbor) && (min_j..=max_j).contains(&neighbor.0) && (min_i..=max_i).contains(&neighbor.1) {
                queue.push_back(neighbor);
                outside.insert(neighbor);
            }
        }
    }

    outside
}

#[aoc(day18, part1)]
fn part1(input: &Input) -> usize {
    let mut grid = HashSet::from([(0, 0)]);
    let mut position = (0, 0);

    for &(direction, amount, _) in input {
        for _ in 0..amount {
            position = direction.step(position);
            grid.insert(position);
        }
    }

    let bounds = grid
        .iter()
        .fold((0, 0, 0, 0), |(acc_min_j, acc_max_j, acc_min_i, acc_max_i), &(j, i)| (
            min(acc_min_j, j - 1),
            max(acc_max_j, j + 1),
            min(acc_min_i, i - 1),
            max(acc_max_i, i + 1),
        ));

    let outside = fill_outside(&grid, bounds);

    for j in (bounds.0)..=(bounds.1) {
        for i in (bounds.2)..=(bounds.3) {
            if !outside.contains(&(j, i)) {
                grid.insert((j, i));
            }
        }
    }

    grid.len()
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
}
