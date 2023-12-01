use std::collections::HashMap;
use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
fn parse(input: &str) -> Vec<String> {
    input
        .lines()
        .map(|line| line.to_string())
        .collect()
}

#[aoc(day1, part1)]
fn part1(input: &[String]) -> u32 {
    input
        .iter()
        .map(|line| {
            line.chars()
                .filter(|c| c.is_ascii_digit())
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .map(|digits| digits.first().unwrap() * 10 + digits.last().unwrap())
        .sum()
}

#[aoc(day1, part2)]
fn part2(input: &[String]) -> u32 {
    let digits_spelled_out = HashMap::from([
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ]);

    input
        .iter()
        .map(|line| {
            let mut normalized_line = String::new();
            let mut line = line.as_str();

            while line.len() > 0 {
                match digits_spelled_out.iter().find(|(&dso, &d)| line.starts_with(dso)) {
                    Some((&dso, &d)) => {
                        normalized_line.push_str(d);
                        line = &line[1..];
                    }
                    None => {
                        normalized_line.push_str(&line[0..1]);
                        line = &line[1..];
                    }
                }
            }

            normalized_line
        })
        .map(|line| {
            line.chars()
                .filter(|c| c.is_ascii_digit())
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .map(|digits| digits.first().unwrap() * 10 + digits.last().unwrap())
        .map(|x| { eprintln!("{x}"); x })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        assert_eq!(
            142,
            part1(&parse(include_str!("../test_input/day1.part1.142.txt")))
        );
    }

    #[test]
    fn part2_example1() {
        assert_eq!(
            281,
            part2(&parse(include_str!("../test_input/day1.part2.281.txt")))
        );
    }
}
