use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
fn parse(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .filter(|c| c.is_ascii_digit())
                .map(|c| c.to_digit(10).unwrap())
                .collect()
        })
        .collect()
}

#[aoc(day1, part1)]
fn part1(input: &[Vec<u32>]) -> u32 {
    input
        .iter()
        .map(|digits| digits.first().unwrap() * 10 + digits.last().unwrap())
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
}
