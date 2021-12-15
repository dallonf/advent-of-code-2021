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
struct VisitedNodeInfo {
    prev: Point,
    distance: u32,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum NodeState {
    Start,
    Unvisited,
    PendingVisit(VisitedNodeInfo),
    Visited(VisitedNodeInfo),
}

impl Default for NodeState {
    fn default() -> Self {
        NodeState::Unvisited
    }
}

fn find_lowest_risk_path(map: &ArrayGrid<u8>) -> u32 {
    let destination = Point::new(map.width() - 1, map.height() - 1);
    let mut node_map: ArrayGrid<NodeState> = ArrayGrid::new(map.width(), map.height());
    let mut current = Point::new(0, 0);
    node_map.set(current, NodeState::Start);

    while current != destination {
        let current_distance = match node_map.get(current) {
            NodeState::Start => 0,
            NodeState::PendingVisit(VisitedNodeInfo { distance, .. }) => *distance,
            _ => panic!(),
        };
        for neighbor in map.adjacent_points(current) {
            let cost = *map.get(neighbor);
            let path_distance = current_distance + cost as u32;
            let new_state = match node_map.get(neighbor) {
                NodeState::Start | NodeState::Visited(_) => None,
                NodeState::Unvisited => Some(NodeState::PendingVisit(VisitedNodeInfo {
                    prev: current,
                    distance: path_distance,
                })),
                NodeState::PendingVisit(VisitedNodeInfo {
                    distance: existing_distance,
                    ..
                }) => {
                    if *existing_distance > path_distance {
                        Some(NodeState::PendingVisit(VisitedNodeInfo {
                            prev: current,
                            distance: path_distance,
                        }))
                    } else {
                        None
                    }
                }
            };
            if let Some(new_state) = new_state {
                node_map.set(neighbor, new_state);
            }
        }

        // mark as visited
        let new_state = if let NodeState::PendingVisit(node_info) = node_map.get(current) {
            Some(NodeState::Visited(*node_info))
        } else {
            None
        };
        if let Some(new_state) = new_state {
            node_map.set(current, new_state);
        }

        current = node_map
            .all_points()
            .into_iter()
            .filter_map(|node| match node_map.get(node) {
                NodeState::PendingVisit(node_info) => Some((node, node_info)),
                _ => None,
            })
            .min_by_key(|(_, info)| info.distance)
            .map(|(point, _)| point)
            .unwrap();
    }

    if let NodeState::PendingVisit(VisitedNodeInfo { distance, .. }) = node_map.get(destination) {
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
