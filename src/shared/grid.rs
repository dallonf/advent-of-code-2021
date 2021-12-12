use crate::prelude::*;
use std::fmt::Debug;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point {
    x: usize,
    y: usize,
}

impl Point {
    pub fn new(x: usize, y: usize) -> Self {
        Point { x, y }
    }
}

#[derive(Clone, Debug)]
pub struct Grid<T> {
    width: usize,
    data: Box<[T]>,
}

impl<T> Grid<T> {
    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.data.len() / self.width
    }

    pub fn get(&self, point: Point) -> &T {
        &self.data[point.y * self.width + point.x]
    }

    pub fn adjacent_points(&self, point: Point) -> impl Iterator<Item = Point> + '_ {
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

    pub fn all_points(&self) -> impl Iterator<Item = Point> + '_ {
        (0..self.height()).flat_map(|y| (0..self.width).map(move |x| Point::new(x, y)))
    }
}

impl Grid<u8> {
    pub fn from_digit_lines<'a>(lines: &[&'a str]) -> Result<Self, String> {
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
}
