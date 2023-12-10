use std::cmp::max;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::{Display, Formatter, Write};
use std::iter::successors;
use aoc_runner_derive::{aoc, aoc_generator};
use anyhow::{bail, Context, Error, Result};
use itertools::Itertools;

#[derive(Copy, Clone, Debug)]
enum Pipe {
    Vertical,
    Horizontal,
    BendNorthEast,
    BendNorthWest,
    BendSouthEast,
    BendSouthWest,
}

impl Display for Pipe {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Pipe::Vertical => f.write_char('│'),
            Pipe::Horizontal => f.write_char('─'),
            Pipe::BendNorthEast => f.write_char('└'),
            Pipe::BendNorthWest => f.write_char('┘'),
            Pipe::BendSouthEast => f.write_char('┌'),
            Pipe::BendSouthWest => f.write_char('┐'),
        }
    }
}

impl TryFrom<char> for Pipe {
    type Error = Error;

    fn try_from(value: char) -> Result<Self> {
        Ok(
            match value {
                '|' => Pipe::Vertical,
                '-' => Pipe::Horizontal,
                'L' => Pipe::BendNorthEast,
                'J' => Pipe::BendNorthWest,
                'F' => Pipe::BendSouthEast,
                '7' => Pipe::BendSouthWest,
                _ => bail!("Invalid pipe: {value}"),
            }
        )
    }
}

type Input = ((i32, i32), HashMap<(i32, i32), Pipe>);

fn determine_starting_pipe(starting_position: &(i32, i32), map: &HashMap<(i32, i32), Pipe>) -> Result<Pipe, Error> {
    match (
        map.get(&(starting_position.0, starting_position.1 - 1)),
        map.get(&(starting_position.0, starting_position.1 + 1)),
        map.get(&(starting_position.0 - 1, starting_position.1)),
        map.get(&(starting_position.0 + 1, starting_position.1)),
    ) {
        (Some(Pipe::Horizontal | Pipe::BendNorthEast | Pipe::BendSouthEast), Some(Pipe::Horizontal | Pipe::BendNorthWest | Pipe::BendSouthWest), _, _) => Some(Pipe::Horizontal),
        (_, _, Some(Pipe::Vertical | Pipe::BendSouthEast | Pipe::BendSouthWest), Some(Pipe::Vertical | Pipe::BendNorthEast | Pipe::BendNorthWest)) => Some(Pipe::Vertical),
        (_, Some(Pipe::Horizontal | Pipe::BendNorthWest | Pipe::BendSouthWest), Some(Pipe::Vertical | Pipe::BendSouthEast | Pipe::BendSouthWest), _) => Some(Pipe::BendNorthEast),
        (Some(Pipe::Horizontal | Pipe::BendNorthEast | Pipe::BendSouthEast), _, Some(Pipe::Vertical | Pipe::BendSouthEast | Pipe::BendSouthWest), _) => Some(Pipe::BendNorthWest),
        (_, Some(Pipe::Horizontal | Pipe::BendNorthWest | Pipe::BendSouthWest), _, Some(Pipe::Vertical | Pipe::BendNorthEast | Pipe::BendNorthWest)) => Some(Pipe::BendSouthEast),
        (Some(Pipe::Horizontal | Pipe::BendNorthEast | Pipe::BendSouthEast), _, _, Some(Pipe::Vertical | Pipe::BendNorthEast | Pipe::BendNorthWest)) => Some(Pipe::BendSouthWest),
        _ => None,
    }.context("No starting position found")
}

#[aoc_generator(day10)]
fn parse(input: &str) -> Result<Input> {
    let mut map = HashMap::new();
    let mut starting_position = None;

    for (j, line) in input.lines().enumerate() {
        for (i, c) in line.chars().enumerate() {
            let position = (j as i32, i as i32);
            match c {
                '.' => {},
                'S' => { starting_position = Some(position); },
                _ => { map.insert(position, c.try_into()?); },
            }
        }
    }

    let starting_position = starting_position.context("No starting position found")?;
    let starting_pipe = determine_starting_pipe(&starting_position, &map)?;
    map.insert(starting_position, starting_pipe);

    Ok((starting_position, map))
}

fn step(pos: (i32, i32), ppos: Option<(i32, i32)>, map: &HashMap<(i32, i32), Pipe>) -> ((i32, i32), Option<(i32, i32)>) {
    match (map.get(&pos), ppos) {
        (Some(Pipe::Vertical | Pipe::BendSouthEast | Pipe::BendSouthWest), None) => ((pos.0 + 1, pos.1), Some(pos)),
        (Some(Pipe::Horizontal | Pipe::BendNorthWest), None) => ((pos.0, pos.1 + 1), Some(pos)),
        (Some(Pipe::BendNorthEast), None) => ((pos.0, pos.1 - 1), Some(pos)),
        (Some(Pipe::Vertical | Pipe::BendSouthEast | Pipe::BendSouthWest), Some((pj, _))) if pj != pos.0 + 1 => ((pos.0 + 1, pos.1), Some(pos)),
        (Some(Pipe::Vertical | Pipe::BendNorthEast | Pipe::BendNorthWest), Some((pj, _))) if pj != pos.0 - 1 => ((pos.0 - 1, pos.1), Some(pos)),
        (Some(Pipe::Horizontal | Pipe::BendNorthWest | Pipe::BendSouthWest), Some((_, pi))) if pi != pos.1 - 1 => ((pos.0, pos.1 - 1), Some(pos)),
        (Some(Pipe::Horizontal | Pipe::BendNorthEast | Pipe::BendSouthEast), Some((_, pi))) if pi != pos.1 + 1 => ((pos.0, pos.1 + 1), Some(pos)),
        (pipe, _) => panic!("Nowhere to go from {:?}, coming from {:?}: {:?}", pos, ppos, pipe),
    }
}

