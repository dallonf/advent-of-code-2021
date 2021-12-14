use std::collections::VecDeque;

// Day 10: Syntax Scoring
use crate::prelude::*;

lazy_static! {
    static ref PUZZLE_INPUT: Box<[&'static str]> = include_lines!("day10_input.txt").collect();
}

pub fn part_one() -> Result<u32, String> {
    compute_syntax_score(PUZZLE_INPUT.iter().copied())
}

pub fn part_two() -> Result<u64, String> {
    compute_autocomplete_score(PUZZLE_INPUT.iter().copied())
}

#[derive(Debug, PartialEq, Eq)]
enum ParseResult {
    Valid,
    UnexpectedToken(char),
    Incomplete { stack: Vec<char> },
}

fn expected_closing_token(opening_token: char) -> char {
    match opening_token {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => panic!("invalid opening token: {}", opening_token),
    }
}

fn parse_line(line: &str) -> ParseResult {
    let mut chunk_stack = VecDeque::<char>::new();
    for token in line.chars() {
        let expected_closing = chunk_stack.back().map(|c| expected_closing_token(*c));
        if expected_closing == Some(token) {
            chunk_stack.pop_back();
        } else {
            match token {
                '(' | '[' | '{' | '<' => chunk_stack.push_back(token),
                _ => return ParseResult::UnexpectedToken(token),
            }
        }
    }
    if chunk_stack.is_empty() {
        ParseResult::Valid
    } else {
        ParseResult::Incomplete {
            stack: chunk_stack.into(),
        }
    }
}

fn compute_syntax_score<'a, T>(lines: T) -> Result<u32, String>
where
    T: IntoIterator<Item = &'a str>,
{
    lines
        .into_iter()
        .map(parse_line)
        .map(|result| match result {
            ParseResult::UnexpectedToken(')') => Ok(3),
            ParseResult::UnexpectedToken(']') => Ok(57),
            ParseResult::UnexpectedToken('}') => Ok(1197),
            ParseResult::UnexpectedToken('>') => Ok(25137),
            ParseResult::UnexpectedToken(other_char) => {
                Err(format!("invalid token: {}", other_char))
            }
            _ => Ok(0),
        })
        .try_fold(0, |prev, next| next.map(|next| prev + next))
}

fn compute_autocomplete_score<'a, T: IntoIterator<Item = &'a str>>(
    lines: T,
) -> Result<u64, String> {
    let scores = lines
        .into_iter()
        .map(|line| autocomplete_score_for_result(&parse_line(line)))
        .collect::<Result<Vec<Option<_>>, _>>()?;
    let scores = scores
        .into_iter()
        .filter_map(|it| it)
        .sorted()
        .collect_vec();

    if scores.len() == 0 {
        return Err("No lines autocompleted".to_string());
    }

    Ok(scores[scores.len() / 2])
}

fn autocomplete_score_for_result(parse_result: &ParseResult) -> Result<Option<u64>, String> {
    if let ParseResult::Incomplete { stack } = parse_result {
        stack
            .iter()
            .rev()
            .map(|c| match c {
                '(' => Ok(1),
                '[' => Ok(2),
                '{' => Ok(3),
                '<' => Ok(4),
                &other_char => Err(format!("invalid token: {}", other_char)),
            })
            .try_fold(0 as u64, |prev, next| Ok(prev * 5 + next?))
            .map(|it| Some(it))
    } else {
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: [&'static str; 10] = [
        "[({(<(())[]>[[{[]{<()<>>",
        "[(()[<>])]({[<{<<[]>>(",
        "{([(<{}[<>[]}>{[]{[(<()>",
        "(((({<>}<{<{<>}{[]{[]{}",
        "[[<[([]))<([[{}[[()]]]",
        "[{[{({}]{}}([{[{{{}}([]",
        "{<[[]]>}<{[{[{[]{()[[[]",
        "[<(<(<(<{}))><([]([]()",
        "<{([([[(<>()){}]>(<<{{",
        "<{([{{}}[<[[[<>{}]]]>[]]",
    ];

    #[test]
    fn test_corrupted_line() {
        assert_eq!(
            parse_line("{([(<{}[<>[]}>{[]{[(<()>"),
            ParseResult::UnexpectedToken('}')
        );
    }

    #[test]
    fn test_incomplete_line() {
        assert_eq!(
            parse_line("(((({<>}<{<{<>}{[]{[]{}"),
            ParseResult::Incomplete {
                stack: "((((<{<{{".chars().collect()
            }
        );
    }

    #[test]
    fn test_syntax_score() {
        let result = compute_syntax_score(EXAMPLE_INPUT.iter().copied());
        assert_eq!(result, Ok(26397));
    }

    #[test]
    fn part_one_answer() {
        let result = part_one();
        assert_eq!(result, Ok(367059));
    }

    #[test]
    fn test_autocomplete_score() {
        let result = autocomplete_score_for_result(&parse_line(EXAMPLE_INPUT[0]));
        assert_eq!(result, Ok(Some(288957)));
        let result = autocomplete_score_for_result(&parse_line(EXAMPLE_INPUT[1]));
        assert_eq!(result, Ok(Some(5566)));
    }

    #[test]
    fn test_autocomplete_aggregation() {
        let result = compute_autocomplete_score(EXAMPLE_INPUT.iter().copied());
        assert_eq!(result, Ok(288957));
    }

    #[test]
    fn part_two_answer() {
        let result = part_two();
        assert_eq!(result, Ok(1952146692));
    }
}
