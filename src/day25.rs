use std::collections::{HashMap, HashSet, VecDeque};
use aoc_runner_derive::{aoc, aoc_generator};

type Input = Vec<(String, String)>;

#[aoc_generator(day25)]
fn parse(input: &str) -> Input {
    input
        .lines()
        .flat_map(|line| {
            let (a, bs) = line.split_once(": ").unwrap();
            bs.split_ascii_whitespace().map(|b| (a.to_string(), b.to_string()))
        })
        .collect()
}

fn two_component_lens<'a>(edges: impl IntoIterator<Item = &'a (String, String)>) -> (usize, usize) {
    let mut connections: HashMap<&str, Vec<&str>> = HashMap::new();

    for (a, b) in edges {
        connections.entry(a).or_insert_with(Vec::new).push(b);
        connections.entry(b).or_insert_with(Vec::new).push(a);
    }

    let source = connections.keys().next().unwrap();

    let mut queue = VecDeque::from([source]);
    let mut seen = HashSet::from([source]);

    while let Some(current) = queue.pop_front() {
        for neighbor in connections.get(current).unwrap() {
            if seen.contains(neighbor) {
                continue
            }

            queue.push_back(neighbor);
            seen.insert(neighbor);
        }
    }

    (seen.len(), connections.keys().len() - seen.len())
}

#[aoc(day25, part1)]
fn part1(input: &Input) -> usize {
    let edges_to_be_removed = [
        ("sxx", "zvk"),
        ("njx", "pbx"),
        ("pzr", "sss"),
    ];

    let pruned_edges = input
        .iter()
        .filter(|(a, b)| !(edges_to_be_removed.contains(&(a, b)) || edges_to_be_removed.contains(&(b, a))));

    let (a, b) = two_component_lens(pruned_edges);

    a * b
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn part1_example1() {
    //     assert_eq!(54, part1(&parse(include_str!("../test_input/day25.example1.txt"))).unwrap());
    // }

    #[test]
    fn part1_input() {
        assert_eq!(582590, part1(&parse(include_str!("../input/2023/day25.txt"))));
    }
}
