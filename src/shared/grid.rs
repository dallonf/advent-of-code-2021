use std::{
    fmt::{Debug, Display},
    ops::RangeInclusive,
};

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

#[derive(Clone)]
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

    fn index_of(&self, point: Point) -> usize {
        point.y * self.width + point.x
    }

    pub fn get(&self, point: Point) -> &T {
        &self.data[self.index_of(point)]
    }

    pub fn set(&mut self, point: Point, new_val: T) {
        self.data[self.index_of(point)] = new_val;
    }

    pub fn update(&mut self, point: Point, new_val_fn: fn(prev: &T) -> T) {
        self.data[self.index_of(point)] = new_val_fn(&self.data[self.index_of(point)]);
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

    pub fn adjacent_points_with_diagonals(&self, point: Point) -> impl Iterator<Item = Point> + '_ {
        let x = point.x as isize;
        let y = point.y as isize;
        RangeInclusive::new(x - 1, x + 1).flat_map(move |new_x| {
            RangeInclusive::new(y - 1, y + 1).filter_map(move |new_y| {
                if new_x >= 0
                    && new_x < self.width as isize
                    && new_y >= 0
                    && new_y < self.height() as isize
                    && (new_x, new_y) != (x, y)
                {
                    Some(Point::new(new_x as usize, new_y as usize))
                } else {
                    None
                }
            })
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

impl<T: Display> Debug for Grid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let size = format!("{} x {}", self.width(), self.height());
        let data_strs: Box<[String]> = self.data.iter().map(|data| data.to_string()).collect();

        write!(f, "Grid ({})\n", size)?;

        let max_size = data_strs.iter().map(|it| it.char_indices().count()).max();
        let data_strs: Box<[String]> = data_strs
            .into_iter()
            .map(|it| format!("{:indent$}", it, indent = max_size.unwrap()))
            .collect();

        let grid_lines: Box<[String]> = (0..self.height())
            .map(|y| {
                (0..self.width())
                    .map(|x| data_strs[self.index_of(Point::new(x, y))].as_str())
                    .collect::<Box<[&str]>>()
                    .join("")
            })
            .collect();

        let data_str = grid_lines.join("\n");

        write!(f, "{}", data_str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;

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
