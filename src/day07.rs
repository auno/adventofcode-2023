use std::fmt::{Display, Formatter};
use std::str::FromStr;
use aoc_runner_derive::{aoc, aoc_generator};
use anyhow::{Context, Result, Error, bail};
use itertools::Itertools;

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Copy, Clone)]
enum Card {
    Num2,
    Num3,
    Num4,
    Num5,
    Num6,
    Num7,
    Num8,
    Num9,
    T,
    J,
    Q,
    K,
    A,
}

impl TryFrom<char> for Card {
    type Error = Error;

    fn try_from(value: char) -> std::result::Result<Self, Self::Error> {
        match value {
            '2' => Ok(Card::Num2),
            '3' => Ok(Card::Num3),
            '4' => Ok(Card::Num4),
            '5' => Ok(Card::Num5),
            '6' => Ok(Card::Num6),
            '7' => Ok(Card::Num7),
            '8' => Ok(Card::Num8),
            '9' => Ok(Card::Num9),
            'T' => Ok(Card::T),
            'J' => Ok(Card::J),
            'Q' => Ok(Card::Q),
            'K' => Ok(Card::K),
            'A' => Ok(Card::A),
            _ => bail!("Invalid card: {value}"),
        }
    }
}

#[derive(Ord, PartialOrd, Eq, PartialEq)]
struct Hand {
    hand_type: u8,
    cards: [Card; 5],
    string_representation: String,
}

impl FromStr for Hand {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        if s.len() != 5 {
            bail!("Invalid hand: {s}");
        }

        let cards: [Card; 5] = s
            .chars()
            .map(Card::try_from)
            .collect::<Result<Vec<Card>>>()?
            .try_into()
            .unwrap();

        let card_type_counts = cards
            .into_iter()
            .sorted()
            .rev()
            .group_by(|c| *c)
            .into_iter()
            .map(|(_, g)| g.count())
            .sorted()
            .rev()
            .collect_vec();

        let hand_type = match &card_type_counts[..] {
            [5] => 6,
            [4, 1] => 5,
            [3, 2] => 4,
            [3, 1, 1] => 3,
            [2, 2, 1] => 2,
            [2, 1, 1, 1] => 1,
            [1, 1, 1, 1, 1] => 0,
            _ => unreachable!()
        };

        let string_representation = s.to_string();

        Ok(Hand { hand_type, cards, string_representation })
    }
}

impl Display for Hand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.string_representation)
    }
}

#[aoc_generator(day7)]
fn parse(input: &str) -> Result<Vec<(Hand, u32)>> {
    input
        .lines()
        .map(|line| {
            let (hand, bid) = line.split_once(' ').context(format!("Input line invalid: {line}"))?;
            Ok((
                hand.parse()?,
                bid.parse()?,
            ))
        })
        .collect()
}

#[aoc(day7, part1)]
fn part1(input: &[(Hand, u32)]) -> u32 {
    input
        .iter()
        .sorted_by_key(|(hand, _)| hand)
        .enumerate()
        .map(|(rank, (_, bid))| (rank + 1) as u32 * bid)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        assert_eq!(6440, part1(&parse(include_str!("../test_input/day07.part1.6440.txt")).unwrap()));
    }

    #[test]
    fn part1_input() {
        assert_eq!(248179786, part1(&parse(include_str!("../input/2023/day7.txt")).unwrap()));
    }
}
