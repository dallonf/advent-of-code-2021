// Day 9: Smoke Basin
use crate::prelude::*;

lazy_static! {
    static ref PUZZLE_INPUT: Grid =
        Grid::from_lines(&include_lines!("day09_input.txt").collect::<Box<[&str]>>()).unwrap();
}

pub fn part_one() -> u32 {
    PUZZLE_INPUT
        .get_risk_levels_of_low_points()
        .map(|it| it as u32)
        .sum()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Clone)]
struct Grid {
    width: usize,
    data: Box<[u8]>,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Point { x, y }
    }
}

impl Grid {
    fn from_lines<'a>(lines: &[&'a str]) -> Result<Self, String> {
        let mut validate_iter = lines.iter();
        let width = validate_iter
            .next()
            .ok_or("Can't create grid from empty slice")?
            .char_indices()
            .count();
        if validate_iter.any(|line| line.char_indices().count() != width) {
            return Err("Not all lines are the same length".to_string());
        }
        let data = lines
            .into_iter()
            .flat_map(|line| {
                line.chars().map(|digit| {
                    digit
                        .to_string()
                        .parse()
                        .map_err(|_| format!("Invalid digit: {}", digit))
                })
            })
            .collect::<Result<Box<[u8]>, _>>()?;
        Ok(Grid { width, data })
    }

    fn height(&self) -> usize {
        self.data.len() / self.width
    }

    fn get(&self, point: Point) -> u8 {
        self.data[point.y * self.width + point.x]
    }

    fn adjacent_points(&self, point: Point) -> impl Iterator<Item = Point> + '_ {
        let x = point.x as isize;
        let y = point.y as isize;
        [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)]
            .into_iter()
            .filter_map(|(x, y)| {
                if x >= 0 && x < self.width as isize && y >= 0 && y < self.height() as isize {
                    Some(Point::new(x as usize, y as usize))
                } else {
                    None
                }
            })
    }

    fn get_risk_level(&self, point: Point) -> u8 {
        self.get(point) + 1
    }

    fn all_points(&self) -> impl Iterator<Item = Point> + '_ {
        (0..self.height()).flat_map(|y| (0..self.width).map(move |x| Point::new(x, y)))
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
        static ref EXAMPLE_INPUT: Grid = Grid::from_lines(&[
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
        assert_eq!(EXAMPLE_INPUT.get(Point::new(0, 0)), 2);
        assert_eq!(EXAMPLE_INPUT.get(Point::new(1, 0)), 1);
        assert_eq!(EXAMPLE_INPUT.get(Point::new(0, 1)), 3);
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
