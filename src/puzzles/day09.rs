// Day 9: Smoke Basin
use crate::prelude::*;
use crate::shared::grid::{Grid, Point};

lazy_static! {
    static ref PUZZLE_INPUT: SmokeBasinGrid = SmokeBasinGrid(
        Grid::from_digit_lines(&include_lines!("day09_input.txt").collect::<Box<[&str]>>())
            .unwrap()
    );
}

pub fn part_one() -> u32 {
    PUZZLE_INPUT
        .get_risk_levels_of_low_points()
        .map(|it| it as u32)
        .sum()
}

struct SmokeBasinGrid(Grid<u8>);

impl SmokeBasinGrid {
    fn get_risk_level(&self, point: Point) -> u8 {
        self.0.get(point) + 1
    }

    fn low_points(&self) -> impl Iterator<Item = Point> + '_ {
        self.0.all_points().filter(|it| self.is_low_point(*it))
    }

    fn get_risk_levels_of_low_points(&self) -> impl Iterator<Item = u8> + '_ {
        self.low_points().map(|it| self.get_risk_level(it))
    }

    fn is_low_point(&self, point: Point) -> bool {
        let value = self.0.get(point);
        self.0
            .adjacent_points(point)
            .all(|other| self.0.get(other) > value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    lazy_static! {
        static ref EXAMPLE_INPUT: SmokeBasinGrid = SmokeBasinGrid(
            Grid::from_digit_lines(&[
                "2199943210",
                "3987894921",
                "9856789892",
                "8767896789",
                "9899965678",
            ])
            .unwrap()
        );
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
