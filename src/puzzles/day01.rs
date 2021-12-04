// Day 1: Sonar Sweep
use crate::prelude::*;

lazy_static! {
    static ref PUZZLE_INPUT: Box<[u32]> = include_lines!("day01_input.txt")
        .map(|it| it.parse().unwrap())
        .collect();
}

pub fn part_one() -> usize {
    count_increases(&PUZZLE_INPUT)
}

pub fn part_two() -> usize {
    count_window_increases(&PUZZLE_INPUT)
}

fn count_increases(readings: &[u32]) -> usize {
    readings
        .windows(2)
        .filter(|&window| {
            let prev = window[0];
            let next = window[1];
            next > prev
        })
        .count()
}

fn count_window_increases(readings: &[u32]) -> usize {
    let windows: Box<[u32]> = readings
        .windows(3)
        .map(|window| window.into_iter().sum())
        .collect();
    count_increases(&windows)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: [u32; 10] = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

    #[test]
    fn part_one_example() {
        let result = count_increases(&EXAMPLE_INPUT);
        assert_eq!(result, 7);
    }

    #[test]
    fn part_one_answer() {
        let result = part_one();
        assert_eq!(result, 1766);
    }

    #[test]
    fn part_two_example() {
        let result = count_window_increases(&EXAMPLE_INPUT);
        assert_eq!(result, 5);
    }

    #[test]
    fn part_two_answer() {
        let result = part_two();
        assert_eq!(result, 1797);
    }
}
