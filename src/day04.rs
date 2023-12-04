use std::collections::HashSet;
use std::str::FromStr;
use aoc_runner_derive::{aoc, aoc_generator};
use anyhow::{Result, Error, Context};

struct Card {
    winning_numbers: HashSet<u32>,
    numbers: HashSet<u32>,
}

impl FromStr for Card {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let (_, combined_numbers) = s
            .split_once(": ")
            .context(format!("Unable to parse card: header not found: {s}"))?;
        let (winning_numbers, numbers) = combined_numbers
            .split_once(" | ")
            .context(format!("Unable to parse card: separator not found {s}"))?;

        let winning_numbers = winning_numbers
            .trim()
            .split_ascii_whitespace()
            .map(str::trim)
            .map(|number| number.parse::<u32>().context(format!("Unable to parse number: {number}")))
            .collect::<Result<HashSet<u32>>>()?;

        let numbers = numbers
            .trim()
            .split_ascii_whitespace()
            .map(str::trim)
            .map(|number| number.parse().context(format!("Unable to parse number: {number}")))
            .collect::<Result<HashSet<u32>>>()?;

        Ok(Card { winning_numbers, numbers })
    }
}

#[aoc_generator(day4)]
fn parse(input: &str) -> Result<Vec<Card>> {
    input.lines().map(str::parse).collect()
}

#[aoc(day4, part1)]
fn part1(cards: &[Card]) -> u32 {
    cards
        .iter()
        .map(|card| card.winning_numbers.intersection(&card.numbers).count() as u32)
        .map(|num_winners| if num_winners == 0 { 0 } else { 2u32.pow(num_winners - 1) })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        assert_eq!(13, part1(&parse(include_str!("../test_input/day04.part1.13.txt")).unwrap()));
    }

    #[test]
    fn part1_input() {
        assert_eq!(21105, part1(&parse(include_str!("../input/2023/day4.txt")).unwrap()));
    }
}
