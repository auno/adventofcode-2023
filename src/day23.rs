use std::collections::{BinaryHeap, HashMap};
use std::fmt::Debug;
use std::hash::Hash;
use aoc_runner_derive::{aoc, aoc_generator};
use strum_macros::EnumIter;
use anyhow::{bail, Context, Error, Result};
use strum::IntoEnumIterator;

#[derive(Copy, Clone, Eq, PartialEq, EnumIter, Ord, PartialOrd, Hash, Debug)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    fn step(&self, (j, i): (usize, usize)) -> Option<(usize, usize)> {
        Some(
            match self {
                Direction::North => (j.checked_sub(1)?, i),
                Direction::South => (j.checked_add(1)?, i),
                Direction::West => (j, i.checked_sub(1)?),
                Direction::East => (j, i.checked_add(1)?),
            }
        )
    }

    fn reverse(&self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
            Direction::East => Direction::West,
        }
    }
}

impl TryFrom<char> for Direction {
    type Error = Error;

    fn try_from(value: char) -> Result<Self> {
        Ok(
            match value {
                '^' => Direction::North,
                'v' => Direction::South,
                '<' => Direction::West,
                '>' => Direction::East,
                _ => bail!("Invalid direction: {value}"),
            }
        )
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Tile {
    Path,
    Forest,
    Slope(Direction),
}

impl TryFrom<char> for Tile {
    type Error = Error;

    fn try_from(value: char) -> Result<Self> {
        Ok(
            match value {
                '.' => Tile::Path,
                '#' => Tile::Forest,
                _ => Tile::Slope(Direction::try_from(value).context(format!("Invalid tile: {value}"))?),
            }
        )
    }
}

type Input = (usize, usize, HashMap<(usize, usize), Tile>);

#[aoc_generator(day23)]
fn parse(input: &str) -> Result<Input> {
    let height = input.lines().count();
    let width = input.lines().next().context("Unexpected empty line")?.len();

    let map = input
        .chars()
        .filter(|c| !c.is_ascii_whitespace())
        .enumerate()
        .map(|(i, c)| Ok(((i / width, i % width), Tile::try_from(c)?)))
        .collect::<Result<_>>()?;

    Ok((height, width, map))
}

fn distance<SearchState: Ord + Hash + Copy + Debug, IntoNeighborIterator: IntoIterator<Item=(SearchState, usize)>>(
    source: SearchState,
    neighbors: impl Fn(SearchState) -> IntoNeighborIterator,
    arrived: impl Fn(SearchState) -> bool,
) -> Option<usize> {
    let mut distances: HashMap<SearchState, usize> = HashMap::new();
    let mut queue: BinaryHeap<(usize, SearchState)> = BinaryHeap::new();

    distances.insert(source, 0);
    queue.push((0, source));

    while let Some((current_distance, state)) = queue.pop() {
        for (neighbor, neighbor_increment) in neighbors(state) {
            let best_neighbor_distance = distances.entry(neighbor).or_insert(0);

            if *best_neighbor_distance < current_distance + neighbor_increment {
                *best_neighbor_distance = current_distance + neighbor_increment;
                queue.push((*best_neighbor_distance, neighbor));
            }
        }
    }

    distances
        .into_iter()
        .find(|&(state, _)| arrived(state))
        .map(|(_, distance)| distance)
}

#[aoc(day23, part1)]
fn part1((height, width, map): &Input) -> Option<usize> {
    let source = map.iter().find(|&(&(j, _), &v)| v == Tile::Path && j == 0).map(|(&k, _)| k)?;
    let target = map.iter().find(|&(&(j, _), &v)| v == Tile::Path && j == (height - 1)).map(|(&k, _)| k)?;

    type State = ((usize, usize), Direction);

    let neighbors = |(position, direction): State| {
        let Some(&tile) = map.get(&position) else { panic!("No tile at current position: {:?}", position) };

        Direction::iter()
            .filter(move |&neighbor_direction| {
                let Some(neighbor_position) = neighbor_direction.step(position) else { return false; };
                matches!(map.get(&neighbor_position), Some(Tile::Path | Tile::Slope(_)))
            })
            .filter(move |&neighbor_direction| {
                let Tile::Slope(slope_direction) = tile else { return true };
                neighbor_direction == slope_direction
            })
            .filter(move |&neighbor_direction| neighbor_direction != direction.reverse())
            .filter(move |&neighbor_direction| {
                let Some((j, i)) = neighbor_direction.step(position) else { return false };
                (0..*height).contains(&j) && (0..*width).contains(&i)
            })
            .filter_map(move |neighbor_direction| {
                let neighbor_position = neighbor_direction.step(position)?;
                Some(((neighbor_position, neighbor_direction), 1))
            })
    };

    let arrived = |(position, _)| position == target;

    distance((source, Direction::East), neighbors, arrived)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        assert_eq!(94, part1(&parse(include_str!("../test_input/day23.example1.txt")).unwrap()).unwrap());
    }

    #[test]
    fn part1_input() {
        assert_eq!(2278, part1(&parse(include_str!("../input/2023/day23.txt")).unwrap()).unwrap());
    }
}
