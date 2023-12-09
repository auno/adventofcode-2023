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

fn get_next(numbers: &[i32]) -> Option<i32> {
    if numbers.len() < 2 {
        panic!("Too few numbers: {:?}", numbers);
    }

    let diffs = numbers
        .iter().tuple_windows()
        .map(|(a, b)| b - a)
        .collect_vec();

    if diffs.iter().all_equal() {
        return Some(numbers.last()? + diffs.first()?)
    }

    Some(numbers.last()? + get_next(&diffs)?)
}

#[aoc(day9, part1)]
fn part1(input: &[Vec<i32>]) -> Option<i32> {
    input
        .iter()
        .map(Vec::as_slice)
        .map(get_next)
        .sum()
}

fn get_previous(numbers: &[i32]) -> Option<i32> {
    let diffs = numbers
        .iter().tuple_windows()
        .map(|(a, b)| b - a)
        .collect_vec();

    if diffs.iter().all_equal() {
        return Some(numbers.first()? - diffs.first()?)
    }

    Some(numbers.first()? - get_previous(&diffs)?)
}

#[aoc(day9, part2)]
fn part2(input: &[Vec<i32>]) -> Option<i32> {
    input
        .iter()
        .map(Vec::as_slice)
        .map(get_previous)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        assert_eq!(114, part1(&parse(include_str!("../test_input/day09.part1.114.txt")).unwrap()).unwrap());
    }

    #[test]
    fn part1_input() {
        assert_eq!(1916822650, part1(&parse(include_str!("../input/2023/day9.txt")).unwrap()).unwrap());
    }

    #[test]
    fn part2_example1() {
        assert_eq!(2, part2(&parse(include_str!("../test_input/day09.part2.2.txt")).unwrap()).unwrap());
    }

    #[test]
    fn part2_input() {
        assert_eq!(966, part2(&parse(include_str!("../input/2023/day9.txt")).unwrap()).unwrap());
    }
}
