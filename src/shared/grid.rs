use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fmt::{Debug, Display};
use std::ops::RangeInclusive;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    pub fn new(x: usize, y: usize) -> Self {
        Point { x, y }
    }

    pub fn adjacent_points(&self, layout: &GridLayout) -> impl Iterator<Item = Point> {
        let x = self.x as isize;
        let y = self.y as isize;
        let width = layout.width as isize;
        let height = layout.height as isize;
        [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)]
            .into_iter()
            .filter_map(move |(new_x, new_y)| {
                if new_x >= 0 && new_x < width && new_y >= 0 && new_y < height {
                    Some(Point::new(new_x as usize, new_y as usize))
                } else {
                    None
                }
            })
    }

    pub fn adjacent_points_with_diagonals(
        &self,
        layout: &GridLayout,
    ) -> impl Iterator<Item = Point> {
        let x = self.x as isize;
        let y = self.y as isize;
        let width = layout.width as isize;
        let height = layout.height as isize;
        RangeInclusive::new(x - 1, x + 1).flat_map(move |new_x| {
            RangeInclusive::new(y - 1, y + 1).filter_map(move |new_y| {
                if new_x >= 0
                    && new_x < width
                    && new_y >= 0
                    && new_y < height
                    && (new_x, new_y) != (x, y)
                {
                    Some(Point::new(new_x as usize, new_y as usize))
                } else {
                    None
                }
            })
        })
    }
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

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct GridLayout {
    pub width: usize,
    pub height: usize,
}

impl GridLayout {
    pub fn new(width: usize, height: usize) -> Self {
        GridLayout { width, height }
    }

    pub fn all_points<'a>(&'a self) -> impl Iterator<Item = Point> + 'a {
        (0..self.height).flat_map(|y| (0..self.width).map(move |x| Point::new(x, y)))
    }
}

#[derive(Clone)]
pub struct ArrayGrid<T> {
    layout: GridLayout,
    data: Box<[T]>,
}

impl<T> ArrayGrid<T> {
    fn index_of(&self, point: Point) -> usize {
        point.y * self.layout.width + point.x
    }

    pub fn layout(&self) -> &GridLayout {
        &self.layout
    }

    pub fn get(&self, point: Point) -> &T {
        &self.data[self.index_of(point)]
    }

    pub fn set(&mut self, point: Point, new_val: T) {
        self.data[self.index_of(point)] = new_val;
    }

    pub fn update<Function>(&mut self, point: Point, new_val_fn: Function)
    where
        Function: Fn(&T) -> T,
    {
        self.data[self.index_of(point)] = new_val_fn(&self.data[self.index_of(point)]);
    }

    // this is mostly for debugging
    #[allow(dead_code)]
    pub fn map_values<Other>(&self, map_fn: fn(val: &T) -> Other) -> ArrayGrid<Other>
    where
        Other: Default + Clone,
    {
        let mut new_grid = ArrayGrid::new(self.layout.width, self.layout.height);
        for point in self.layout.all_points() {
            let prev_val = self.get(point);
            let new_val = map_fn(prev_val);
            new_grid.set(point, new_val);
        }
        new_grid
    }
}

impl<T> ArrayGrid<T>
where
    T: Default + Clone,
{
    pub fn new(width: usize, height: usize) -> Self {
        Self::from_layout(GridLayout::new(width, height))
    }

    pub fn from_layout(layout: GridLayout) -> Self {
        let data = vec![T::default(); layout.width * layout.height];
        ArrayGrid {
            data: data.into_boxed_slice(),
            layout,
        }
    }
}

impl ArrayGrid<u8> {
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

        let layout = GridLayout::new(width, data.len() / width);
        Ok(ArrayGrid {
            data,
            layout: layout,
        })
    }
}

impl<T: Display> Debug for ArrayGrid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let size = format!("{} x {}", self.layout.width, self.layout.height);
        let data_strs: Box<[String]> = self.data.iter().map(|data| data.to_string()).collect();

        write!(f, "Grid ({})\n", size)?;

        let max_size = data_strs.iter().map(|it| it.char_indices().count()).max();
        let data_strs: Box<[String]> = data_strs
            .into_iter()
            .map(|it| format!("{:indent$}", it, indent = max_size.unwrap()))
            .collect();

        let grid_lines: Box<[String]> = (0..self.layout.height)
            .map(|y| {
                (0..self.layout.width)
                    .map(|x| data_strs[self.index_of(Point::new(x, y))].as_str())
                    .collect::<Box<[&str]>>()
                    .join("")
            })
            .collect();

        let data_str = grid_lines.join("\n");

        write!(f, "{}", data_str)
    }
}

#[derive(Clone)]
pub struct HashGrid<T> {
    map: HashMap<Point, T>,
    layout: GridLayout,
}

impl<T> HashGrid<T> {
    pub fn new() -> Self {
        HashGrid {
            map: HashMap::new(),
            layout: GridLayout::new(0, 0),
        }
    }

    pub fn layout(&self) -> &GridLayout {
        &self.layout
    }

    pub fn get(&self, point: Point) -> Option<&T> {
        self.map.get(&point)
    }

    pub fn set(&mut self, point: Point, new_val: T) {
        self.map.insert(point, new_val);
        self.grow_layout(point);
    }

    pub fn update<Function>(&mut self, point: Point, new_val_fn: Function)
    where
        Function: Fn(Option<&T>) -> T,
    {
        match self.map.entry(point) {
            Entry::Occupied(mut entry) => {
                entry.insert(new_val_fn(Some(entry.get())));
            }
            Entry::Vacant(entry) => {
                entry.insert(new_val_fn(None));
                self.grow_layout(point);
            }
        }
    }

    pub fn all_extant_points(&self) -> impl Iterator<Item = (Point, &T)> {
        self.map.iter().map(|(point, it)| (*point, it))
    }

    fn grow_layout(&mut self, new_point: Point) {
        if self.layout.width <= new_point.x {
            self.layout.width = new_point.x + 1;
        }
        if self.layout.height <= new_point.y {
            self.layout.height = new_point.y + 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;

    lazy_static! {
        static ref EXAMPLE_INPUT: ArrayGrid<u8> = ArrayGrid::from_digit_lines(&[
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
        let result: Vec<Point> = Point::new(0, 0)
            .adjacent_points(EXAMPLE_INPUT.layout())
            .collect_vec();
        let expected: Vec<Point> = vec![Point::new(1, 0), Point::new(0, 1)];
        assert_eq!(result, expected);
    }
}
