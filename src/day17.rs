use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use aoc_runner_derive::{aoc, aoc_generator};
use anyhow::{Context, Result};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

type Input = (usize, usize, HashMap<(usize, usize), u32>);

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
                .map(move |(i, c)| Ok(((j + 1, i + 1), c.to_digit(10).context(format!("Invalid heat loss number: {c}"))?)))
        })
        .collect::<Result<HashMap<(usize, usize), u32>>>();

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

fn distance(source: (usize, usize), target: (usize, usize), (height, width, grid): &Input) -> Option<u32> {
    type State = ((usize, usize), Direction, usize);

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

                (neighbor_position, neighbor_direction, neighbor_run_len)
            })
    };

    let mut distances: HashMap<State, u32> = HashMap::new();
    let mut queue: BinaryHeap<(Reverse<u32>, State)> = BinaryHeap::new();

    distances.insert((source, Direction::Right, 0), 0);
    queue.push((Reverse(0), (source, Direction::Right, 0)));

    while let Some((Reverse(cost), state)) = queue.pop() {
        let (position, _, _) = state;

        if position == target {
            return Some(cost);
        }

        for neighbor in neighbors(state) {
            let neighbor_distance = distances.entry(neighbor).or_insert(u32::MAX);
            let (neighbor_position, _, _) = neighbor;
            let neighbor_cost = *grid.get(&neighbor_position)?;

            if *neighbor_distance > cost + neighbor_cost {
                *neighbor_distance = cost + neighbor_cost;
                queue.push((Reverse(*neighbor_distance), neighbor));
            }
        }
    }

    None
}

#[aoc(day17, part1)]
fn part1(input: &Input) -> Option<u32> {
    let &(height, width, _) = input;
    distance((1, 1), (height, width), input)
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
}
