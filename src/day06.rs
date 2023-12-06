use std::iter::zip;
use aoc_runner_derive::{aoc, aoc_generator};
use anyhow::{Context, Result};

#[aoc_generator(day6)]
fn parse(input: &str) -> Result<Vec<(u32, u32)>> {
    let mut lines = input.lines();

    let times = lines
        .next()
        .context("Input missing first line")?
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse::<u32>().context(format!("Could not parse number: {s}")));
    let distances = lines
        .next()
        .context("Input missing second line")?
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse::<u32>().context(format!("Could not parse number: {s}")));

    zip(times, distances).map(|(t, d)| Ok((t?, d?))).collect()
}

#[aoc(day6, part1)]
fn part1(input: &[(u32, u32)]) -> usize {
    input
        .iter()
        .map(|&(time, distance)| {
            (0..time)
                .map(|c| (time - c) * c)
                .filter(|&d| d > distance)
                .count()
        })
        .product()
}

#[aoc(day6, part2)]
fn part2(input: &[(u32, u32)]) -> usize {
    let (time, distance) = input
        .iter()
        .map(|(t, d)| (t.to_string(), d.to_string()))
        .fold((String::new(), String::new()),  |(acct, accd), (t, d)| (acct + &t, accd + &d));

    let (time, distance) = (time.parse::<u64>().unwrap(), distance.parse::<u64>().unwrap());

    (0..time)
        .map(|c| (time - c) * c)
        .filter(|&d| d > distance)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        assert_eq!(288, part1(&parse(include_str!("../test_input/day06.part1.288.txt")).unwrap()));
    }

    #[test]
    fn part1_input() {
        assert_eq!(503424, part1(&parse(include_str!("../input/2023/day6.txt")).unwrap()));
    }
    #[test]
    fn part2_example1() {
        assert_eq!(71503, part2(&parse(include_str!("../test_input/day06.part2.71503.txt")).unwrap()));
    }

    #[test]
    fn part2_input() {
        assert_eq!(32607562, part2(&parse(include_str!("../input/2023/day6.txt")).unwrap()));
    }
}
