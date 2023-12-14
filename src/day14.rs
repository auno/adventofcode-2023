use std::collections::HashMap;
use aoc_runner_derive::{aoc, aoc_generator};
use anyhow::{bail, Context, Error, Result};
use itertools::Itertools;

#[derive(Eq, PartialEq, Copy, Clone, Hash)]
enum Rock {
    Cubed,
    Round,
}

impl TryFrom<char> for Rock {
    type Error = Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(
            match value {
                '#' => Rock::Cubed,
                'O' => Rock::Round,
                _ => bail!("Invalid rock: {value}"),
            }
        )
    }
}

type Input = (usize, usize, HashMap<(usize, usize), Rock>);

#[aoc_generator(day14)]
fn parse(input: &str) -> Result<Input> {
        let height = input.lines().count();
        let width = input.lines().next().context("Unexpected empty line")?.len();
        let map = input
            .lines()
            .enumerate()
            .flat_map(|(j, line)| {
                line.chars()
                    .enumerate()
                    .filter(|&(_, c)| c != '.')
                    .map(move |(i, c)| Ok(((j, i), Rock::try_from(c)?)))
            })
            .collect::<Result<HashMap<(usize, usize), Rock>>>();

        Ok((height, width, map?))
}

#[aoc(day14, part1)]
fn part1(input: &Input) -> usize {
    let (height, _width, mut map) = input.clone();
    let round_keys = map.keys().copied().sorted().filter(|k| map.get(k) == Some(&Rock::Round)).collect_vec();

    for (j, i) in round_keys {
        if let Some(j1) = (0..j).rev().take_while(|&candidate| !map.contains_key(&(candidate, i))).last() {
            map.remove(&(j, i));
            map.insert((j1, i), Rock::Round);
        }
    }

    map.iter()
        .filter(|&(_, &rock)| rock == Rock::Round)
        .map(|((j, _), _)| height - j)
        .sum()
}

#[aoc(day14, part2)]
fn part2(input: &Input) -> usize {
    let (height, width, mut map) = input.clone();
    let mut hashes = HashMap::new();
    let mut loads = vec![];

    for iteration in 0usize.. {
        let hash = fxhash::hash32(&map.keys().copied().filter(|k| map.get(k) == Some(&Rock::Round)).sorted().collect_vec());

        if let Some(cycle_start) = hashes.get(&hash) {
            let cycle_length = iteration - cycle_start;
            let remainder = (1000000000 - cycle_start) % cycle_length;

            return loads[cycle_start + remainder];
        }

        let load: usize = map.iter().filter(|&(_, &rock)| rock == Rock::Round).map(|((j, _), _)| height - j).sum();
        hashes.insert(hash, iteration);
        loads.push(load);


        let round_keys = map.keys().copied().sorted().filter(|k| map.get(k) == Some(&Rock::Round)).collect_vec();
        for (j, i) in round_keys {
            if let Some(j1) = (0..j).rev().take_while(|&candidate| !map.contains_key(&(candidate, i))).last() {
                map.remove(&(j, i));
                map.insert((j1, i), Rock::Round);
            }
        }

        let round_keys = map.keys().copied().filter(|k| map.get(k) == Some(&Rock::Round)).sorted_by_key(|&(j, i)| (i, j)).collect_vec();
        for (j, i) in round_keys {
            if let Some(i1) = (0..i).rev().take_while(|&candidate| !map.contains_key(&(j, candidate))).last() {
                map.remove(&(j, i));
                map.insert((j, i1), Rock::Round);
            }
        }

        let round_keys = map.keys().copied().filter(|k| map.get(k) == Some(&Rock::Round)).sorted().rev().collect_vec();
        for (j, i) in round_keys {
            if let Some(j1) = ((j + 1)..height).take_while(|&candidate| !map.contains_key(&(candidate, i))).last() {
                map.remove(&(j, i));
                map.insert((j1, i), Rock::Round);
            }
        }

        let round_keys = map.keys().copied().filter(|k| map.get(k) == Some(&Rock::Round)).sorted_by_key(|&(j, i)| (i, j)).rev().collect_vec();
        for (j, i) in round_keys {
            if let Some(i1) = ((i + 1)..width).take_while(|&candidate| !map.contains_key(&(j, candidate))).last() {
                map.remove(&(j, i));
                map.insert((j, i1), Rock::Round);
            }
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        assert_eq!(136, part1(&parse(include_str!("../test_input/day14.part1.136.txt")).unwrap()));
    }

    #[test]
    fn part1_input() {
        assert_eq!(108955, part1(&parse(include_str!("../input/2023/day14.txt")).unwrap()));
    }

    #[test]
    fn part2_example1() {
        assert_eq!(64, part2(&parse(include_str!("../test_input/day14.part2.64.txt")).unwrap()));
    }

    #[test]
    fn part2_input() {
        assert_eq!(106689, part2(&parse(include_str!("../input/2023/day14.txt")).unwrap()));
    }
}
