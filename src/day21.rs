use std::cmp::{min, Reverse};
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::hash::Hash;
use aoc_runner_derive::{aoc, aoc_generator};
use anyhow::{Context, Result};

type Input = (usize, usize, (usize, usize), HashSet<(usize, usize)>);

#[aoc_generator(day21)]
fn parse(input: &str) -> Result<Input> {
    let height = input.lines().count();
    let width = input.lines().next().context("Unexpected empty line")?.len();
    let mut rocks = HashSet::new();
    let mut starting_position = None;

    for (i, c) in input.chars().filter(|c| !c.is_ascii_whitespace()).enumerate() {
        let position = ((i / width), (i % width));

        if c == 'S' {
            starting_position = Some(position);
        }

        if c == '#' {
            rocks.insert(position);
        }
    }

    Ok((height, width, starting_position.context("Starting position not found")?, rocks))
}

type State = (usize, usize);

fn distances<SearchState: Ord + Hash + Copy, IntoNeighborIterator: IntoIterator<Item=(SearchState, usize)>>(
    source: SearchState,
    neighbors: impl Fn(SearchState) -> IntoNeighborIterator,
) -> impl IntoIterator<Item=(SearchState, usize)> {
    let mut distances: HashMap<SearchState, usize> = HashMap::new();
    let mut queue: BinaryHeap<(Reverse<usize>, SearchState)> = BinaryHeap::new();

    distances.insert(source, 0);
    queue.push((Reverse(0), source));

    while let Some((Reverse(current_distance), state)) = queue.pop() {
        for (neighbor, neighbor_increment) in neighbors(state) {
            let best_neighbor_distance = distances.entry(neighbor).or_insert(usize::MAX);

            if *best_neighbor_distance > current_distance + neighbor_increment {
                *best_neighbor_distance = current_distance + neighbor_increment;
                queue.push((Reverse(*best_neighbor_distance), neighbor));
            }
        }
    }

    distances
}

fn solve((height, width, starting_position, rocks): &Input, steps: usize) -> usize {
    let neighbors = |(j, i): State| {
        [(j.overflowing_sub(1).0, i), (j + 1, i), (j, i.overflowing_sub(1).0), (j, i + 1)]
            .into_iter()
            .filter(|position| !rocks.contains(position))
            .filter(|(j, i)| j < height && i < width)
            .map(|neighbor_position| (neighbor_position, 1usize))
    };

    distances(*starting_position, neighbors)
        .into_iter()
        .filter(|(_, distance)| *distance <= steps && (distance % 2) == (steps % 2))
        .count()
}

#[aoc(day21, part1)]
fn part1(input: &Input) -> usize {
    solve(input, 64)
}

