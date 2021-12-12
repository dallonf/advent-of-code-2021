use std::collections::{hash_map::Entry, HashMap, HashSet, VecDeque};

// Day 12: Passage Pathing
use crate::prelude::*;

lazy_static! {
    static ref PUZZLE_INPUT: ConnectionMap<'static> =
        ConnectionMap::from_lines(include_lines!("day12_input.txt")).unwrap();
}

pub fn part_one() -> usize {
    PUZZLE_INPUT.count_paths()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Node<'a> {
    Start,
    End,
    SmallCave(&'a str),
    LargeCave(&'a str),
}

impl<'a> Node<'a> {
    fn from_str(s: &'a str) -> Result<Node<'a>, String> {
        match s {
            "start" => Ok(Node::Start),
            "end" => Ok(Node::End),
            s if s.to_uppercase() == s => Ok(Node::LargeCave(s)),
            s if s.to_lowercase() == s => Ok(Node::SmallCave(s)),
            s => Err(format!("Could not parse '{}' as a node", s)),
        }
    }
}

struct ConnectionMap<'a>(HashMap<Node<'a>, Vec<Node<'a>>>);

impl<'a> ConnectionMap<'a> {
    fn from_lines<T: IntoIterator<Item = &'a str>>(iter: T) -> Result<Self, String> {
        let pairs = iter.into_iter().map(|line| {
            let (from_str, to_str) = line
                .split_once('-')
                .ok_or(format!("Expected nodes to be delimited by '-': '{}'", line))?;
            let from = Node::from_str(from_str);
            let to = Node::from_str(to_str);
            match (from, to) {
                (Ok(from), Ok(to)) => Ok((from, to)),
                (Err(err), _) | (_, Err(err)) => Err(err),
            }
        });
        let mut result = ConnectionMap(HashMap::new());
        for pair in pairs {
            match pair {
                Err(err) => return Err(err),
                Ok((from, to)) => result.insert_connection(from, to),
            }
        }
        Ok(result)
    }

    fn insert_one_way_connection(&mut self, from: Node<'a>, to: Node<'a>) {
        match self.0.entry(from) {
            Entry::Occupied(mut entry) => {
                let connections = entry.get_mut();
                if !connections.iter().any(|other| *other == to) {
                    connections.push(to);
                }
            }
            Entry::Vacant(entry) => {
                entry.insert(vec![to]);
            }
        }
    }

    fn insert_connection(&mut self, from: Node<'a>, to: Node<'a>) {
        self.insert_one_way_connection(from, to);
        self.insert_one_way_connection(to, from);
    }

    fn count_paths(&self) -> usize {
        let mut already_explored = HashSet::<Vec<Node<'a>>>::new();
        let mut explore_queue = VecDeque::<Vec<Node<'a>>>::new();
        explore_queue.push_back(vec![Node::Start]);
        let mut complete_paths: usize = 0;
        while let Some(path) = explore_queue.pop_front() {
            if already_explored.contains(&path) {
                continue;
            }
            already_explored.insert(path.clone());
            let last_node = *path.last().unwrap();
            if last_node == Node::End {
                complete_paths += 1;
            } else {
                if let Some(connections) = self.0.get(&last_node) {
                    for &next in connections {
                        let next_node_small_cave_already_visited = || {
                            if let Node::SmallCave(id) = next {
                                path.contains(&Node::SmallCave(id))
                            } else {
                                false
                            }
                        };
                        if next != Node::Start && !next_node_small_cave_already_visited() {
                            let mut new_path = path.clone();
                            new_path.push(next);
                            explore_queue.push_back(new_path);
                        }
                    }
                }
            }
        }
        complete_paths
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_paths() {
        let map = ConnectionMap::from_lines([
            "start-A", "start-b", "A-c", "A-b", "b-d", "A-end", "b-end",
        ])
        .unwrap();
        let result = map.count_paths();
        assert_eq!(result, 10);
    }

    #[test]
    fn test_count_paths_larger() {
        let map = ConnectionMap::from_lines([
            "dc-end", "HN-start", "start-kj", "dc-start", "dc-HN", "LN-dc", "HN-end", "kj-sa",
            "kj-HN", "kj-dc",
        ])
        .unwrap();
        let result = map.count_paths();
        assert_eq!(result, 19);
    }

    #[test]
    fn test_count_paths_largest() {
        let map = ConnectionMap::from_lines([
            "fs-end", "he-DX", "fs-he", "start-DX", "pj-DX", "end-zg", "zg-sl", "zg-pj", "pj-he",
            "RW-he", "fs-DX", "pj-RW", "zg-RW", "start-pj", "he-WI", "zg-he", "pj-fs", "start-RW",
        ])
        .unwrap();
        let result = map.count_paths();
        assert_eq!(result, 226);
    }

    #[test]
    fn part_one_answer() {
        let result = part_one();
        assert_eq!(result, 4912);
    }
}
