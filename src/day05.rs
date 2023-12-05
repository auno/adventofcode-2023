use std::collections::HashMap;
use aoc_runner_derive::{aoc, aoc_generator};
use anyhow::Result;
use itertools::Itertools;
use scan_fmt::scan_fmt;
use rayon::prelude::*;

type Map = HashMap<String, (String, Vec<(u64, u64, u64)>)>;

#[aoc_generator(day5)]
fn parse(input: &str) -> (Vec<u64>, Map) {
    let seeds = input
        .lines()
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(str::parse)
        .map(Result::unwrap)
        .collect::<Vec<u64>>();

    let mut map = HashMap::new();

    for chunk in input.split("\n\n").skip(1) {
        let (source, destination) = scan_fmt!(chunk.lines().next().unwrap(), "{}-to-{} map:", String, String).unwrap();

        let mappings: Vec<(u64, u64, u64)> = chunk
            .lines()
            .skip(1)
            .map(|line| scan_fmt!(line, "{d} {d} {d}", u64, u64, u64).unwrap())
            .collect();

        map.insert(source.to_string(), (destination.to_string(), mappings));
    }

    (seeds, map)
}

#[aoc(day5, part1)]
fn part1(input: &(Vec<u64>, Map)) -> u64 {
    let (seeds, map) = input;

    seeds
        .iter()
        .map(|seed| {
            let (mut current_type, mut current_number) = (&"seed".to_string(), *seed);

            while current_type != "location" {
                let (next_type, mappings) = map.get(current_type).unwrap();
                let mut next_number = current_number;

                for &(dstart, sstart, len) in mappings {
                    if (sstart..(sstart + len)).contains(&current_number) {
                        next_number = current_number - sstart + dstart;
                        break;
                    }
                }

                current_type = next_type;
                current_number = next_number;
            }

            current_number
        })
        .min()
        .unwrap()
}

#[aoc(day5, part2)]
fn part2(input: &(Vec<u64>, Map)) -> u64 {
    let (seeds, map) = input;

    seeds
        .iter()
        .tuples()
        .flat_map(|(start, len)| *start..(start + len))
        .par_bridge()
        .map(|seed| {
            let (mut current_type, mut current_number) = (&"seed".to_string(), seed);

            while current_type != "location" {
                let (next_type, mappings) = map.get(current_type).unwrap();
                let mut next_number = current_number;

                for &(dstart, sstart, len) in mappings {
                    if (sstart..(sstart + len)).contains(&current_number) {
                        next_number = current_number - sstart + dstart;
                        break;
                    }
                }

                current_type = next_type;
                current_number = next_number;
            }

            current_number
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        assert_eq!(35, part1(&parse(include_str!("../test_input/day05.part1.35.txt"))));
    }

    #[test]
    fn part1_input() {
        assert_eq!(346433842, part1(&parse(include_str!("../input/2023/day5.txt"))));
    }

    #[test]
    fn part2_example1() {
        assert_eq!(46, part2(&parse(include_str!("../test_input/day05.part2.46.txt"))));
    }

    #[test]
    fn part2_input() {
        assert_eq!(60294664, part2(&parse(include_str!("../input/2023/day5.txt"))));
    }
}