fn find_path(starting_position: (i32, i32), map: &HashMap<(i32, i32), Pipe>) -> Vec<(i32, i32)> {
    successors(Some((starting_position, None)), |(position, previous_position)| {
        let (position, previous_position) = step(*position, *previous_position, map);

        if position == starting_position {
            return None;
        }

        Some((position, previous_position))
    }).map(|(pos, _)| pos).collect_vec()
}

fn transform_tile(position: (i32, i32), pipe: Pipe) -> [((i32, i32), Pipe); 3] {
    let (j, i) = position;

    match pipe {
        Pipe::Vertical => [((j * 3 - 1, i * 3), Pipe::Vertical), ((j * 3, i * 3), Pipe::Vertical), ((j * 3 + 1, i * 3), Pipe::Vertical)],
        Pipe::Horizontal => [((j * 3, i * 3 - 1), Pipe::Horizontal), ((j * 3, i * 3), Pipe::Horizontal), ((j * 3, i * 3 + 1), Pipe::Horizontal)],
        Pipe::BendNorthEast => [((j * 3 - 1, i * 3), Pipe::Vertical), ((j * 3, i * 3), Pipe::BendNorthEast), ((j * 3, i * 3 + 1), Pipe::Horizontal)],
        Pipe::BendNorthWest => [((j * 3 - 1, i * 3), Pipe::Vertical), ((j * 3, i * 3), Pipe::BendNorthWest), ((j * 3, i * 3 - 1), Pipe::Horizontal)],
        Pipe::BendSouthEast => [((j * 3 + 1, i * 3), Pipe::Vertical), ((j * 3, i * 3), Pipe::BendSouthEast), ((j * 3, i * 3 + 1), Pipe::Horizontal)],
        Pipe::BendSouthWest => [((j * 3 + 1, i * 3), Pipe::Vertical), ((j * 3, i * 3), Pipe::BendSouthWest), ((j * 3, i * 3 - 1), Pipe::Horizontal)],
    }
}

fn transform_path(path: Vec<(i32, i32)>, map: &HashMap<(i32, i32), Pipe>) -> HashSet<(i32, i32)> {
    path
        .iter()
        .flat_map(|&position| transform_tile(position, *map.get(&position).unwrap()))
        .map(|(position, _)| position)
        .collect()
}

fn fill(path: &HashSet<(i32, i32)>, height: i32, width: i32) -> HashSet<(i32, i32)> {
    let mut outside = HashSet::from([(-1, -1)]);
    let mut queue = VecDeque::from([(-1, -1)]);

    while let Some(position) = queue.pop_front() {
        for neighbor in [
            (position.0 - 1, position.1 - 1), (position.0 - 1, position.1), (position.0 - 1, position.1 + 1),
            (position.0, position.1 - 1), (position.0, position.1 + 1),
            (position.0 + 1, position.1 - 1), (position.0 + 1, position.1), (position.0 + 1, position.1 + 1),
        ] {
            if !outside.contains(&neighbor) && !path.contains(&neighbor) && (-1..height).contains(&neighbor.0) && (-1..width).contains(&neighbor.1) {
                queue.push_back(neighbor);
                outside.insert(neighbor);
            }
        }
    }
    outside
}

fn count_inside(transformed_path: &HashSet<(i32, i32)>, height: i32, width: i32, outside: &HashSet<(i32, i32)>) -> usize {
    (0..height).step_by(3).cartesian_product((0..width).step_by(3))
        .filter(|position| !outside.contains(position))
        .filter(|position| !transformed_path.contains(position))
        .count()
}

#[aoc(day10, part1)]
fn part1(input: &Input) -> usize {
    let (starting_position, map) = input.clone();
    let path = find_path(starting_position, &map);

    path.len() / 2
}

#[aoc(day10, part2)]
fn part2(input: &Input) -> usize {
    let (starting_position, map) = input.clone();
    let path = transform_path(find_path(starting_position, &map), &map);

    let (height, width) = map
        .keys()
        .fold((0, 0), |acc, &pos| (max(acc.0, pos.0 * 3 + 2), max(acc.1, pos.1 * 3 + 2)));

    let outside = fill(&path, height, width);

    count_inside(&path, height, width, &outside)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        assert_eq!(4, part1(&parse(include_str!("../test_input/day10.part1.4.txt")).unwrap()));
    }

    #[test]
    fn part1_example2() {
        assert_eq!(8, part1(&parse(include_str!("../test_input/day10.part1.8.txt")).unwrap()));
    }

    #[test]
    fn part1_input() {
        assert_eq!(6882, part1(&parse(include_str!("../input/2023/day10.txt")).unwrap()));
    }

    #[test]
    fn part2_example1() {
        assert_eq!(4, part2(&parse(include_str!("../test_input/day10.part2.4.txt")).unwrap()));
    }

    #[test]
    fn part2_example2() {
        assert_eq!(8, part2(&parse(include_str!("../test_input/day10.part2.8.txt")).unwrap()));
    }

    #[test]
    fn part2_example3() {
        assert_eq!(10, part2(&parse(include_str!("../test_input/day10.part2.10.txt")).unwrap()));
    }

    #[test]
    fn part2_input() {
        assert_eq!(491, part2(&parse(include_str!("../input/2023/day10.txt")).unwrap()));
    }
}
