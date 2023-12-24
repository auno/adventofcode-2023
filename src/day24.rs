use std::ops::{Add, Mul};
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use nalgebra::{Matrix2, Vector2};
use z3::ast::{Ast, Int};

type Input = Vec<(i64, i64, i64, i64, i64, i64)>;

#[aoc_generator(day24)]
fn parse(input: &str) -> Input {
    input
        .lines()
        .flat_map(|line| line.split(&[',', ' ', '@']).filter_map(|s| s.parse::<i64>().ok()).tuples())
        .collect()
}

#[aoc(day24, part1)]
fn part1(input: &Input) -> usize {
    solve1(input, (200000000000000, 400000000000000))
}

fn solve1(input: &Input, limits: (usize, usize)) -> usize {
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

#[aoc(day24, part2)]
fn part2(input: &Input) -> i64 {
    let z3_conf = z3::Config::new();
    let ctx = z3::Context::new(&z3_conf);
    let solver = z3::Solver::new(&ctx);

    let m1 = Int::new_const(&ctx, "m1");
    let m2 = Int::new_const(&ctx, "m2");
    let m3 = Int::new_const(&ctx, "m3");
    let k1 = Int::new_const(&ctx, "k1");
    let k2 = Int::new_const(&ctx, "k2");
    let k3 = Int::new_const(&ctx, "k3");
    let ta = Int::new_const(&ctx, "ta");
    let tb = Int::new_const(&ctx, "tb");
    let tc = Int::new_const(&ctx, "tc");

    solver.assert(&Int::from_i64(&ctx, input[0].0).add(Int::from_i64(&ctx, input[0].3).mul(&ta))._eq(&(&m1).add((&k1).mul(&ta))));
    solver.assert(&Int::from_i64(&ctx, input[0].1).add(Int::from_i64(&ctx, input[0].4).mul(&ta))._eq(&(&m2).add((&k2).mul(&ta))));
    solver.assert(&Int::from_i64(&ctx, input[0].2).add(Int::from_i64(&ctx, input[0].5).mul(&ta))._eq(&(&m3).add((&k3).mul(&ta))));

    solver.assert(&Int::from_i64(&ctx, input[1].0).add(Int::from_i64(&ctx, input[1].3).mul(&tb))._eq(&(&m1).add((&k1).mul(&tb))));
    solver.assert(&Int::from_i64(&ctx, input[1].1).add(Int::from_i64(&ctx, input[1].4).mul(&tb))._eq(&(&m2).add((&k2).mul(&tb))));
    solver.assert(&Int::from_i64(&ctx, input[1].2).add(Int::from_i64(&ctx, input[1].5).mul(&tb))._eq(&(&m3).add((&k3).mul(&tb))));

    solver.assert(&Int::from_i64(&ctx, input[2].0).add(Int::from_i64(&ctx, input[2].3).mul(&tc))._eq(&(&m1).add((&k1).mul(&tc))));
    solver.assert(&Int::from_i64(&ctx, input[2].1).add(Int::from_i64(&ctx, input[2].4).mul(&tc))._eq(&(&m2).add((&k2).mul(&tc))));
    solver.assert(&Int::from_i64(&ctx, input[2].2).add(Int::from_i64(&ctx, input[2].5).mul(&tc))._eq(&(&m3).add((&k3).mul(&tc))));

    solver.check();

    let model = solver.get_model().unwrap();

    let m1v = model.eval(&m1, true).unwrap().as_i64().unwrap();
    let m2v = model.eval(&m2, true).unwrap().as_i64().unwrap();
    let m3v = model.eval(&m3, true).unwrap().as_i64().unwrap();

    m1v + m2v + m3v
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        assert_eq!(2, solve1(&parse(include_str!("../test_input/day24.example1.txt")), (7, 27)));
    }

    #[test]
    fn part1_input() {
        assert_eq!(17906, part1(&parse(include_str!("../input/2023/day24.txt"))));
    }

    #[test]
    fn part2_example1() {
        assert_eq!(47, part2(&parse(include_str!("../test_input/day24.example1.txt"))));
    }

    #[test]
    fn part2_input() {
        assert_eq!(571093786416929, part2(&parse(include_str!("../input/2023/day24.txt"))));
    }
}
