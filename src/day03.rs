use std::collections::HashMap;
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use scan_fmt::scan_fmt;

type Schematic = (HashMap<(isize, isize), ((isize, isize), u32, usize)>, HashMap<(isize, isize), char>);

#[aoc_generator(day3)]
fn parse(input: &str) -> Schematic {
    let mut numbers = HashMap::new();
    let mut symbols = HashMap::new();

    for (j, line) in input.lines().enumerate() {
        let mut i = 0;

        while i < line.len() {
            match line.chars().nth(i).unwrap() {
                '.' => {
                    i += 1;
                },
                '0'..='9' => {
                    let number = scan_fmt!(&line[i..], "{d}", String).unwrap();
                    let num_digits = number.len();
                    let number = number.parse::<u32>().unwrap();

                    for k in 0..(num_digits) {
                        numbers.insert((j as isize, (i+ k) as isize), ((j as isize, i as isize), number, num_digits));
                    }

                    i += num_digits;
                },
                symbol => {
                    symbols.insert((j as isize, i as isize), symbol);
                    i += 1;
                },
            }
        }
    }

    (numbers, symbols)
}

fn neighbors(j: isize, i: isize) -> [(isize, isize); 8] {
    [
        (j, i - 1),
        (j, i + 1),
        (j - 1, i),
        (j + 1, i),
        (j - 1, i - 1),
        (j - 1, i + 1),
        (j + 1, i - 1),
        (j + 1, i + 1)
    ]
}

#[aoc(day3, part1)]
fn part1(schematic: &Schematic) -> u32 {
    let (numbers, symbols) = schematic;

    numbers
        .values()
        .unique()
        .filter(|((j, i), _, num_digits)| {
            (0..(*num_digits as isize))
                .flat_map(|k| neighbors(*j, i + k))
                .any(|neighbor| symbols.contains_key(&neighbor))
        })
        .map(|(_, number, _)| number)
        .sum()
}

#[aoc(day3, part2)]
fn part2(schematic: &Schematic) -> u32 {
    let (numbers, symbols) = schematic;

    symbols
        .iter()
        .filter(|&(_, symbol)| *symbol == '*')
        .map(|((j, i), _)| {
            let neighboring_numbers = neighbors(*j, *i)
                .iter()
                .filter_map(|neighbor| numbers.get(neighbor))
                .unique()
                .collect_vec();

            if neighboring_numbers.len() != 2 {
                return 0;
            }

            neighboring_numbers
                .iter()
                .map(|(_, number, _)| number)
                .product::<u32>()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        assert_eq!(4361, part1(&parse(include_str!("../test_input/day03.part1.4361.txt"))));
    }

    #[test]
    fn part1_input() {
        assert_eq!(539713, part1(&parse(include_str!("../input/2023/day3.txt"))));
    }

    #[test]
    fn part2_example1() {
        assert_eq!(467835, part2(&parse(include_str!("../test_input/day03.part2.467835.txt"))));
    }

    #[test]
    fn part2_input() {
        assert_eq!(84159075, part2(&parse(include_str!("../input/2023/day3.txt"))));
    }
}
