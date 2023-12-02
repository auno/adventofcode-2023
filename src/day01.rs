use aoc_runner_derive::{aoc, aoc_generator};
use phf::phf_map;

static DIGITS_SPELLED_OUT: phf::Map<&'static str, char> = phf_map! {
    "one" => '1',
    "two" => '2',
    "three" => '3',
    "four" => '4',
    "five" => '5',
    "six" => '6',
    "seven" => '7',
    "eight" => '8',
    "nine" => '9',
};

#[aoc_generator(day1)]
fn parse(input: &str) -> Vec<String> {
    input
        .lines()
        .map(|line| line.to_string())
        .collect()
}

fn solve(input: &[String], normalizer: fn(&String) -> String) -> u32 {
    input
        .iter()
        .map(normalizer)
        .map(|line| {
            line.chars()
                .filter(|c| c.is_ascii_digit())
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .map(|digits| digits.first().unwrap() * 10 + digits.last().unwrap())
        .sum()
}

#[aoc(day1, part1)]
fn part1(input: &[String]) -> u32 {
    solve(input, String::to_owned)
}

#[aoc(day1, part2)]
fn part2(input: &[String]) -> u32 {
    let normalizer = |line: &String| {
        let normalized_line = (0..line.len())
            .map(|i| {
                match DIGITS_SPELLED_OUT.entries().find(|(&dso, _)| line[i..].starts_with(dso)) {
                    Some((_, &d)) => d,
                    None => line[i..].chars().next().unwrap(),
                }
            })
            .collect::<String>();

        normalized_line
    };

    solve(input, normalizer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        assert_eq!(142, part1(&parse(include_str!("../test_input/day01.part1.142.txt"))));
    }

    #[test]
    fn part1_input() {
        assert_eq!(53921, part1(&parse(include_str!("../input/2023/day1.txt"))));
    }

    #[test]
    fn part2_example1() {
        assert_eq!(281, part2(&parse(include_str!("../test_input/day01.part2.281.txt"))));
    }

    #[test]
    fn part2_overlapping() {
        assert_eq!(82, part2(&parse("eightwo")));
        assert_eq!(98, part2(&parse("nineight")));
    }

    #[test]
    fn part2_input() {
        assert_eq!(54676, part2(&parse(include_str!("../input/2023/day1.txt"))));
    }
}
