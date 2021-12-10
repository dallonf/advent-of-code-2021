use std::collections::VecDeque;

// Day 10: Syntax Scoring
use crate::prelude::*;

lazy_static! {
    static ref PUZZLE_INPUT: Box<[&'static str]> = include_lines!("day10_input.txt").collect();
}

pub fn part_one() -> Result<u32, String> {
    compute_score(PUZZLE_INPUT.iter().copied())
}

#[derive(Debug, PartialEq, Eq)]
enum ParseResult {
    Valid,
    UnexpectedToken(char),
    Incomplete,
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
        ParseResult::Incomplete
    }
}

fn compute_score<'a, T>(lines: T) -> Result<u32, String>
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
            ParseResult::Incomplete
        );
    }

    #[test]
    fn test_score() {
        let result = compute_score(EXAMPLE_INPUT.iter().copied());
        assert_eq!(result, Ok(26397));
    }

    #[test]
    fn part_one_answer() {
        let result = part_one();
        assert_eq!(result, Ok(367059));
    }
}
