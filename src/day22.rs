use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};
use std::str::FromStr;
use aoc_runner_derive::{aoc, aoc_generator};
use anyhow::{bail, Context, Result, Error};
use itertools::Itertools;
use rayon::iter::{ParallelBridge, ParallelIterator};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct Cube {
    x: usize,
    y: usize,
    z: usize,
}

impl Cube {
    fn lower(&self) -> Self {
        Self {
            x: self.x,
            y: self.y,
            z: self.z - 1,
        }
    }

    fn raise(&self) -> Self {
        Self {
            x: self.x,
            y: self.y,
            z: self.z + 1,
        }
    }
}

#[derive(Clone, Eq, PartialEq, Hash)]
struct Brick {
    cubes: Vec<Cube>,
    level: usize,
}

impl Brick {
    fn lower(&self) -> Self {
        Self {
            cubes: self.cubes.iter().map(|cube| cube.lower()).collect_vec(),
            level: self.level - 1,
        }
    }

    fn raise(&self) -> Self {
        Self {
            cubes: self.cubes.iter().map(|cube| cube.raise()).collect_vec(),
            level: self.level + 1,
        }
    }
}

impl FromStr for Brick {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let coords = s
            .split(&[',', '~'])
            .map(|num| num.parse::<usize>().context(format!("Invalid brick: {s}")))
            .collect::<Result<Vec<_>>>()?;

        let Some((x1, y1, z1, x2, y2, z2)) = coords.into_iter().collect_tuple() else {
            bail!("Invalid brick: {s}");
        };

        let mut cubes = vec![];

        for x in min(x1, x2)..=max(x1, x2) {
            for y in min(y1, y2)..=max(y1, y2) {
                for z in min(z1, z2)..=max(z1, z2) {
                    cubes.push(Cube { x, y, z });
                }
            }
        }

        cubes.sort_by_key(|cube| (cube.z, cube.y, cube.x));

        Ok(Brick { cubes, level: min(z1, z2) })
    }
}

type Input = Vec<Brick>;

#[aoc_generator(day22)]
fn parse(input: &str) -> Result<Input> {
    input
        .lines()
        .map(Brick::from_str)
        .collect()
}

fn settle_bricks(bricks: &[Brick]) -> (Vec<Brick>, usize) {
    let mut occupied = HashSet::new();
    let mut settled_bricks = vec![];
    let mut num_fallen = 0;

    for brick in bricks.iter().sorted_by_key(|brick| brick.level) {
        let mut settling_brick = brick.clone();

        while settling_brick.level > 1 {
            let lowered_brick = settling_brick.lower();

            if lowered_brick.cubes.iter().any(|cube| occupied.contains(cube)) {
                break;
            }

            settling_brick = lowered_brick;
        }

        for &cube in &settling_brick.cubes {
            occupied.insert(cube);
        }

        if settling_brick.level < brick.level {
            num_fallen += 1;
        }

        settled_bricks.push(settling_brick);
    }

    (settled_bricks, num_fallen)
}

#[aoc(day22, part1)]
fn part1(bricks: &Input) -> usize {
    let (settled_bricks, _) = settle_bricks(bricks);

    let bricks_by_cube = settled_bricks
        .iter()
        .flat_map(|brick| brick.cubes.iter().map(move |&cube| (cube, brick)))
        .collect::<HashMap<_, _>>();

    settled_bricks
        .iter()
        .filter(|settled_brick| {
            let supported_bricks = settled_brick.raise().cubes
                .iter()
                .filter_map(|cube| bricks_by_cube.get(cube))
                .unique()
                .filter(|brick| brick != &settled_brick)
                .collect_vec();

            supported_bricks
                .iter()
                .all(|supported_brick| {
                    supported_brick.lower().cubes
                        .iter()
                        .filter_map(|cube| bricks_by_cube.get(cube))
                        .unique()
                        .filter(|brick| brick != supported_brick)
                        .count() > 1
                })
        })
        .count()
}

#[aoc(day22, part2)]
fn part2(bricks: &Input) -> usize {
    let (settled_bricks, _) = settle_bricks(bricks);

    (0..settled_bricks.len())
        .par_bridge()
        .map(|i| {
            let bricks = settled_bricks
                .iter()
                .enumerate()
                .filter(|&(j, _)| j != i)
                .map(|(_, b)| b)
                .cloned()
                .collect_vec();
            let (_, num_fallen) = settle_bricks(&bricks);

            num_fallen
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        assert_eq!(5, part1(&parse(include_str!("../test_input/day22.example1.txt")).unwrap()));
    }

    #[test]
    fn part1_input() {
        assert_eq!(499, part1(&parse(include_str!("../input/2023/day22.txt")).unwrap()));
    }

    #[test]
    fn part2_example1() {
        assert_eq!(7, part2(&parse(include_str!("../test_input/day22.example1.txt")).unwrap()));
    }

    #[test]
    fn part2_input() {
        assert_eq!(95059, part2(&parse(include_str!("../input/2023/day22.txt")).unwrap()));
    }
}
