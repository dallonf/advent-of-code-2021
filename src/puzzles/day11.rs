use std::collections::VecDeque;

// Day 11: Dumbo Octopus
use crate::prelude::*;
use crate::shared::grid::{Grid, Point};

lazy_static! {
    static ref PUZZLE_INPUT: OctopusGrid =
        OctopusGrid::from_digit_lines(&include_lines!("day11_input.txt").collect::<Box<[&str]>>())
            .unwrap();
}

pub fn part_one() -> usize {
    PUZZLE_INPUT.clone().simulate(100)
}

pub fn part_two() -> usize {
    PUZZLE_INPUT.clone().steps_until_sync()
}

#[derive(Clone, Debug)]
struct OctopusGrid(Grid<u8>);

impl OctopusGrid {
    fn from_digit_lines(lines: &[&str]) -> Result<Self, String> {
        Ok(OctopusGrid(Grid::from_digit_lines(lines)?))
    }

    /// Returns the number of flashes
    fn step(&mut self) -> usize {
        let mut flashed_this_step: Vec<Point> = Vec::new();
        let mut increment_queue: VecDeque<Point> = self.0.all_points().collect();

        while let Some(point) = increment_queue.pop_front() {
            self.0.update(point, |x| *x + 1);
            if *self.0.get(point) == 10 {
                flashed_this_step.push(point);
                for adjacent in self.0.adjacent_points_with_diagonals(point) {
                    increment_queue.push_back(adjacent);
                }
            }
        }

        for point in flashed_this_step.iter() {
            self.0.set(*point, 0);
        }

        flashed_this_step.len()
    }

    /// Returns the number of flashes
    fn simulate(&mut self, num_steps: usize) -> usize {
        (0..num_steps).map(|_| self.step()).sum()
    }

    fn steps_until_sync(&mut self) -> usize {
        let mut steps = 0;
        let octopus_count = self.0.width() * self.0.height();
        loop {
            steps += 1;
            let flashes = self.step();
            if flashes == octopus_count {
                return steps;
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    lazy_static! {
        static ref EXAMPLE_INPUT: OctopusGrid = OctopusGrid::from_digit_lines(&[
            "5483143223",
            "2745854711",
            "5264556173",
            "6141336146",
            "6357385478",
            "4167524645",
            "2176841721",
            "6882881134",
            "4846848554",
            "5283751526",
        ])
        .unwrap();
    }

    #[test]
    fn test_simple_step() {
        let mut grid =
            OctopusGrid::from_digit_lines(&["11111", "19991", "19191", "19991", "11111"]).unwrap();
        assert_eq!(grid.step(), 9);
    }

    #[test]
    fn test_steps() {
        let mut grid = EXAMPLE_INPUT.clone();
        assert_eq!(grid.step(), 0);
        assert_eq!(grid.step(), 35);
    }

    #[test]
    fn test_quick_example() {
        let result = EXAMPLE_INPUT.clone().simulate(10);
        assert_eq!(result, 204);
    }

    #[test]
    fn test_long_example() {
        let result = EXAMPLE_INPUT.clone().simulate(100);
        assert_eq!(result, 1656);
    }

    #[test]
    fn part_one_answer() {
        let result = part_one();
        assert_eq!(result, 1697);
    }

    #[test]
    fn test_steps_until_sync() {
        let result = EXAMPLE_INPUT.clone().steps_until_sync();
        assert_eq!(result, 195);
    }

    #[test]
    fn part_two_answer() {
        let result = part_two();
        assert_eq!(result, 344);
    }
}
