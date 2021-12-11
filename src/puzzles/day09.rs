// Day 9: Smoke Basin
use crate::prelude::*;
use crate::shared::grid::{Grid, Point};

lazy_static! {
    static ref PUZZLE_INPUT: Grid<u8> =
        Grid::from_digit_lines(&include_lines!("day09_input.txt").collect::<Box<[&str]>>())
            .unwrap();
}

pub fn part_one() -> u32 {
    PUZZLE_INPUT
        .get_risk_levels_of_low_points()
        .map(|it| it as u32)
        .sum()
}

impl Grid<u8> {
    fn get_risk_level(&self, point: Point) -> u8 {
        self.get(point) + 1
    }

    fn low_points(&self) -> impl Iterator<Item = Point> + '_ {
        self.all_points().filter(|it| self.is_low_point(*it))
    }

    fn get_risk_levels_of_low_points(&self) -> impl Iterator<Item = u8> + '_ {
        self.low_points().map(|it| self.get_risk_level(it))
    }

    fn is_low_point(&self, point: Point) -> bool {
        let value = self.get(point);
        self.adjacent_points(point)
            .all(|other| self.get(other) > value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    lazy_static! {
        static ref EXAMPLE_INPUT: Grid<u8> = Grid::from_digit_lines(&[
            "2199943210",
            "3987894921",
            "9856789892",
            "8767896789",
            "9899965678",
        ])
        .unwrap();
    }

    #[test]
    fn test_get_from_grid() {
        assert_eq!(*EXAMPLE_INPUT.get(Point::new(0, 0)), 2);
        assert_eq!(*EXAMPLE_INPUT.get(Point::new(1, 0)), 1);
        assert_eq!(*EXAMPLE_INPUT.get(Point::new(0, 1)), 3);
    }

    #[test]
    fn test_get_adjacent_points() {
        let result: Box<[Point]> = EXAMPLE_INPUT.adjacent_points(Point::new(0, 0)).collect();
        let expected: Box<[Point]> = Box::new([Point::new(1, 0), Point::new(0, 1)]);
        assert_eq!(result, expected);
    }

    #[test]
    fn is_low_point() {
        assert_eq!(EXAMPLE_INPUT.is_low_point(Point::new(0, 0)), false);
        assert_eq!(EXAMPLE_INPUT.is_low_point(Point::new(1, 0)), true);
    }

    #[test]
    fn test_get_low_points() {
        let result: Box<[Point]> = EXAMPLE_INPUT.low_points().collect();
        let expected: Box<[Point]> = Box::new([
            Point::new(1, 0),
            Point::new(9, 0),
            Point::new(2, 2),
            Point::new(6, 4),
        ]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_get_risk_levels() {
        let result: Box<[u8]> = EXAMPLE_INPUT.get_risk_levels_of_low_points().collect();
        let expected: Box<[u8]> = Box::new([2, 1, 6, 6]);
        assert_eq!(result, expected);
    }

    #[test]
    fn part_one_answer() {
        let result = part_one();
        assert_eq!(result, 554);
    }
}
