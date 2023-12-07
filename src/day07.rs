use std::fmt::{Display, Formatter};
use std::str::FromStr;
use aoc_runner_derive::{aoc, aoc_generator};
use anyhow::{Context, Result, Error, bail};
use itertools::Itertools;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Copy, Clone, EnumIter)]
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
enum HandType {
    HighCard,
    OnePair,
    TwoPairs,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl From<[Card; 5]> for HandType {
    fn from(cards: [Card; 5]) -> Self {
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

        match &card_type_counts[..] {
            [5] => HandType::FiveOfAKind,
            [4, 1] => HandType::FourOfAKind,
            [3, 2] => HandType::FullHouse,
            [3, 1, 1] => HandType::ThreeOfAKind,
            [2, 2, 1] => HandType::TwoPairs,
            [2, 1, 1, 1] => HandType::OnePair,
            [1, 1, 1, 1, 1] => HandType::HighCard,
            _ => unreachable!()
        }
    }
}

#[derive(Ord, PartialOrd, Eq, PartialEq)]
struct Hand {
    hand_type: HandType,
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

        let hand_type = cards.into();
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

#[aoc(day7, part2)]
fn part2(input: &[(Hand, u32)]) -> u32 {
    input
        .iter()
        .sorted_by_key(|(hand, _)| {
            let hand_type = Card::iter()
                .map(|substitute| {
                    let mut cards = hand.cards;

                    for card in &mut cards {
                        if *card == Card::J {
                            *card = substitute;
                        }
                    }

                    HandType::from(cards)
                })
                .max();

            let cards_proxy: [u8; 5] = hand.cards
                .into_iter()
                .map(|card| match card {
                    Card::J => 0,
                    Card::Num2 => 1,
                    Card::Num3 => 2,
                    Card::Num4 => 3,
                    Card::Num5 => 4,
                    Card::Num6 => 5,
                    Card::Num7 => 6,
                    Card::Num8 => 7,
                    Card::Num9 => 8,
                    Card::T => 9,
                    Card::Q => 10,
                    Card::K => 11,
                    Card::A => 12,
                })
                .collect_vec()
                .try_into()
                .unwrap();

            (hand_type, cards_proxy)
        })
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

    #[test]
    fn part2_example1() {
        assert_eq!(5905, part2(&parse(include_str!("../test_input/day07.part2.5905.txt")).unwrap()));
    }

    #[test]
    fn part2_input() {
        assert_eq!(247885995, part2(&parse(include_str!("../input/2023/day7.txt")).unwrap()));
    }
}
