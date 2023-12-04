use std::collections::HashSet;
use std::str::FromStr;
use aoc_runner_derive::{aoc, aoc_generator};
use anyhow::{Result, Error, Context};
use itertools::Itertools;

#[derive(Clone)]
struct Card {
    card_number: usize,
    winning_numbers: HashSet<u32>,
    numbers: HashSet<u32>,
}

impl FromStr for Card {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let (header, combined_numbers) = s
            .split_once(": ")
            .context(format!("Unable to parse card: header not found: {s}"))?;

        let card_number = header
            .split_whitespace()
            .nth(1)
            .context(format!("Unable to parse card header: {header}"))?
            .parse()
            .context(format!("Unable to parse card number from header: {header}"))?;

        let (winning_numbers, numbers) = combined_numbers
            .split_once(" | ")
            .context(format!("Unable to parse card: separator not found {s}"))?;

        let winning_numbers = winning_numbers
            .split_whitespace()
            .map(str::trim)
            .map(|number| number.parse::<u32>().context(format!("Unable to parse number: {number}")))
            .collect::<Result<HashSet<u32>>>()?;

        let numbers = numbers
            .split_whitespace()
            .map(str::trim)
            .map(|number| number.parse().context(format!("Unable to parse number: {number}")))
            .collect::<Result<HashSet<u32>>>()?;

        Ok(Card { card_number, winning_numbers, numbers })
    }
}

impl Card {
    fn count_winners(&self) -> usize {
        self.winning_numbers.intersection(&self.numbers).count()
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
        .map(|card| card.count_winners() as u32)
        .map(|num_winners| if num_winners == 0 { 0 } else { 2u32.pow(num_winners - 1) })
        .sum()
}

#[aoc(day4, part2)]
fn part2(cards: &[Card]) -> usize {
    let mut precomputed_cards = cards
        .iter()
        .map(|card| (
            card.card_number,
            card.count_winners(),
        ))
        .collect_vec();

    let mut i = 0;

    while i < precomputed_cards.len() {
        let &(card_number, num_winners) = &precomputed_cards[i];

        for j in 1..=num_winners {
            precomputed_cards.push(precomputed_cards[card_number - 1 + j]);
        }

        i += 1;
    }

    precomputed_cards.len()
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

    #[test]
    fn part2_example1() {
        assert_eq!(30, part2(&parse(include_str!("../test_input/day04.part2.30.txt")).unwrap()));
    }

    #[test]
    fn part2_input() {
        assert_eq!(5329815, part2(&parse(include_str!("../input/2023/day4.txt")).unwrap()));
    }
}
