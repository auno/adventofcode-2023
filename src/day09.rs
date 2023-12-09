use aoc_runner_derive::{aoc, aoc_generator};
use anyhow::{Context, Result};
use itertools::Itertools;

#[aoc_generator(day9)]
fn parse(input: &str) -> Result<Vec<Vec<i32>>> {
    input
        .lines()
        .map(|line| line
            .split_whitespace()
            .map(|n| n.parse().context(format!("Could not parse number: {n}")))
            .collect()
        )
        .collect()
}

fn get_next(numbers: &[i32]) -> i32 {
    if numbers.len() < 2 {
        panic!("Too few numbers: {:?}", numbers);
    }

    let diffs = numbers
        .iter().tuple_windows()
        .map(|(a, b)| b - a)
        .collect_vec();

    if diffs.iter().all_equal() {
        return numbers.last().unwrap() + diffs[0]
    }

    numbers.last().unwrap() + get_next(&diffs)
}

#[aoc(day9, part1)]
fn part1(input: &[Vec<i32>]) -> i32 {
    input
        .iter()
        .map(Vec::as_slice)
        .map(get_next)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        assert_eq!(114, part1(&parse(include_str!("../test_input/day09.part1.114.txt")).unwrap()));
    }

    #[test]
    fn part1_input() {
        assert_eq!(1916822650, part1(&parse(include_str!("../input/2023/day9.txt")).unwrap()));
    }
}
