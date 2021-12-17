// Day 15: Chiton
use crate::{
    prelude::*,
    shared::grid::{ArrayGrid, GridLayout, Point},
};
use std::collections::{hash_map::Entry, HashMap};
use std::ops::Deref;

lazy_static! {
    static ref PUZZLE_INPUT: ArrayGrid<u8> =
        ArrayGrid::from_digit_lines(&include_lines!("day15_input.txt").collect_vec()).unwrap();
}

pub fn part_one() -> u32 {
    find_lowest_risk_path(PUZZLE_INPUT.deref())
}

pub fn part_two() -> u32 {
    let expanded = ExpandedGrid::new(&PUZZLE_INPUT);
    find_lowest_risk_path(&expanded)
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct NodeInfo {
    prev: Point,
    distance: u32,
}

trait RiskMap {
    fn layout(&self) -> &GridLayout;
    fn get(&self, point: Point) -> u8;
}

impl RiskMap for ArrayGrid<u8> {
    fn layout(&self) -> &GridLayout {
        self.layout()
    }

    fn get(&self, point: Point) -> u8 {
        *self.get(point)
    }
}

fn find_lowest_risk_path(map: &impl RiskMap) -> u32 {
    let destination = Point::new(map.layout().width - 1, map.layout().height - 1);
    let mut pending_visit: HashMap<Point, NodeInfo> = HashMap::new();
    let mut visited: HashMap<Point, NodeInfo> = HashMap::new();
    let mut current = Point::new(0, 0);
    pending_visit.insert(
        current,
        NodeInfo {
            distance: 0,
            prev: current,
        },
    );

    while current != destination {
        let &NodeInfo {
            distance: current_distance,
            ..
        } = pending_visit.get(&current).unwrap();

        for neighbor in current
            .adjacent_points(map.layout())
            .filter(|point| !visited.contains_key(&point))
        {
            let cost = map.get(neighbor);
            let path_distance = current_distance + cost as u32;
            match pending_visit.entry(neighbor) {
                Entry::Occupied(mut entry) => {
                    let existing_distance = entry.get().distance;
                    if existing_distance > path_distance {
                        entry.insert(NodeInfo {
                            prev: current,
                            distance: existing_distance,
                        });
                    }
                }
                Entry::Vacant(entry) => {
                    entry.insert(NodeInfo {
                        prev: current,
                        distance: path_distance,
                    });
                }
            }
        }

        // mark as visited
        let info = pending_visit.remove(&current).unwrap();
        visited.insert(current, info);

        current = pending_visit
            .iter()
            .min_by_key(|(_, info)| info.distance)
            .map(|(point, _)| *point)
            .unwrap();
    }

    if let Some(NodeInfo { distance, .. }) = pending_visit.get(&current) {
        *distance
    } else {
        panic!()
    }
}

const META_GRID_SCALE: usize = 5;

struct ExpandedGrid<'a> {
    original_grid: &'a ArrayGrid<u8>,
    expanded_layout: GridLayout,
}

impl<'a> ExpandedGrid<'a> {
    fn new(original_grid: &'a ArrayGrid<u8>) -> Self {
        ExpandedGrid {
            original_grid,
            expanded_layout: GridLayout::new(
                original_grid.layout().width * META_GRID_SCALE,
                original_grid.layout().width * META_GRID_SCALE,
            ),
        }
    }
}

impl RiskMap for ExpandedGrid<'_> {
    fn layout(&self) -> &GridLayout {
        &self.expanded_layout
    }

    fn get(&self, point: Point) -> u8 {
        let original_layout = self.original_grid.layout();
        let meta_grid_x = point.x / original_layout.width;
        let meta_grid_y = point.y / original_layout.height;
        let original_point = Point::new(
            point.x % original_layout.width,
            point.y % original_layout.height,
        );
        let original_value = self.original_grid.get(original_point);
        (((original_value - 1) + meta_grid_x as u8 + meta_grid_y as u8) % 9) + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    lazy_static! {
        static ref EXAMPLE_INPUT: ArrayGrid<u8> = ArrayGrid::from_digit_lines(&[
            "1163751742",
            "1381373672",
            "2136511328",
            "3694931569",
            "7463417111",
            "1319128137",
            "1359912421",
            "3125421639",
            "1293138521",
            "2311944581",
        ])
        .unwrap();
    }

    #[test]
    fn test_example() {
        let result = find_lowest_risk_path(EXAMPLE_INPUT.deref());
        assert_eq!(result, 40);
    }

    #[test]
    fn part_one_answer() {
        let result = part_one();
        assert_eq!(result, 361);
    }

    #[test]
    fn test_simple_expansion() {
        let base = ArrayGrid::from_digit_lines(&["8"]).unwrap();
        let expanded = ExpandedGrid::new(&base);
        let expected =
            ArrayGrid::from_digit_lines(&["89123", "91234", "12345", "23456", "34567"]).unwrap();

        fn collect_test_grid(grid: &impl RiskMap) -> Vec<(Point, u8)> {
            grid.layout()
                .all_points()
                .map(|point| (point, grid.get(point)))
                .collect()
        }

        assert_eq!(collect_test_grid(&expanded), collect_test_grid(&expected));
    }

    #[test]
    fn test_expansion() {
        let expanded = ExpandedGrid::new(&EXAMPLE_INPUT);
        let original = Point::new(2, 1);
        assert_eq!(*EXAMPLE_INPUT.get(original), 8);
        let expected =
            ArrayGrid::from_digit_lines(&["89123", "91234", "12345", "23456", "34567"]).unwrap();
        let original_layout = EXAMPLE_INPUT.layout();
        for (meta_x, meta_y) in (0..META_GRID_SCALE).cartesian_product(0..META_GRID_SCALE) {
            let expanded_point = Point::new(
                original.x + meta_x * original_layout.width,
                original.y + meta_y * original_layout.height,
            );
            let meta_point = Point::new(meta_x, meta_y);
            assert_eq!(
                expanded.get(expanded_point),
                *expected.get(meta_point),
                "Expanded point: {:?}; Meta point: {:?}",
                expanded_point,
                meta_point
            );
        }
    }

    #[test]
    fn test_expanded_path() {
        let expanded = ExpandedGrid::new(&EXAMPLE_INPUT);
        let result = find_lowest_risk_path(&expanded);
        assert_eq!(result, 315);
    }

    #[test]
    fn part_two_answer() {
        let result = part_two();
        assert_eq!(result, 2838);
    }
}
