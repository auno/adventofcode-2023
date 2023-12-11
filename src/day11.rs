use std::collections::HashSet;
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day11)]
fn parse(input: &str) -> Vec<(usize, usize)> {
    input
        .lines()
        .enumerate()
        .flat_map(|(j, line)| line.chars().enumerate().map(move |(i, c)| ((j, i), c)))
        .filter(|(_, c)| *c == '#')
        .map(|(p, _)| p)
        .collect()
}

#[aoc(day11, part1)]
fn part1(galaxies: &[(usize, usize)]) -> usize {
    let lines: HashSet<_> = galaxies.iter().map(|&(j, _)| j).collect();
    let columns: HashSet<_> = galaxies.iter().map(|&(_, i)| i).collect();

    galaxies
        .iter()
        .map(|&(j, i)| (
            j + (j - lines.iter().filter(|&&l| l < j).count()),
            i + (i - columns.iter().filter(|&&c| c < i).count()),
        ))
        .tuple_combinations::<(_, _)>()
        .map(|(a, b)| a.0.abs_diff(b.0) + a.1.abs_diff(b.1))
        .sum::<usize>()
}

#[aoc(day11, part2)]
fn part2(galaxies: &[(usize, usize)]) -> usize {
    let lines: HashSet<_> = galaxies.iter().map(|&(j, _)| j).collect();
    let columns: HashSet<_> = galaxies.iter().map(|&(_, i)| i).collect();

    galaxies
        .iter()
        .inspect(|x| eprintln!("-- > {:?}", x))
        .map(|&(j, i)| (
            j + 999999 * (j - lines.iter().filter(|&&l| l < j).count()),
            i + 999999 * (i - columns.iter().filter(|&&c| c < i).count()),
        ))
        .tuple_combinations::<(_, _)>()
        .map(|(a, b)| a.0.abs_diff(b.0) + a.1.abs_diff(b.1))
        .sum::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        assert_eq!(374, part1(&parse(include_str!("../test_input/day11.part1.374.txt"))));
    }

    #[test]
    fn part1_input() {
        assert_eq!(10885634, part1(&parse(include_str!("../input/2023/day11.txt"))));
    }

    #[test]
    fn part2_input() {
        assert_eq!(707505470642, part2(&parse(include_str!("../input/2023/day11.txt"))));
    }
}
