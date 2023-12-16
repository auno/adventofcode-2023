use std::collections::{HashMap, HashSet, VecDeque};
use aoc_runner_derive::{aoc, aoc_generator};
use anyhow::{bail, Context, Error, Result};
use itertools::{chain, Itertools};

#[derive(Eq, PartialEq, Copy, Clone)]
enum Element {
    Mirror1,
    Mirror2,
    SplitterH,
    SplitterV,
}

impl TryFrom<char> for Element {
    type Error = Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(
            match value {
                '/' => Element::Mirror1,
                '\\' => Element::Mirror2,
                '|' => Element::SplitterV,
                '-' => Element::SplitterH,
                _ => bail!("Invalid element: {value}"),
            }
        )
    }
}

type Elements = HashMap<(usize, usize), Element>;
type Input = (usize, usize, Elements);

#[aoc_generator(day16)]
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
                .map(move |(i, c)| Ok(((j + 1, i + 1), Element::try_from(c)?)))
        })
        .collect::<Result<HashMap<(usize, usize), Element>>>();

    Ok((height, width, map?))
}

#[derive(Eq, PartialEq, Copy, Clone, Hash, Debug)]
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
}

type State = ((usize, usize), Direction);

fn neighbors((position, direction): State, elements: &Elements, height: usize, width: usize) -> Vec<State> {
    let neighbors = match (elements.get(&position), direction) {
        (None, _) => vec![(direction.step(position), direction)],

        (Some(Element::Mirror1), Direction::Up) => vec![(Direction::Right.step(position), Direction::Right)],
        (Some(Element::Mirror1), Direction::Down) => vec![(Direction::Left.step(position), Direction::Left)],
        (Some(Element::Mirror1), Direction::Left) => vec![(Direction::Down.step(position), Direction::Down)],
        (Some(Element::Mirror1), Direction::Right) => vec![(Direction::Up.step(position), Direction::Up)],

        (Some(Element::Mirror2), Direction::Up) => vec![(Direction::Left.step(position), Direction::Left)],
        (Some(Element::Mirror2), Direction::Down) => vec![(Direction::Right.step(position), Direction::Right)],
        (Some(Element::Mirror2), Direction::Left) => vec![(Direction::Up.step(position), Direction::Up)],
        (Some(Element::Mirror2), Direction::Right) => vec![(Direction::Down.step(position), Direction::Down)],

        (Some(Element::SplitterH), Direction::Left | Direction::Right) => vec![(direction.step(position), direction)],
        (Some(Element::SplitterH), Direction::Up | Direction::Down) => vec![
            (Direction::Left.step(position), Direction::Left),
            (Direction::Right.step(position), Direction::Right)
        ],

        (Some(Element::SplitterV), Direction::Up | Direction::Down) => vec![(direction.step(position), direction)],
        (Some(Element::SplitterV), Direction::Left | Direction::Right) => vec![
            (Direction::Up.step(position), Direction::Up),
            (Direction::Down.step(position), Direction::Down)
        ],
    };

    neighbors
        .into_iter()
        .filter(|((j, i), _)| (1..=height).contains(j) && (1..=width).contains(i))
        .collect_vec()
}

fn count_energized_tiles(elements: &Elements, height: &usize, width: &usize, start_state: ((usize, usize), Direction)) -> usize {
    let mut seen: HashSet<State> = HashSet::from([start_state]);
    let mut queue: VecDeque<State> = VecDeque::from([start_state]);

    while let Some(state) = queue.pop_front() {
        for &neighbor in neighbors(state, elements, *height, *width).iter() {
            if seen.contains(&neighbor) {
                continue;
            }

            seen.insert(neighbor);
            queue.push_back(neighbor);
        }
    }

    seen.into_iter()
        .map(|(pos, _)| pos)
        .unique()
        .count()
}

#[aoc(day16, part1)]
fn part1((height, width, elements): &Input) -> usize {
    count_energized_tiles(elements, height, width, ((1, 1), Direction::Right))
}

#[aoc(day16, part2)]
fn part2((height, width, elements): &Input) -> Option<usize> {
    chain(
        (1..=*height).flat_map(|j| [((j, 1), Direction::Right), ((j, *width), Direction::Left)]),
        (1..=*width).flat_map(|i| [((1, i), Direction::Down), ((*height, i), Direction::Up)]),
    )
        .map(|start_state| count_energized_tiles(elements, height, width, start_state))
        .max()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        assert_eq!(46, part1(&parse(include_str!("../test_input/day16.part1.46.txt")).unwrap()));
    }

    #[test]
    fn part1_input() {
        assert_eq!(7608, part1(&parse(include_str!("../input/2023/day16.txt")).unwrap()));
    }

    #[test]
    fn part2_example1() {
        assert_eq!(51, part2(&parse(include_str!("../test_input/day16.part2.51.txt")).unwrap()).unwrap());
    }

    #[test]
    fn part2_input() {
        assert_eq!(8221, part2(&parse(include_str!("../input/2023/day16.txt")).unwrap()).unwrap());
    }
}
