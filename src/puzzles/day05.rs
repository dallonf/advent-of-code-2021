use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fmt::Debug;
use std::ops::RangeInclusive;
use std::str::FromStr;

// Day 5: Hydrothermal Venture
use crate::prelude::*;

lazy_static! {
    static ref PUZZLE_INPUT: Box<[Line]> = include_lines!("day05_input.txt")
        .map(|line| line.parse().unwrap())
        .collect();
}

pub fn part_one() -> usize {
    compute_overlapping_for_horizontal_lines(PUZZLE_INPUT.iter().copied())
}

pub fn part_two() -> usize {
    compute_overlapping(PUZZLE_INPUT.iter().copied())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

impl FromStr for Point {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s
            .split_once(",")
            .ok_or("Expected point to be delimited by ,".to_string())?;

        let x = x
            .parse()
            .map_err(|_| format!("Can't convert {} to a number", x))?;
        let y = y
            .parse()
            .map_err(|_| format!("Can't convert {} to a number", y))?;

        Ok(Point { x, y })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Line(Point, Point);

impl FromStr for Line {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (p1, p2) = s
            .split_once(" -> ")
            .ok_or("Expected line to be divided by ->".to_string())?;

        Ok(Line(p1.parse()?, p2.parse()?))
    }
}

impl Line {
    fn points(&self) -> LineIterator {
        LineIterator {
            current: self.0,
            x_delta: self.1.x as isize - self.0.x as isize,
            y_delta: self.1.y as isize - self.0.y as isize,
            emitted_last_point: false,
        }
    }

    fn is_horizontal(&self) -> bool {
        self.0.x == self.1.x || self.0.y == self.1.y
    }
}

struct LineIterator {
    current: Point,
    x_delta: isize,
    y_delta: isize,
    emitted_last_point: bool,
}

impl Iterator for LineIterator {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.emitted_last_point {
            return None;
        }

        // the result is the current item _before_ incrementing
        let result = self.current;

        let x_sign = self.x_delta.signum();
        let y_sign = self.y_delta.signum();

        if x_sign == 0 && self.y_delta == 0 {
            // nothing to do, so mark that we're done
            self.emitted_last_point = true;
        } else {
            self.current.x = (self.current.x as isize + x_sign).try_into().unwrap();
            self.x_delta -= x_sign;
            self.current.y = (self.current.y as isize + y_sign).try_into().unwrap();
            self.y_delta -= y_sign;
        }

        Some(result)
    }
}

struct Grid {
    map: HashMap<Point, usize>,
}

impl FromIterator<Line> for Grid {
    fn from_iter<T: IntoIterator<Item = Line>>(iter: T) -> Self {
        let mut map = HashMap::new();
        iter.into_iter()
            .flat_map(|line| line.points())
            .for_each(|point| match map.entry(point) {
                Entry::Occupied(mut entry) => *entry.get_mut() += 1,
                Entry::Vacant(entry) => {
                    entry.insert(1);
                }
            });
        Grid { map }
    }
}

impl Grid {
    fn overlapping_points(&self) -> usize {
        self.map.values().filter(|&&count| count > 1).count()
    }
}

impl Debug for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let keys: Box<[&Point]> = self.map.keys().collect();
        let max_x = keys.iter().map(|it| it.x).max().unwrap();
        let max_y = keys.iter().map(|it| it.y).max().unwrap();

        let diagram = RangeInclusive::new(0, max_y)
            .map(|y| {
                RangeInclusive::new(0, max_x)
                    .map(|x| match self.map.get(&Point { x, y }) {
                        Some(count) => count.to_string(),
                        None => ".".to_string(),
                    })
                    .collect::<Box<[String]>>()
                    .join("")
            })
            .collect::<Box<[String]>>()
            .join("\n");

        write!(f, "{}", diagram)
    }
}

fn compute_overlapping_for_horizontal_lines<T: IntoIterator<Item = Line>>(lines: T) -> usize {
    let grid: Grid = lines.into_iter().filter(Line::is_horizontal).collect();
    grid.overlapping_points()
}

fn compute_overlapping<T: IntoIterator<Item = Line>>(lines: T) -> usize {
    let grid: Grid = lines.into_iter().collect();
    grid.overlapping_points()
}

#[cfg(test)]
mod tests {
    use super::*;

    lazy_static! {
        static ref EXAMPLE_INPUT: Box<[Line]> = [
            "0,9 -> 5,9",
            "8,0 -> 0,8",
            "9,4 -> 3,4",
            "2,2 -> 2,1",
            "7,0 -> 7,4",
            "6,4 -> 2,0",
            "0,9 -> 2,9",
            "3,4 -> 1,4",
            "0,0 -> 8,8",
            "5,5 -> 8,2",
        ]
        .into_iter()
        .map(|it| it.parse().unwrap())
        .collect();
    }

    #[test]
    fn part_one_example() {
        let result = compute_overlapping_for_horizontal_lines(EXAMPLE_INPUT.iter().copied());
        assert_eq!(result, 5);
    }

    #[test]
    fn part_one_answer() {
        let result = part_one();
        assert_eq!(result, 5774);
    }

    #[test]
    fn part_two_example() {
        let result = compute_overlapping(EXAMPLE_INPUT.iter().copied());
        assert_eq!(result, 12);
    }

    #[test]
    fn part_two_answer() {
        let result = part_two();
        assert_eq!(result, 18423);
    }
}
