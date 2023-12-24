use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use nalgebra::{Matrix2, Vector2};

type Input = Vec<(isize, isize, isize, isize, isize, isize)>;

#[aoc_generator(day24)]
fn parse(input: &str) -> Input {
    input
        .lines()
        .flat_map(|line| line.split(&[',', ' ', '@']).filter_map(|s| s.parse::<isize>().ok()).tuples())
        .collect()
}

#[aoc(day24, part1)]
fn part1(input: &Input) -> usize {
    solve(input, (200000000000000, 400000000000000))
}

fn solve(input: &Input, limits: (usize, usize)) -> usize {
    (0..input.len()).tuple_combinations()
        .filter(|&(i, j)| {
            let a = Matrix2::new(
                input[i].3 as f64, -input[j].3 as f64,
                input[i].4 as f64, -input[j].4 as f64,
            );
            let b = Vector2::new(input[j].0 as f64 - input[i].0 as f64, input[j].1 as f64 - input[i].1 as f64);
            let Some(solution) = a.lu().solve(&b) else { return false };

            let x = input[i].0 as f64 + input[i].3 as f64 * solution[0];
            let y = input[i].1 as f64 + input[i].4 as f64 * solution[0];

            solution[0] >= 0.0 && solution[1] >= 0.0 && ((limits.0 as f64)..=(limits.1 as f64)).contains(&x) && ((limits.0 as f64)..=(limits.1 as f64)).contains(&y)
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        assert_eq!(2, solve(&parse(include_str!("../test_input/day24.example1.txt")), (7, 27)));
    }

    #[test]
    fn part1_input() {
        assert_eq!(17906, part1(&parse(include_str!("../input/2023/day24.txt"))));
    }
}
