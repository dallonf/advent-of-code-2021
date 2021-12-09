// Day 8: Seven Segment Search
mod digit;
mod solver;

use crate::prelude::*;
use digit::DigitDisplay;
use std::str::FromStr;

lazy_static! {
    static ref PUZZLE_INPUT: Box<[Entry]> = include_lines!("day08_input.txt")
        .map(|line| line.parse().unwrap())
        .collect();
}

pub fn part_one() -> usize {
    count_simple_digits_in_output(PUZZLE_INPUT.iter())
}

#[derive(Debug, Clone, Copy)]
struct Entry {
    patterns: [DigitDisplay; 10],
    output: [DigitDisplay; 4],
}

impl FromStr for Entry {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut words = s.split_whitespace();
        let mut patterns = [DigitDisplay::default(); 10];
        let mut output = [DigitDisplay::default(); 4];

        for i in 0..10 {
            patterns[i] = words
                .next()
                .ok_or("Ran out of items reading signal patterns".to_string())?
                .parse()?;
        }

        if words.next() != Some("|") {
            return Err("Expected '|' between signal patterns and output".to_string());
        }

        for i in 0..4 {
            output[i] = words
                .next()
                .ok_or("Ran out of items reading output value".to_string())?
                .parse()?;
        }

        if let Some(unexpected) = words.next() {
            return Err(format!(
                "Unexpected string after output values: {}",
                unexpected
            ));
        }

        Ok(Entry { output, patterns })
    }
}

fn count_simple_digits_in_output<'a, T>(example_input: T) -> usize
where
    T: IntoIterator<Item = &'a Entry>,
{
    example_input
        .into_iter()
        .flat_map(|entry| entry.output.iter())
        .filter(|digit| digit.is_simple_digit())
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    lazy_static! {
        static ref EXAMPLE_INPUT: Box<[Entry]> = [
            "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe",
            "edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc",
            "fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg",
            "fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb",
            "aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea",
            "fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb",
            "dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe",
            "bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef",
            "egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb",
            "gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce"
        ].into_iter().map(|line| line.parse().unwrap()).collect();
    }

    #[test]
    fn part_one_example() {
        let result = count_simple_digits_in_output(EXAMPLE_INPUT.iter());
        assert_eq!(result, 26);
    }

    #[test]
    fn part_one_answer() {
        let result = part_one();
        assert_eq!(result, 342);
    }
}
