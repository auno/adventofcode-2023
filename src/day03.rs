use std::collections::HashMap;
use anyhow::bail;
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

enum Tile {
    Digit(u32),
    Symbol(char),
}

impl TryFrom<char> for Tile {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => bail!("No such tile: {value}"),
            '0'..='9' => Ok(Tile::Digit(value.to_digit(10).unwrap())),
            _ => Ok(Tile::Symbol(value)),
        }
    }
}

type Schematic = HashMap<(isize, isize), Tile>;

#[aoc_generator(day3)]
fn parse(input: &str) -> Schematic {
    let mut schematic = Schematic::new();

    for (j, line) in input.lines().enumerate() {
        for (i, c) in line.chars().enumerate() {
            if let Ok(tile) = Tile::try_from(c) {
                schematic.insert((j as isize, i as isize), tile);
            }
        }
    }

    schematic
}

#[aoc(day3, part1)]
fn part1(schematic: &Schematic) -> u32 {
    let sorted_tiles = schematic.iter().sorted_by_key(|(&(j, i), _)| (j, i)).collect_vec();

    let mut sum = 0;
    let mut previous_position = (0, 0);
    let mut number = 0;
    let mut adjacent_symbol = false;

    for (&(j, i), tile) in sorted_tiles {
        if previous_position != (j, i - 1) {
            if adjacent_symbol {
                sum += number;
                eprintln!("-- Adding {} at {:?}", number, (j, i));
            }

            number = 0;
            adjacent_symbol = false;
        }

        if let Tile::Digit(d) = tile {
            number = number * 10 + d;

            for neighbor in [(j, i-1), (j, i+1), (j-1, i), (j+1, i), (j-1, i-1), (j-1, i+1), (j+1, i-1), (j+1, i+1)] {
                if let Some(Tile::Symbol(_)) = schematic.get(&neighbor) {
                    adjacent_symbol = true;
                }
            }
        } else {
            if adjacent_symbol {
                sum += number;
                eprintln!("-- Adding {} at {:?}", number, (j, i));
            }

            number = 0;
            adjacent_symbol = false;
        }

        previous_position = (j, i);
    }

    if adjacent_symbol {
        sum += number;
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        assert_eq!(4361, part1(&parse(include_str!("../test_input/day03.part1.4361.txt"))));
    }

    #[test]
    fn part1_input() {
        assert_eq!(539713, part1(&parse(include_str!("../input/2023/day3.txt"))));
    }
}
