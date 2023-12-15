use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day15)]
fn parse(input: &str) -> Vec<String> {
    input
        .trim()
        .split(',')
        .map(|step| step.to_string())
        .collect()
}

fn hash(s: &str) -> u32 {
    s.chars().fold(0, |acc, c| ((acc + c as u32) * 17) % 256)
}

#[aoc(day15, part1)]
fn part1(input: &[String]) -> u32 {
    input
        .iter()
        .map(String::as_str)
        .map(hash)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        assert_eq!(1320, part1(&parse(include_str!("../test_input/day15.part1.1320.txt"))));
    }

    #[test]
    fn part1_input() {
        assert_eq!(505379, part1(&parse(include_str!("../input/2023/day15.txt"))));
    }
}
