use std::iter::zip;
use aoc_runner_derive::{aoc, aoc_generator};
use anyhow::{Context, Result};

type Input = (Vec<(u64, u64)>, (u64, u64));

#[aoc_generator(day6)]
fn parse(input: &str) -> Result<Input> {
    let mut lines = input.lines();

    let a_times = lines
        .next()
        .context("Input missing first line")?
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse::<u64>().context(format!("Could not parse number: {s}")));
    let a_distances = lines
        .next()
        .context("Input missing second line")?
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse::<u64>().context(format!("Could not parse number: {s}")));

    let mut lines = input.lines();

    let b_time = lines
        .next()
        .context("Input missing first line")?
        .replace(' ', "")
        .split_once(':')
        .map(|(_, s)| s.parse::<u64>().context(format!("Could not parse number: {s}")))
        .context("Could not parse time")??;

    let b_distance = lines
        .next()
        .context("Input missing first line")?
        .replace(' ', "")
        .split_once(':')
        .map(|(_, s)| s.parse::<u64>().context(format!("Could not parse number: {s}")))
        .context("Could not parse distance")??;

    Ok((
        zip(a_times, a_distances).map(|(t, d)| Ok((t?, d?))).collect::<Result<Vec<(u64, u64)>>>()?,
        (b_time, b_distance)
    ))

}

fn count_wins(time: f64, distance: f64) -> usize {
    let r1 = ((time + (time * time - 4.0 * distance).sqrt()) / 2.0).ceil() as usize - 1;
    let r2 = ((time - (time * time - 4.0 * distance).sqrt()) / 2.0).floor() as usize + 1;

    r1 - r2 + 1
}

#[aoc(day6, part1)]
fn part1(input: &Input) -> usize {
    let (input, _) = input;

    input
        .iter()
        .map(|&(time, distance)| count_wins(time as f64, distance as f64))
        .product()
}

#[aoc(day6, part2)]
fn part2(input: &Input) -> usize {
    let (_, (time, distance)) = *input;
    count_wins(time as f64, distance as f64)
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
