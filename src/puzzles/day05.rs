use core::panic;
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
    fn points(&self) -> Box<dyn Iterator<Item = Point>> {
        if self.0.x == self.1.x {
            let x = self.0.x;
            let range = if self.1.y >= self.0.y {
                RangeInclusive::new(self.0.y, self.1.y)
            } else {
                RangeInclusive::new(self.1.y, self.0.y)
            };
            Box::new(range.map(move |y| Point { x, y }))
        } else if self.0.y == self.1.y {
            let y = self.0.y;
            let range = if self.1.x >= self.0.x {
                RangeInclusive::new(self.0.x, self.1.x)
            } else {
                RangeInclusive::new(self.1.x, self.0.x)
            };
            Box::new(range.map(move |x| Point { x, y }))
        } else {
            panic!("Haven't implemented iteration over diagonal lines yet");
        }
    }

    fn is_horizontal(&self) -> bool {
        self.0.x == self.1.x || self.0.y == self.1.y
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
}
