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

pub fn part_two() -> usize {
    PUZZLE_INPUT.get_largest_basins_score()
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

    fn get_basin_sizes(&self) -> impl Iterator<Item = usize> {
        let mut basin_grid: Grid<Option<usize>> = Grid::new(self.0.width(), self.0.height());
        let mut next_basin_id = 0;

        for point in self.0.all_points() {
            let value = *self.0.get(point);
            if value == 9 {
                continue;
            }

            // are we going to make a new basin, add on to an existing basin, or merge two basins together?
            let neighboring_basins: Box<[usize]> = basin_grid
                .adjacent_points(point)
                .filter_map(|neighbor| *(basin_grid.get(neighbor)))
                .sorted()
                .collect();

            if neighboring_basins.len() == 0 {
                basin_grid.set(point, Some(next_basin_id));
                next_basin_id += 1;
            } else {
                let only_neighboring_basin = {
                    let first = neighboring_basins[0];
                    if neighboring_basins.iter().all(|&it| it == first) {
                        Some(first)
                    } else {
                        None
                    }
                };
                if let Some(only_neighboring_basin) = only_neighboring_basin {
                    basin_grid.set(point, Some(only_neighboring_basin));
                } else {
                    // this is where the fun begins
                    let possible_basins: Box<[usize]> = neighboring_basins
                        .iter()
                        .group_by(|it| *it)
                        .into_iter()
                        .map(|(key, _)| *key)
                        .collect();

                    let populated_points: Box<[Point]> = basin_grid
                        .all_points()
                        .take_while(|&scan_point| scan_point != point)
                        .collect();

                    let largest_neighboring_basin = *possible_basins
                        .iter()
                        .max_by_key(|&&basin_id| {
                            populated_points
                                .iter()
                                .filter(|&&it| *basin_grid.get(it) == Some(basin_id))
                                .count()
                        })
                        .unwrap();

                    // merge other basins into the largest
                    for &other_point in populated_points.iter() {
                        let value = *basin_grid.get(other_point);
                        if let Some(value) = value {
                            if possible_basins.contains(&value) {
                                basin_grid.set(other_point, Some(largest_neighboring_basin));
                            }
                        }
                    }

                    basin_grid.set(point, Some(largest_neighboring_basin));
                }
            }
        }

        basin_grid
            .all_points()
            .filter_map(|point| *basin_grid.get(point))
            .counts()
            .into_values()
    }

    fn get_largest_basins_score(&self) -> usize {
        self.get_basin_sizes().sorted().rev().take(3).product()
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

    #[test]
    fn test_get_basin_sizes() {
        let expected: Box<[usize]> = [3, 9, 14, 9].into_iter().sorted().collect();
        let result: Box<[usize]> = EXAMPLE_INPUT.get_basin_sizes().sorted().collect();
        assert_eq!(result, expected);
    }

    #[test]
    fn get_largest_basins_score() {
        let result = EXAMPLE_INPUT.get_largest_basins_score();
        assert_eq!(result, 1134);
    }

    #[test]
    fn part_two_answer() {
        let result = part_two();
        assert_eq!(result, 1017792);
    }
}
