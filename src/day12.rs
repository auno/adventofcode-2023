use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

type Input = Vec<(String, Vec<usize>)>;

#[aoc_generator(day12)]
fn parse(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            let (springs, damaged_groups) = line.split_once(' ').unwrap();

            (
                springs.to_string(),
                damaged_groups.split(',').map(str::parse).map(Result::unwrap).collect(),
            )
        })
        .collect()
}

#[aoc(day12, part1)]
fn part1(input: &Input) -> usize {
    input
        .iter()
        .map(|(springs, damaged_groups)| {
            let num_damaged: usize = damaged_groups.iter().sum();
            let num_known_damaged = springs.chars().filter(|&c| c == '#').count();

            springs
                .chars()
                .positions(|c| c == '?')
                .combinations(num_damaged - num_known_damaged)
                .filter(|fix| springs
                    .chars()
                    .enumerate()
                    .map(|(i, c)| if fix.contains(&i) { '#' } else { c })
                    .dedup_with_count()
                    .filter(|&(_, spring)| spring == '#')
                    .map(|(count, _)| count)
                    .eq(damaged_groups.iter().copied()))
                .count()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        assert_eq!(21, part1(&parse(include_str!("../test_input/day12.part1.21.txt"))));
    }

    #[test]
    fn part1_input() {
        assert_eq!(7670, part1(&parse(include_str!("../input/2023/day12.txt"))));
    }
}