#[aoc(day21, part2)]
fn part2(input: &Input) -> usize {
    let (height, width, starting_position, rocks) = input;
    let steps = 26501365;
    let mut count = 0;

    let neighbors = |(j, i): State| {
        [(j.overflowing_sub(1).0, i), (j + 1, i), (j, i.overflowing_sub(1).0), (j, i + 1)]
            .into_iter()
            .filter(|position| !rocks.contains(position))
            .filter(|(j, i)| j < height && i < width)
            .map(|neighbor_position| (neighbor_position, 1usize))
    };

    let max_num_reachable = [
        distances(*starting_position, neighbors).into_iter().filter(|(_, distance)| (distance % 2) == 0).count(),
        distances(*starting_position, neighbors).into_iter().filter(|(_, distance)| (distance % 2) == 1).count(),
    ];

    let max_num_steps_from_entry_point = [
        (0, 0), (0, starting_position.1), (0, width - 1),
        (starting_position.0, 0), (starting_position.0, width - 1),
        (height - 1, 0), (height - 1, starting_position.1), (height - 1, width - 1),
    ]
        .into_iter()
        .flat_map(|p| distances(p, neighbors))
        .map(|(_, d)| d)
        .max()
        .unwrap();

    let mut cache = HashMap::new();

    // For each level up and down
    for level in 0.. {
        let steps_taken_at_level_center_entry_point = starting_position.0 + 1 + level * height;

        if steps_taken_at_level_center_entry_point > steps {
            break;
        }

        let steps_left_at_level_center_entry_point = steps - steps_taken_at_level_center_entry_point;

        // Add all grids to the sides
        let steps_taken_at_first_grid_to_the_side = steps_taken_at_level_center_entry_point + starting_position.1 + 1;
        let steps_left_at_first_grid_to_the_side = steps - steps_taken_at_first_grid_to_the_side;
        let num_fully_reachable_grids_to_the_side = (((steps_left_at_first_grid_to_the_side.saturating_sub(max_num_steps_from_entry_point)) / width) / 2) * 2; // Divide then multiply by two to make even

        count += num_fully_reachable_grids_to_the_side / 2 * (max_num_reachable[0] + max_num_reachable[1]) * 4;

        for grid_offset_from_first_grid_to_the_side in num_fully_reachable_grids_to_the_side.. {
            let steps_taken_at_grid_entry_point = steps_taken_at_first_grid_to_the_side + grid_offset_from_first_grid_to_the_side * width;

            if steps_taken_at_grid_entry_point > steps {
                break;
            }

            let steps_left_at_grid_entry_point = steps - steps_taken_at_grid_entry_point;

            // Up left
            count += *cache.entry(((height - 1, width - 1), min(steps_left_at_grid_entry_point, max_num_steps_from_entry_point), steps_left_at_grid_entry_point % 2)).or_insert_with(|| {
                distances((height - 1, width - 1), neighbors)
                    .into_iter()
                    .filter(|(_, distance)| *distance <= steps_left_at_grid_entry_point && ((steps_taken_at_grid_entry_point + distance) % 2) == (steps % 2))
                    .count()
            });

            // Up right
            count += *cache.entry(((height - 1, 0), min(steps_left_at_grid_entry_point, max_num_steps_from_entry_point), steps_left_at_grid_entry_point % 2)).or_insert_with(|| {
                distances((height - 1, 0), neighbors)
                    .into_iter()
                    .filter(|(_, distance)| *distance <= steps_left_at_grid_entry_point && ((steps_taken_at_grid_entry_point + distance) % 2) == (steps % 2))
                    .count()
            });

            // Down left
            count += *cache.entry(((0, width - 1), min(steps_left_at_grid_entry_point, max_num_steps_from_entry_point), steps_left_at_grid_entry_point % 2)).or_insert_with(|| {
                distances((0, width - 1), neighbors)
                    .into_iter()
                    .filter(|(_, distance)| *distance <= steps_left_at_grid_entry_point && ((steps_taken_at_grid_entry_point + distance) % 2) == (steps % 2))
                    .count()
            });

            // Down right
            count += *cache.entry(((0, 0), min(steps_left_at_grid_entry_point, max_num_steps_from_entry_point), steps_left_at_grid_entry_point % 2)).or_insert_with(|| {
                distances((0, 0), neighbors)
                    .into_iter()
                    .filter(|(_, distance)| *distance <= steps_left_at_grid_entry_point && ((steps_taken_at_grid_entry_point + distance) % 2) == (steps % 2))
                    .count()
            });
        }

        // Up center
        count += *cache.entry(((height - 1, starting_position.1), min(steps_left_at_level_center_entry_point, max_num_steps_from_entry_point), steps_left_at_level_center_entry_point % 2)).or_insert_with(|| {
            distances((height - 1, starting_position.1), neighbors)
                .into_iter()
                .filter(|(_, distance)| *distance <= steps_left_at_level_center_entry_point && ((steps_taken_at_level_center_entry_point + distance) % 2) == (steps % 2))
                .count()
        });

        // Down center
        count += *cache.entry(((0, starting_position.1), min(steps_left_at_level_center_entry_point, max_num_steps_from_entry_point), steps_left_at_level_center_entry_point % 2)).or_insert_with(|| {
            distances((0, starting_position.1), neighbors)
                .into_iter()
                .filter(|(_, distance)| *distance <= steps_left_at_level_center_entry_point && ((steps_taken_at_level_center_entry_point + distance) % 2) == (steps % 2))
                .count()
        });

        if steps_left_at_level_center_entry_point <= *height {
            break;
        }
    }

    // Center level

    let steps_taken_at_first_grid_to_the_side = starting_position.1 + 1;
    let steps_left_at_first_grid_to_the_side = steps - steps_taken_at_first_grid_to_the_side;
    let num_fully_reachable_grids_to_the_side = (((steps_left_at_first_grid_to_the_side.saturating_sub(max_num_steps_from_entry_point)) / width) / 2) * 2; // Divide then multiply by two to make even

    count += num_fully_reachable_grids_to_the_side / 2 * (max_num_reachable[0] + max_num_reachable[1]) * 2;

    for grid_offset_from_first_grid_to_the_side in num_fully_reachable_grids_to_the_side.. {
        let steps_taken_at_grid_entry_point = steps_taken_at_first_grid_to_the_side + grid_offset_from_first_grid_to_the_side * width;

        if steps_taken_at_grid_entry_point > steps {
            break;
        }

        let steps_left_at_grid_entry_point = steps - steps_taken_at_grid_entry_point;

        // Center left
        count += *cache.entry(((starting_position.0, width - 1), min(steps_left_at_grid_entry_point, max_num_steps_from_entry_point), steps_left_at_grid_entry_point % 2)).or_insert_with(|| {
            distances((starting_position.0, width - 1), neighbors)
                .into_iter()
                .filter(|(_, distance)| *distance <= steps_left_at_grid_entry_point && ((steps_taken_at_grid_entry_point + distance) % 2) == (steps % 2))
                .count()
        });


        // Center right
        count += *cache.entry(((starting_position.0, 0), min(steps_left_at_grid_entry_point, max_num_steps_from_entry_point), steps_left_at_grid_entry_point % 2)).or_insert_with(|| {
            distances((starting_position.0, 0), neighbors)
                .into_iter()
                .filter(|(_, distance)| *distance <= steps_left_at_grid_entry_point && ((steps_taken_at_grid_entry_point + distance) % 2) == (steps % 2))
                .count()
        });
    }

    // Center center
    count += max_num_reachable[steps % 2];

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_example_6() {
        assert_eq!(16, solve(&parse(include_str!("../test_input/day21.example1.txt")).unwrap(), 6));
    }

    #[test]
    fn part1_input() {
        assert_eq!(3764, part1(&parse(include_str!("../input/2023/day21.txt")).unwrap()));
    }

    #[test]
    fn part2_input() {
        assert_eq!(622926941971282, part2(&parse(include_str!("../input/2023/day21.txt")).unwrap()));
    }
}
