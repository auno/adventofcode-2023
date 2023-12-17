use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use std::hash::Hash;
use aoc_runner_derive::{aoc, aoc_generator};
use anyhow::{Context, Result};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

type Input = (usize, usize, HashMap<(usize, usize), usize>);

#[aoc_generator(day17)]
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
                .map(move |(i, c)| Ok(((j + 1, i + 1), c.to_digit(10).context(format!("Invalid heat loss number: {c}"))? as usize)))
        })
        .collect::<Result<_>>();

    Ok((height, width, map?))
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Hash, EnumIter)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn step(&self, (j, i): (usize, usize)) -> (usize, usize) {
        match self {
            Direction::Up => (j - 1, i),
            Direction::Down => (j + 1, i),
            Direction::Left => (j, i - 1),
            Direction::Right => (j, i + 1),
        }
    }

    fn reverse(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

type State = ((usize, usize), Direction, usize);
fn distance<SearchState: Ord + Hash + Copy, IntoNeighborIterator: IntoIterator<Item=(SearchState, usize)>>(
    source: SearchState,
    neighbors: impl Fn(SearchState) -> IntoNeighborIterator,
    arrived: impl Fn(SearchState) -> bool,
) -> Option<usize> {
    let mut distances: HashMap<SearchState, usize> = HashMap::new();
    let mut queue: BinaryHeap<(Reverse<usize>, SearchState)> = BinaryHeap::new();

    distances.insert(source, 0);
    queue.push((Reverse(0), source));

    while let Some((Reverse(current_distance), state)) = queue.pop() {
        if arrived(state) {
            return Some(current_distance);
        }

        for (neighbor, neighbor_increment) in neighbors(state) {
            let best_neighbor_distance = distances.entry(neighbor).or_insert(usize::MAX);

            if *best_neighbor_distance > current_distance + neighbor_increment {
                *best_neighbor_distance = current_distance + neighbor_increment;
                queue.push((Reverse(*best_neighbor_distance), neighbor));
            }
        }
    }

    None
}

#[aoc(day17, part1)]
fn part1(input: &Input) -> Option<usize> {
    let (height, width, grid) = input;
    let neighbors = |(position, direction, run_len): State| {
        Direction::iter()
            .filter(move |&neighbor_direction| neighbor_direction != direction.reverse())
            .filter(move |&neighbor_direction| run_len < 3 || neighbor_direction != direction)
            .filter(move |&neighbor_direction| {
                let (j, i) = neighbor_direction.step(position);
                (1..=*height).contains(&j) && (1..=*width).contains(&i)
            })
            .map(move |neighbor_direction| {
                let neighbor_position = neighbor_direction.step(position);
                let neighbor_run_len = if neighbor_direction == direction { run_len + 1 } else { 1 };
                let neighbor_cost = *grid.get(&neighbor_position).unwrap_or(&usize::MAX);

                ((neighbor_position, neighbor_direction, neighbor_run_len), neighbor_cost)
            })
    };
    let arrived = |(position, _, _)| position == (*height, *width);

    distance(((1, 1), Direction::Right, 0), neighbors, arrived)
}

#[aoc(day17, part2)]
fn part2(input: &Input) -> Option<usize> {
    let (height, width, grid) = input;
    let neighbors = |(position, direction, run_len): State| {
        Direction::iter()
            .filter(move |&neighbor_direction| neighbor_direction != direction.reverse())
            .filter(move |&neighbor_direction| (run_len == 0 || run_len >= 4) || neighbor_direction == direction)
            .filter(move |&neighbor_direction| run_len < 10 || neighbor_direction != direction)
            .filter(move |&neighbor_direction| {
                let (j, i) = neighbor_direction.step(position);
                (1..=*height).contains(&j) && (1..=*width).contains(&i)
            })
            .map(move |neighbor_direction| {
                let neighbor_position = neighbor_direction.step(position);
                let neighbor_run_len = if neighbor_direction == direction { run_len + 1 } else { 1 };
                let neighbor_cost = *grid.get(&neighbor_position).unwrap_or(&usize::MAX);

                ((neighbor_position, neighbor_direction, neighbor_run_len), neighbor_cost)
            })
    };
    let arrived = |(position, _, run_len)| position == (*height, *width) && run_len >= 4;

    distance(((1, 1), Direction::Right, 0), neighbors, arrived)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        assert_eq!(102, part1(&parse(include_str!("../test_input/day17.part1.102.txt")).unwrap()).unwrap());
    }

    #[test]
    fn part1_input() {
        assert_eq!(1004, part1(&parse(include_str!("../input/2023/day17.txt")).unwrap()).unwrap());
    }

    #[test]
    fn part2_example1() {
        assert_eq!(94, part2(&parse(include_str!("../test_input/day17.part2.94.txt")).unwrap()).unwrap());
    }

    #[test]
    fn part2_example2() {
        assert_eq!(71, part2(&parse(include_str!("../test_input/day17.part2.71.txt")).unwrap()).unwrap());
    }

    #[test]
    fn part2_input() {
        assert_eq!(1171, part2(&parse(include_str!("../input/2023/day17.txt")).unwrap()).unwrap());
    }
}
