use std::collections::HashMap;
use std::usize;
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::{Itertools, repeat_n};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

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

fn matches(springs: &str, damaged_groups: &[usize]) -> usize {
    matches_cached(&mut HashMap::new(), springs, damaged_groups)
}

fn matches_cached(cache: &mut HashMap<(usize, usize), usize>, springs: &str, damaged_groups: &[usize]) -> usize {
    if let Some(&count) = cache.get(&(springs.len(), damaged_groups.len())) {
        return count;
    }

    let mut count = 0;

    if springs.is_empty() && damaged_groups.is_empty() {
        count += 1;
    }

    if !springs.is_empty() {
        if let Some(&n) = damaged_groups.first() {
            if springs.len() >= n && springs[0..n].chars().all(|c| c != '.') {
                if springs.len() == n {
                    count += matches_cached(cache, "", &damaged_groups[1..]);
                } else if &springs[n..(n + 1)] != "#" {
                    count += matches_cached(cache, &springs[(n + 1)..], &damaged_groups[1..]);
                }
            }
        }

        if &springs[0..1] != "#" {
            count += matches_cached(cache, &springs[1..], damaged_groups)
        }
    }

    cache.insert((springs.len(), damaged_groups.len()), count);

    count
}

#[aoc(day12, part1)]
fn part1(input: &Input) -> usize {
    input
        .par_iter()
        .map(|(springs, damaged_groups)| matches(springs, damaged_groups))
        .sum()
}

#[aoc(day12, part2)]
fn part2(input: &Input) -> usize {
    input
        .par_iter()
        .map(|(springs, damaged_groups)| {
            let springs = repeat_n(springs, 5).join("?");
            let damaged_groups = damaged_groups.iter().cycle().take(damaged_groups.len() * 5).copied().collect_vec();

            matches(&springs, &damaged_groups)
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

    #[test]
    fn part2_example1() {
        assert_eq!(525152, part2(&parse(include_str!("../test_input/day12.part2.525152.txt"))));
    }

    #[test]
    fn matches_example1() {
        let springs = "???.###????.###????.###????.###????.###";
        let damaged_groups = [ 1, 1, 3, 1, 1, 3, 1, 1, 3, 1, 1, 3, 1, 1, 3 ];
        assert_eq!(1, matches(springs, &damaged_groups));
    }

    #[test]
    fn matches_example1_foo() {
        let springs = "???.###";
        let damaged_groups = [ 1, 1, 3 ];
        assert_eq!(1, matches(springs, &damaged_groups));
    }

    #[test]
    fn matches_example2() {
        let springs = ".??..??...?##.?.??..??...?##.?.??..??...?##.?.??..??...?##.?.??..??...?##.";
        let damaged_groups = [ 1, 1, 3, 1, 1, 3, 1, 1, 3, 1, 1, 3, 1, 1, 3 ];
        assert_eq!(16384, matches(springs, &damaged_groups));
    }

    #[test]
    fn matches_example3() {
        let springs = "?#?#?#?#?#?#?#???#?#?#?#?#?#?#???#?#?#?#?#?#?#???#?#?#?#?#?#?#???#?#?#?#?#?#?#?";
        let damaged_groups = [ 1, 3, 1, 6, 1, 3, 1, 6, 1, 3, 1, 6, 1, 3, 1, 6, 1, 3, 1, 6 ];
        assert_eq!(1, matches(springs, &damaged_groups));
    }

    #[test]
    fn matches_example4() {
        let springs = "????.#...#...?????.#...#...?????.#...#...?????.#...#...?????.#...#...";
        let damaged_groups = [ 4, 1, 1, 4, 1, 1, 4, 1, 1, 4, 1, 1, 4, 1, 1 ];
        assert_eq!(16, matches(springs, &damaged_groups));
    }

    #[test]
    fn matches_example5() {
        let springs = "????.######..#####.?????.######..#####.?????.######..#####.?????.######..#####.?????.######..#####.";
        let damaged_groups = [ 1, 6, 5, 1, 6, 5, 1, 6, 5, 1, 6, 5, 1, 6, 5 ];
        assert_eq!(2500, matches(springs, &damaged_groups));
    }

    #[test]
    fn matches_example6() {
        let springs = "?###??????????###??????????###??????????###??????????###????????";
        let damaged_groups = [ 3, 2, 1, 3, 2, 1, 3, 2, 1, 3, 2, 1, 3, 2, 1 ];
        assert_eq!(506250, matches(springs, &damaged_groups));
    }

    #[test]
    fn part2_input() {
        assert_eq!(157383940585037, part2(&parse(include_str!("../input/2023/day12.txt"))));
    }
}
