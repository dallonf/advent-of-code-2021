use std::collections::{hash_map::Entry, HashMap};

// Day 15: Chiton
use crate::{
    prelude::*,
    shared::grid::{ArrayGrid, Grid, Point},
};

lazy_static! {
    static ref PUZZLE_INPUT: ArrayGrid<u8> =
        ArrayGrid::from_digit_lines(&include_lines!("day15_input.txt").collect_vec()).unwrap();
}

pub fn part_one() -> u32 {
    find_lowest_risk_path(&PUZZLE_INPUT)
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct NodeInfo {
    prev: Point,
    distance: u32,
}

fn find_lowest_risk_path(map: &ArrayGrid<u8>) -> u32 {
    let destination = Point::new(map.width() - 1, map.height() - 1);
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

        for neighbor in map
            .adjacent_points(current)
            .into_iter()
            .filter(|point| !visited.contains_key(&point))
        {
            let cost = *map.get(neighbor);
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
        let result = find_lowest_risk_path(&EXAMPLE_INPUT);
        assert_eq!(result, 40);
    }

    #[test]
    fn part_one_answer() {
        let result = part_one();
        assert_eq!(result, 361);
    }
}
