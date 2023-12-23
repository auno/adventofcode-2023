use std::cmp::max;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::Debug;
use std::hash::Hash;
use aoc_runner_derive::{aoc, aoc_generator};
use strum_macros::EnumIter;
use anyhow::{bail, Context, Error, Result};
use itertools::Itertools;
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

type Connections = HashMap<(usize, usize), Vec<((usize, usize), usize)>>;
fn dfs(connections: &Connections, source: (usize, usize), target: (usize, usize)) -> usize {
    type Cache = HashMap<(Vec<(usize, usize)>, usize), usize>;
    fn dfs_impl(cache: &mut Cache, connections: &Connections, path: &mut Vec<(usize, usize)>, target: (usize, usize), distance: usize) -> usize {
        let current_position = path.last().unwrap();
        let cache_key = (path.iter().copied().sorted().collect_vec(), distance);

        if let Some(cached_distance) = cache.get(&cache_key) {
            return *cached_distance;
        }

        if *current_position == target {
            return distance;
        }

        let (position_connected_to_target, _) = connections.get(&target).unwrap()[0];

        if position_connected_to_target != *current_position && path.contains(&position_connected_to_target) {
            cache.insert(cache_key, 0);
            return 0;
        }

        let neighbors = connections
            .get(current_position)
            .unwrap()
            .iter()
            .filter(|(neighbor_position, _)| !path.contains(neighbor_position))
            .collect_vec();

        let mut best = 0;

        for (neighbor_position, neighbor_distance_increment) in neighbors {
            path.push(*neighbor_position);
            best = max(best, dfs_impl(cache, connections, path, target, distance + neighbor_distance_increment));
            path.pop();
        }

        cache.insert(cache_key, best);
        best
    }

    dfs_impl(&mut HashMap::new(), connections, &mut vec![source], target, 0)
}

#[aoc(day23, part2)]
fn part2((height, _, map): &Input) -> usize {
    let source = map.iter().find(|&(&(j, _), &v)| v == Tile::Path && j == 0).map(|(&k, _)| k).unwrap();
    let target = map.iter().find(|&(&(j, _), &v)| v == Tile::Path && j == (height - 1)).map(|(&k, _)| k).unwrap();

    let mut connections = HashMap::new();
    let mut queue = VecDeque::from_iter([(source, Direction::South.step(source).unwrap(), Direction::South)]);
    let mut visited = HashSet::from([source]);

    while let Some((run_source, position, direction)) = queue.pop_front() {
        visited.insert(position);
        let mut distance = 1usize;
        let mut current_position = position;
        let mut current_direction = direction;

        loop {
            let neighbors = Direction::iter()
                .filter(|&neighbor_direction| neighbor_direction != current_direction.reverse())
                .filter_map(|neighbor_direction| Some((neighbor_direction.step(current_position)?, neighbor_direction)))
                .filter(|(neighbor_position, _)| match map.get(neighbor_position) {
                    Some(Tile::Path | Tile::Slope(_)) => true,
                    None | Some(Tile::Forest) => false,
                })
                .collect_vec();

            match neighbors.len() {
                1 => {
                    (current_position, current_direction) = neighbors[0];
                    distance += 1;
                },
                _ => {
                    connections.entry(run_source).or_insert_with(Vec::new).push((current_position, distance));
                    connections.entry(current_position).or_insert_with(Vec::new).push((run_source, distance));

                    for (neighbor_position, neighbor_direction) in neighbors {
                        if !visited.contains(&neighbor_position) {
                            queue.push_back((current_position, neighbor_position, neighbor_direction));
                        }
                    }

                    break;
                },
            }
        }
    }

    dfs(&connections, source, target)
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

    #[test]
    fn part2_example1() {
        assert_eq!(154, part2(&parse(include_str!("../test_input/day23.example1.txt")).unwrap()));
    }

    // #[test]
    // fn part2_input() {
    //     assert_eq!(6734, part2(&parse(include_str!("../input/2023/day23.txt")).unwrap()).unwrap());
    // }
}
