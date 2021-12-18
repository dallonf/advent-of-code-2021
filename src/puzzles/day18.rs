// Day 18: Snailfish
use crate::prelude::*;
use anyhow::{anyhow, bail, Error, Result};
use std::{fmt::Display, iter::Peekable, str::FromStr, sync::Arc};

lazy_static! {
    static ref PUZZLE_INPUT: Box<[&'static str]> = include_lines!("day00_input.txt").collect();
}

pub fn part_one() -> String {
    format!("Hello world! ({})", PUZZLE_INPUT.len())
}

struct SnailfishNumber(Element, Element);

impl SnailfishNumber {
    fn parse_from_stream(
        stream: &mut Peekable<impl Iterator<Item = char>>,
    ) -> Result<SnailfishNumber> {
        macro_rules! expect_char {
            ($stream:expr, $c:expr) => {
                if !$stream.next().map(|it| it == $c).unwrap_or(false) {
                    bail!("Expected '{}'", $c);
                }
            };
        }

        expect_char!(stream, '[');
        let left = Element::parse_from_stream(stream)?;
        expect_char!(stream, ',');
        let right = Element::parse_from_stream(stream)?;
        expect_char!(stream, ']');

        Ok(SnailfishNumber(left, right))
    }
}

impl FromStr for SnailfishNumber {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut chars_iter = s.chars().peekable();
        let result = Self::parse_from_stream(&mut chars_iter)?;
        if let Some(c) = chars_iter.next() {
            bail!("Expected end of string, but got a '{}' instead", c);
        }
        Ok(result)
    }
}

impl Display for SnailfishNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{},{}]", self.0, self.1)
    }
}

enum Element {
    Pair(Arc<SnailfishNumber>),
    Regular(u32),
}

impl Element {
    fn parse_from_stream(stream: &mut Peekable<impl Iterator<Item = char>>) -> Result<Element> {
        if stream.peek() == Some(&'[') {
            let snailfish_number = SnailfishNumber::parse_from_stream(stream)?;
            let snailfish_number = Arc::new(snailfish_number);
            Ok(Element::Pair(snailfish_number))
        } else {
            // assume only a single digit; numbers >=10 only exist during reduction
            let digit_str = stream
                .next()
                .ok_or(anyhow!("End of string while parsing a number"))?;
            let num = digit_str.to_string().parse()?;
            Ok(Element::Regular(num))
        }
    }
}

impl Display for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Element::Pair(snailfish_number) => write!(f, "{}", snailfish_number),
            Element::Regular(num) => write!(f, "{}", num),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_and_display() {
        fn assert_correct_parsing(input: &str) {
            let snailfish_number: SnailfishNumber = input.parse().unwrap();
            assert_eq!(snailfish_number.to_string(), input);
        }

        assert_correct_parsing("[1,2]");
        assert_correct_parsing("[[1,2],3]");
        assert_correct_parsing("[9,[8,7]]");
        assert_correct_parsing("[[1,9],[8,5]]");
        assert_correct_parsing("[[[[1,2],[3,4]],[[5,6],[7,8]]],9]");
        assert_correct_parsing("[[[9,[3,8]],[[0,9],6]],[[[3,7],[4,9]],3]]");
        assert_correct_parsing("[[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]]");
    }

    #[test]
    fn part_one_answer() {
        let result = part_one();
        assert_eq!(result, "Hello world! (3)");
    }
}
