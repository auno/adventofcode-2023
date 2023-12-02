use aoc_runner_derive::{aoc, aoc_generator};
use anyhow::{Context, Result};
use scan_fmt::scan_fmt;

type Game = (u32, Vec<(u32, u32, u32)>);

#[aoc_generator(day2)]
fn parse(input: &str) -> Result<Vec<Game>> {
    input
        .lines()
        .map(|line| {
            let (game, reveals) = line.split_once(": ").context(format!("Unable to parse line: {}", line))?;
            let game_id = scan_fmt!(line, "Game {d}", u32)?;
            let reveals = reveals
                .split("; ")
                .map(|reveal| {
                    reveal
                        .split(", ")
                        .map(|count_color| {
                            let (count, color) = scan_fmt!(count_color, "{d} {}", u32, String).unwrap();
                            match color.as_str() {
                                "red" => (count, 0, 0),
                                "green" => (0, count, 0),
                                "blue" => (0, 0, count),
                                _ => panic!("Unknown color: {}", color),
                            }
                        })
                        .fold((0, 0, 0), |(a1, a2, a3), (b1, b2, b3)| (a1 + b1, a2 + b2, a3 + b3))
                })
                .collect();

            Ok((game_id, reveals))
        })
        .collect()
}

#[aoc(day2, part1)]
fn part1(games: &Vec<Game>) -> u32 {
    games
        .iter()
        .filter(|(_, reveals)| reveals.iter().all(|(r, g, b)| *r <= 12 && *g <= 13 && *b <= 14))
        .map(|(game_id, _)| game_id)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        assert_eq!(8, part1(&parse(include_str!("../test_input/day02.part1.8.txt")).unwrap()));
    }
}
