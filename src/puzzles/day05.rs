use crate::prelude::*;
use crate::shared::grid::{Point, SparseGrid};
use std::fmt::Debug;
use std::str::FromStr;

// Day 5: Hydrothermal Venture

fn parse_puzzle_input() -> Box<[Line]> {
    include_lines!("day05_input.txt")
        .map(|line| line.parse().unwrap())
        .collect()
}

pub fn part_one() -> usize {
    compute_overlapping_for_horizontal_lines(parse_puzzle_input().into_iter().copied())
}

pub fn part_two() -> usize {
    compute_overlapping(parse_puzzle_input().into_iter().copied())
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

struct VentGrid(SparseGrid<usize>);

impl FromIterator<Line> for VentGrid {
    fn from_iter<T: IntoIterator<Item = Line>>(iter: T) -> Self {
        let mut grid = SparseGrid::new();
        iter.into_iter()
            .flat_map(|line| line.points())
            .for_each(|point| {
                grid.update(point, |existing| match existing {
                    Some(existing) => *existing + 1,
                    None => 1,
                })
            });
        VentGrid(grid)
    }
}

impl VentGrid {
    fn overlapping_points(&self) -> usize {
        self.0
            .all_extant_points()
            .filter(|(_, &count)| count > 1)
            .count()
    }
}

// Hasn't been updated for shared SparseGrid implementation

// impl Debug for VentGrid {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         let keys: Box<[&Point]> = self.map.keys().collect();
//         let max_x = keys.iter().map(|it| it.x).max().unwrap();
//         let max_y = keys.iter().map(|it| it.y).max().unwrap();

//         let diagram = RangeInclusive::new(0, max_y)
//             .map(|y| {
//                 RangeInclusive::new(0, max_x)
//                     .map(|x| match self.map.get(&Point { x, y }) {
//                         Some(count) => count.to_string(),
//                         None => ".".to_string(),
//                     })
//                     .collect::<Box<[String]>>()
//                     .join("")
//             })
//             .collect::<Box<[String]>>()
//             .join("\n");

//         write!(f, "{}", diagram)
//     }
// }

fn compute_overlapping_for_horizontal_lines<T: IntoIterator<Item = Line>>(lines: T) -> usize {
    let grid: VentGrid = lines.into_iter().filter(Line::is_horizontal).collect();
    grid.overlapping_points()
}

fn compute_overlapping<T: IntoIterator<Item = Line>>(lines: T) -> usize {
    let grid: VentGrid = lines.into_iter().collect();
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
