// Day 13: Transparent Origami
use crate::prelude::*;
use crate::shared::grid::{HashGrid, Point};
use std::borrow::Cow;
use std::fmt::Display;
use std::str::FromStr;

lazy_static! {
    static ref PUZZLE_INPUT: Instructions =
        Instructions::from_lines(include_lines!("day13_input.txt")).unwrap();
}

pub fn part_one() -> usize {
    PUZZLE_INPUT
        .grid
        .fold(PUZZLE_INPUT.fold_instructions[0])
        .count_dots()
}

pub fn part_two() -> String {
    PUZZLE_INPUT.follow_instructions().to_string()
}

#[derive(Clone)]
struct OrigamiGrid(HashGrid<()>);

impl OrigamiGrid {
    fn fold(&self, instruction: FoldInstruction) -> OrigamiGrid {
        let mut new_grid = OrigamiGrid(HashGrid::new());
        for (point, _) in self.0.all_extant_points() {
            let Point { x, y } = point;
            let new_point = match instruction {
                FoldInstruction::X(fold_x) => {
                    if x > fold_x {
                        Point::new(2 * fold_x - x, y)
                    } else {
                        point
                    }
                }
                FoldInstruction::Y(fold_y) => {
                    if y > fold_y {
                        Point::new(x, 2 * fold_y - y)
                    } else {
                        point
                    }
                }
            };
            new_grid.0.set(new_point, ());
        }
        new_grid
    }

    fn count_dots(&self) -> usize {
        self.0.all_extant_points().count()
    }
}

impl FromIterator<Point> for OrigamiGrid {
    fn from_iter<T: IntoIterator<Item = Point>>(iter: T) -> Self {
        let mut grid = OrigamiGrid(HashGrid::new());
        for point in iter {
            grid.0.set(point, ());
        }
        grid
    }
}

impl Display for OrigamiGrid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let width = self.0.layout().width;
        let display = (0..self.0.layout().height)
            .map(|y| {
                (0..width)
                    .map(|x| match self.0.get(Point::new(x, y)) {
                        Some(()) => "#",
                        None => ".",
                    })
                    .join("")
            })
            .join("\n");

        f.write_str(display.as_str())
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum FoldInstruction {
    X(usize),
    Y(usize),
}

impl FromStr for FoldInstruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (command, number) = s
            .split_once('=')
            .ok_or(format!("Invalid fold command: {}", s))?;
        let number: usize = number.parse().map_err(|_| "Invalid number")?;
        match command {
            "fold along x" => Ok(FoldInstruction::X(number)),
            "fold along y" => Ok(FoldInstruction::Y(number)),
            _ => Err(format!("Invalid fold command: {}", s)),
        }
    }
}

struct Instructions {
    grid: OrigamiGrid,
    fold_instructions: Vec<FoldInstruction>,
}

impl Instructions {
    fn from_lines<'a, T: IntoIterator<Item = &'a str>>(iter: T) -> Result<Instructions, String> {
        let mut iter = iter.into_iter();
        let points = (&mut iter)
            .take_while(|it| !it.is_empty())
            .map(|it| Point::from_str(it))
            .collect::<Result<Vec<_>, _>>()?;
        let grid: OrigamiGrid = points.into_iter().collect();

        let fold_instructions = (&mut iter).map(|it| it.parse()).collect::<Result<_, _>>()?;

        Ok(Instructions {
            grid,
            fold_instructions,
        })
    }

    fn follow_instructions(&self) -> OrigamiGrid {
        self.fold_instructions
            .iter()
            .fold(Cow::Borrowed(&self.grid), |grid, instruction| {
                Cow::Owned(grid.fold(*instruction))
            })
            .into_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    lazy_static! {
        static ref EXAMPLE_INPUT: Instructions = Instructions::from_lines(
            [
                "6,10",
                "0,14",
                "9,10",
                "0,3",
                "10,4",
                "4,11",
                "6,0",
                "6,12",
                "4,1",
                "0,13",
                "10,12",
                "3,4",
                "3,0",
                "8,4",
                "1,10",
                "2,14",
                "8,10",
                "9,0",
                "",
                "fold along y=7",
                "fold along x=5",
            ]
            .into_iter()
        )
        .unwrap();
    }

    #[test]
    fn test_parse() {
        assert_eq!(EXAMPLE_INPUT.grid.count_dots(), 18);
        assert_eq!(EXAMPLE_INPUT.fold_instructions.len(), 2);
    }

    #[test]
    fn test_fold() {
        let result = EXAMPLE_INPUT.grid.fold(EXAMPLE_INPUT.fold_instructions[0]);
        assert_eq!(result.count_dots(), 17);
    }

    #[test]
    fn part_one_answer() {
        let result = part_one();
        assert_eq!(result, 710);
    }

    #[test]
    fn test_display() {
        let expected = ["#####", "#...#", "#...#", "#...#", "#####"].join("\n");
        let result = EXAMPLE_INPUT.follow_instructions();
        assert_eq!(result.to_string(), expected);
    }

    #[test]
    fn part_two_answer() {
        let expected = include_str!("day13_answer.txt").trim_end();
        let result = part_two();
        assert_eq!(result.as_str(), expected);
    }
}
