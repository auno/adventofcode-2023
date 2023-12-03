use std::collections::HashMap;
use aoc_runner_derive::{aoc, aoc_generator};
use scan_fmt::scan_fmt;

type Schematic = (Vec<((isize, isize), u32, usize)>, HashMap<(isize, isize), char>);

#[aoc_generator(day3)]
fn parse(input: &str) -> Schematic {
    let mut numbers = vec![];
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

                    numbers.push(((j as isize, i as isize), number, num_digits));

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

#[aoc(day3, part1)]
fn part1(schematic: &Schematic) -> u32 {
    let (numbers, symbols) = schematic;

    numbers
        .iter()
        .filter(|((j, i), _, num_digits)| {
            (0..(*num_digits as isize))
                .flat_map(|k| [
                    (*j, (i+k)-1),
                    (*j, (i+k)+1),
                    (j-1, (i+k)),
                    (j+1, (i+k)),
                    (j-1, (i+k)-1),
                    (j-1, (i+k)+1),
                    (j+1, (i+k)-1),
                    (j+1, (i+k)+1)
                ])
                .any(|neighbor| symbols.contains_key(&neighbor))
        })
        .map(|(_, number, _)| number)
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
}
