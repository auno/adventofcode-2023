use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::hash::Hash;
use aoc_runner_derive::{aoc, aoc_generator};
use anyhow::{Context, Result};

type Input = (usize, usize, (usize, usize), HashSet<(usize, usize)>);

#[aoc_generator(day21)]
fn parse(input: &str) -> Result<Input> {
    let height = input.lines().count();
    let width = input.lines().next().context("Unexpected empty line")?.len();
    let mut rocks = HashSet::new();
    let mut starting_position = None;

    for (i, c) in input.chars().filter(|c| !c.is_ascii_whitespace()).enumerate() {
        let position = ((i / width) + 1, (i % width) + 1);

        if c == 'S' {
            starting_position = Some(position);
        }

        if c == '#' || c == 'S' {
            rocks.insert(position);
        }
    }

    Ok((height, width, starting_position.context("Starting position not found")?, rocks))
}

type State = (usize, usize);

fn distances<SearchState: Ord + Hash + Copy, IntoNeighborIterator: IntoIterator<Item=(SearchState, usize)>>(
    source: SearchState,
    neighbors: impl Fn(SearchState) -> IntoNeighborIterator,
) -> impl IntoIterator<Item=(SearchState, usize)> {
    let mut distances: HashMap<SearchState, usize> = HashMap::new();
    let mut queue: BinaryHeap<(Reverse<usize>, SearchState)> = BinaryHeap::new();

    distances.insert(source, 0);
    queue.push((Reverse(0), source));

    while let Some((Reverse(current_distance), state)) = queue.pop() {
        for (neighbor, neighbor_increment) in neighbors(state) {
            let best_neighbor_distance = distances.entry(neighbor).or_insert(usize::MAX);

            if *best_neighbor_distance > current_distance + neighbor_increment {
                *best_neighbor_distance = current_distance + neighbor_increment;
                queue.push((Reverse(*best_neighbor_distance), neighbor));
            }
        }
    }

    distances
}

fn solve((height, width, starting_position, rocks): &Input, steps: usize) -> usize {
    let neighbors = |(j, i): State| {
        [(j - 1, i), (j + 1, i), (j, i - 1), (j, i + 1)]
            .into_iter()
            .filter(|position| !rocks.contains(position))
            .filter(|&(j, i)| (1..=*height).contains(&j) && (1..=*width).contains(&i))
            .map(|neighbor_position| (neighbor_position, 1usize))
    };

    distances(*starting_position, neighbors)
        .into_iter()
        .filter(|(_, distance)| *distance <= steps && (distance % 2) == (steps % 2))
        .count()
}

#[aoc(day21, part1)]
fn part1(input: &Input) -> usize {
    solve(input, 64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        assert_eq!(16, solve(&parse(include_str!("../test_input/day21.part1.16.txt")).unwrap(), 6));
    }

    #[test]
    fn part1_input() {
        assert_eq!(3764, part1(&parse(include_str!("../input/2023/day21.txt")).unwrap()));
    }
}
