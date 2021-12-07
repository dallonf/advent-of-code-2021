use std::ops::RangeInclusive;

// Day 7: The Treachery of Whales
use crate::prelude::*;

lazy_static! {
    static ref PUZZLE_INPUT: Box<[isize]> = include_str!("day07_input.txt")
        .trim()
        .split(",")
        .map(|it| it.parse().unwrap())
        .collect();
}

pub fn part_one() -> usize {
    cheapest_alignment(&PUZZLE_INPUT).fuel_required
}

pub fn part_two() -> usize {
    cheapest_alignment_mk2(&PUZZLE_INPUT).fuel_required
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Alignment {
    fuel_required: usize,
    position: isize,
}

// Panics if an empty list is provided
fn cheapest_alignment(crabs: &[isize]) -> Alignment {
    let min = *crabs.iter().min().unwrap();
    let max = *crabs.iter().max().unwrap();
    RangeInclusive::new(min, max)
        .map(|position| Alignment {
            fuel_required: fuel_to_move_to_position(crabs, position),
            position,
        })
        .min_by_key(|alignment| alignment.fuel_required)
        .unwrap()
}

fn fuel_to_move_to_position(crabs: &[isize], position: isize) -> usize {
    crabs
        .into_iter()
        .map(|&crab| (position - crab).abs() as usize)
        .sum()
}

fn cheapest_alignment_mk2(crabs: &[isize]) -> Alignment {
    let min = *crabs.iter().min().unwrap();
    let max = *crabs.iter().max().unwrap();

    let fuel_costs = {
        let greatest_distance = (max - min).abs() as usize;
        let mut fuel_costs: Box<[usize]> = RangeInclusive::new(0, greatest_distance)
            .map(|_| 0)
            .collect();
        for i in RangeInclusive::new(1, greatest_distance) {
            fuel_costs[i] = fuel_costs[i - 1] + i;
        }
        fuel_costs
    };

    RangeInclusive::new(min, max)
        .map(|position| Alignment {
            fuel_required: fuel_to_move_to_position_mk2(crabs, &fuel_costs, position),
            position,
        })
        .min_by_key(|alignment| alignment.fuel_required)
        .unwrap()
}

fn fuel_to_move_to_position_mk2(crabs: &[isize], fuel_costs: &[usize], position: isize) -> usize {
    crabs
        .into_iter()
        .map(|&crab| fuel_costs[(position - crab).abs() as usize])
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: [isize; 10] = [16, 1, 2, 0, 4, 2, 7, 1, 2, 14];

    #[test]
    fn part_one_answer() {
        let result = part_one();
        assert_eq!(result, 349812);
    }

    #[test]
    fn test_cheapest_alignment() {
        let result = cheapest_alignment(&EXAMPLE_INPUT);
        assert_eq!(
            result,
            Alignment {
                fuel_required: 37,
                position: 2,
            },
        )
    }

    #[test]
    fn test_cheapest_alignment_mk2() {
        let result = cheapest_alignment_mk2(&EXAMPLE_INPUT);
        assert_eq!(
            result,
            Alignment {
                fuel_required: 168,
                position: 5,
            },
        )
    }

    #[test]
    fn part_two_answer() {
        let result = part_two();
        assert_eq!(result, 99763899);
    }
}
