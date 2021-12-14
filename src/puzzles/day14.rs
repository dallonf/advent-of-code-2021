use std::{borrow::Cow, collections::HashMap};

// Day 0: Template
use crate::prelude::*;

lazy_static! {
    static ref PUZZLE_INPUT: Input = Input::from_lines(include_lines!("day14_input.txt")).unwrap();
}

pub fn part_one() -> usize {
    let polymer = PUZZLE_INPUT.steps(10);
    frequency_score(&polymer)
}

struct Input {
    polymer: Vec<char>,
    insertion_rules: HashMap<(char, char), char>,
}

impl Input {
    fn from_lines<'a, T: IntoIterator<Item = &'a str>>(iter: T) -> Result<Self, String> {
        let mut iter = iter.into_iter();
        let polymer = iter.next().ok_or("Empty file")?.chars().collect();
        match iter.next() {
            Some("") => (),
            _ => return Err("Expected empty line after polymer template".to_string()),
        }
        let insertion_rules = iter
            .map(|line| {
                let (pair, insert) = line.split_once(" -> ").ok_or("Expected ->".to_string())?;
                let pair = pair.chars().collect_vec();
                let insert = insert.chars().collect_vec();
                if pair.len() != 2 || insert.len() != 1 {
                    return Err(format!("bad pair insertion rule: {}", line));
                }
                let pair = (pair[0], pair[1]);
                let insert = insert[0];
                Ok((pair, insert))
            })
            .collect::<Result<HashMap<_, _>, _>>()?;
        Ok(Input {
            polymer,
            insertion_rules,
        })
    }

    fn step(&self, polymer: &[char]) -> Vec<char> {
        let insertions = polymer
            .windows(2)
            .map(|pair| self.insertion_rules.get(&(pair[0], pair[1])).map(|it| *it));

        itertools::interleave(polymer.iter().map(|it| Some(*it)), insertions)
            .filter_map(|it| it)
            .collect()
    }

    fn steps(&self, count: usize) -> Vec<char> {
        (0..count)
            .fold(Cow::Borrowed(&self.polymer), |current, _| {
                Cow::Owned(self.step(&current))
            })
            .into_owned()
    }
}

fn frequency_score(input: &[char]) -> usize {
    let frequency_map = input.iter().copied().counts();
    let most_frequent = frequency_map
        .iter()
        .map(|(_, count)| *count)
        .max()
        .unwrap_or(0);
    let least_frequent = frequency_map
        .iter()
        .map(|(_, count)| *count)
        .min()
        .unwrap_or(0);
    most_frequent - least_frequent
}

#[cfg(test)]
mod tests {
    use super::*;

    lazy_static! {
        static ref EXAMPLE_INPUT: Input = Input::from_lines(
            [
                "NNCB", "", "CH -> B", "HH -> N", "CB -> H", "NH -> C", "HB -> C", "HC -> B",
                "HN -> C", "NN -> C", "BH -> H", "NC -> B", "NB -> B", "BN -> B", "BB -> N",
                "BC -> B", "CC -> N", "CN -> C",
            ]
            .into_iter()
        )
        .unwrap();
    }

    #[test]
    fn test_parse() {
        assert_eq!(EXAMPLE_INPUT.polymer.len(), 4);
        assert_eq!(EXAMPLE_INPUT.insertion_rules.len(), 16);
    }

    #[test]
    fn test_follow_step() {
        let result = EXAMPLE_INPUT.step(&EXAMPLE_INPUT.polymer);
        assert_eq!(result, "NCNBCHB".chars().collect_vec());
    }

    #[test]
    fn test_steps() {
        let result = EXAMPLE_INPUT.steps(10);
        assert_eq!(result.len(), 3073);
    }

    #[test]
    fn test_frequency_score() {
        let final_polymer = EXAMPLE_INPUT.steps(10);
        let result = frequency_score(&final_polymer);
        assert_eq!(result, 1588);
    }

    #[test]
    fn part_one_answer() {
        let result = part_one();
        assert_eq!(result, 2851);
    }
}
