use std::collections::HashMap;
use aoc_runner_derive::{aoc, aoc_generator};
use anyhow::Result;
use itertools::Itertools;
use scan_fmt::scan_fmt;

type Map = HashMap<String, (String, Vec<(i64, i64, i64)>)>;

#[aoc_generator(day5)]
fn parse(input: &str) -> (Vec<i64>, Map) {
    let seeds = input
        .lines()
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(str::parse)
        .map(Result::unwrap)
        .collect::<Vec<i64>>();

    let mut map = HashMap::new();

    for chunk in input.split("\n\n").skip(1) {
        let (source, destination) = scan_fmt!(chunk.lines().next().unwrap(), "{}-to-{} map:", String, String).unwrap();

        let mappings: Vec<(i64, i64, i64)> = chunk
            .lines()
            .skip(1)
            .map(|line| scan_fmt!(line, "{d} {d} {d}", i64, i64, i64).unwrap())
            .map(|(destination_start, source_start, len)| (
                source_start,
                source_start + len,
                destination_start - source_start
            ))
            .collect();

        map.insert(source.to_string(), (destination.to_string(), mappings));
    }

    (seeds, map)
}

#[aoc(day5, part1)]
fn part1(input: &(Vec<i64>, Map)) -> i64 {
    let (seeds, map) = input;

    seeds
        .iter()
        .map(|seed| {
            let (mut current_type, mut current_number) = (&"seed".to_string(), *seed);

            while current_type != "location" {
                let (next_type, mappings) = map.get(current_type).unwrap();
                let mut next_number = current_number;

                for &(start,end, diff) in mappings {
                    if (start..end).contains(&current_number) {
                        next_number = current_number + diff;
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
fn part2(input: &(Vec<i64>, Map)) -> i64 {
    let (seeds, map) = input;

    let mut mappings = seeds
        .iter()
        .tuples()
        .map(|(start, len)| (*start, start + len, 0))
        .collect_vec();

    let mut category = &"seed".to_string();

    while category != "location" {
        let (next_category, next_category_mappings) = map.get(category).unwrap();

        mappings = mappings
            .iter()
            .flat_map(|&mapping| {
                next_category_mappings
                    .iter()
                    .fold(vec![mapping], |acc, &(ncm_start, ncm_end, _)| {
                        acc.iter()
                            .flat_map(|&(start, end, diff)| {
                                if (start + diff >= ncm_start) && (end + diff <= ncm_end) {
                                    return vec![(start, end, diff)];
                                } else if (start + diff >= ncm_start) && (start + diff < ncm_end) {
                                    return vec![
                                        (start, ncm_end - diff, diff),
                                        (ncm_end - diff, end, diff)
                                    ];
                                } else if (end + diff > ncm_start) && (end + diff < ncm_end) {
                                    return vec![
                                        (start, ncm_start - diff, diff),
                                        (ncm_start - diff, end, diff)
                                    ];
                                }

                                vec![(start, end, diff)]
                            })
                            .collect()
                    })
                    .iter()
                    .map(|&(start, end, diff)| {
                        next_category_mappings
                            .iter()
                            .find(|&&(ncm_start, ncm_end, _)| start + diff >= ncm_start && end + diff <= ncm_end)
                            .map(|&(_, _, ncm_diff)| (start, end, diff + ncm_diff))
                            .unwrap_or((start, end, diff))
                    })
                    .collect_vec()
            })
            .collect_vec();

        category = next_category;
    }

    mappings
        .iter()
        .min_by_key(|(start, _, diff)| start + diff)
        .map(|(start, _, diff)| start + diff)
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
