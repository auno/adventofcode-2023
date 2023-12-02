use aoc_runner_derive::{aoc, aoc_generator};
use anyhow::{Context, Result, bail};
use scan_fmt::scan_fmt;
use std::cmp::max;
use itertools::Itertools;

type Reveals = Vec<(u32, u32, u32)>;
type Game = (u32, Reveals);

#[aoc_generator(day2)]
fn parse(input: &str) -> Result<Vec<Game>> {
    input
        .lines()
        .map(|line| {
            let (game, reveals) = line.split_once(": ").context(format!("Unable to parse line: {}", line))?;
            let game_id = scan_fmt!(game, "Game {d}", u32)?;
            let reveals = reveals
                .split("; ")
                .map(|reveal| {
                    reveal
                        .split(", ")
                        .map(|count_color| {
                            let (count, color) = scan_fmt!(count_color, "{d} {}", u32, String)?;
                            Ok(
                                match color.as_str() {
                                    "red" => (count, 0, 0),
                                    "green" => (0, count, 0),
                                    "blue" => (0, 0, count),
                                    _ => bail!("Unknown color: {}", color),
                                }
                            )
                        })
                        .fold_ok((0, 0, 0), |(r1, g1, b1), (r2, g2, b2)| (r1 + r2, g1 + g2, b1 + b2))
                })
                .collect::<Result<Reveals>>()?;

            Ok((game_id, reveals))
        })
        .collect()
}

#[aoc(day2, part1)]
fn part1(games: &[Game]) -> u32 {
    games
        .iter()
        .filter(|(_, reveals)| reveals.iter().all(|&(r, g, b)| r <= 12 && g <= 13 && b <= 14))
        .map(|(game_id, _)| game_id)
        .sum()
}

#[aoc(day2, part2)]
fn part2(games: &[Game]) -> u32 {
    games
        .iter()
        .map(|(_, reveals)| {
            let (r, g, b) = reveals
                .iter()
                .fold((0, 0, 0), |(r1, g1, b1), &(r2, g2, b2)| (max(r1, r2), max(g1, g2), max(b1, b2)));
            r * g * b
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        assert_eq!(8, part1(&parse(include_str!("../test_input/day02.part1.8.txt")).unwrap()));
    }

    #[test]
    fn part1_input() {
        assert_eq!(2679, part1(&parse(include_str!("../input/2023/day2.txt")).unwrap()));
    }

    #[test]
    fn part2_example1() {
        assert_eq!(2286, part2(&parse(include_str!("../test_input/day02.part2.2286.txt")).unwrap()));
    }

    #[test]
    fn part2_input() {
        assert_eq!(77607, part2(&parse(include_str!("../input/2023/day2.txt")).unwrap()));
    }
}
