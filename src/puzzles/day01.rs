use std::collections::VecDeque;

// Day 1: Sonar Sweep
use crate::prelude::*;

lazy_static! {
    static ref PUZZLE_INPUT: Box<[u32]> = include_str!("day01_input.txt")
        .lines()
        .map(|it| it.parse().unwrap())
        .collect();
}

pub fn part_one() -> usize {
    count_increases(PUZZLE_INPUT.iter().copied())
}

pub fn part_two() -> usize {
    count_window_increases(PUZZLE_INPUT.iter().copied())
}

trait IterWindows<T> {
    fn windows<const N: usize>(&mut self) -> IterWindowIterator<T, Self, N>
    where
        Self: Iterator<Item = T> + Sized;
}

impl<T, TIterator> IterWindows<T> for TIterator
where
    T: Copy,
    TIterator: Iterator<Item = T>,
{
    fn windows<const N: usize>(&mut self) -> IterWindowIterator<T, Self, N>
    where
        Self: Iterator<Item = T> + Sized,
    {
        IterWindowIterator::new(self)
    }
}

impl<'a, T, TIterator: Iterator<Item = T>, const N: usize> IterWindowIterator<'a, T, TIterator, N>
where
    T: Copy,
{
    fn new(iter: &'a mut TIterator) -> Self {
        IterWindowIterator {
            iterator: iter,
            window_in_progress: VecDeque::with_capacity(N),
        }
    }
}

struct IterWindowIterator<'a, T, TIterator: Iterator<Item = T>, const N: usize> {
    iterator: &'a mut TIterator,
    window_in_progress: VecDeque<T>,
}

impl<'a, T, TIterator: Iterator<Item = T>, const N: usize> Iterator
    for IterWindowIterator<'a, T, TIterator, N>
where
    T: Copy,
    T: Default,
{
    type Item = [T; N];

    fn next(&mut self) -> Option<Self::Item> {
        // just starting out
        if self.window_in_progress.len() == 0 {
            for _ in 0..(N - 1) {
                let next = self.iterator.next();
                if let Some(next) = next {
                    self.window_in_progress.push_back(next);
                } else {
                    return None;
                }
            }
        }

        // window should always be waiting for the next element
        assert_eq!(self.window_in_progress.len(), N - 1);

        let next = self.iterator.next();
        if let Some(next) = next {
            self.window_in_progress.push_back(next);
            // could remove the dependency on Default by unsafely allocating an array
            // with garbage data, knowing that we'll completely fill it before returning it
            let mut result = [T::default(); N];
            for i in 0..N {
                result[i] = self.window_in_progress[i];
            }
            self.window_in_progress.pop_front();
            Some(result)
        } else {
            return None;
        }
    }
}

fn count_increases(mut readings: impl Iterator<Item = u32>) -> usize {
    readings
        .windows::<2>()
        .filter(|&window| {
            let prev = window[0];
            let next = window[1];
            next > prev
        })
        .count()
}

fn count_window_increases(mut readings: impl Iterator<Item = u32>) -> usize {
    let windows = readings
        .windows::<3>()
        .map(|window| window.into_iter().sum());
    count_increases(windows)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: [u32; 10] = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

    #[test]
    fn part_one_example() {
        let result = count_increases(EXAMPLE_INPUT.iter().copied());
        assert_eq!(result, 7);
    }

    #[test]
    fn part_one_answer() {
        let result = part_one();
        assert_eq!(result, 1766);
    }

    #[test]
    fn part_two_example() {
        let result = count_window_increases(EXAMPLE_INPUT.iter().copied());
        assert_eq!(result, 5);
    }

    #[test]
    fn part_two_answer() {
        let result = part_two();
        assert_eq!(result, 1797);
    }
}
