// Day 18: Snailfish
use crate::prelude::*;
use anyhow::{anyhow, bail, Error, Result};
use std::fmt::{Debug, Write};
use std::{fmt::Display, iter::Peekable, ops::Add, str::FromStr, sync::Arc};

lazy_static! {
    static ref PUZZLE_INPUT: Box<[&'static str]> = include_lines!("day00_input.txt").collect();
}

pub fn part_one() -> String {
    format!("Hello world! ({})", PUZZLE_INPUT.len())
}

type Int = u32;

macro_rules! element_literal {
    ([$left:tt, $right:tt]) => {
        Element::Pair(snailfish_number!([$left, $right]))
    };
    ($num:tt) => {
        Element::Regular($num)
    };
}

macro_rules! snailfish_number {
    ([$left:tt,$right:tt]) => {
        SnailfishNumber::new(element_literal!($left), element_literal!($right))
    };
}

#[derive(Clone, PartialEq, Eq)]
struct SnailfishNumber(Arc<(Element, Element)>);

impl SnailfishNumber {
    fn new(left: Element, right: Element) -> SnailfishNumber {
        SnailfishNumber(Arc::new((left, right)))
    }

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

        Ok(SnailfishNumber::new(left, right))
    }

    fn left(&self) -> &Element {
        &self.0 .0
    }
    fn right(&self) -> &Element {
        &self.0 .1
    }

    fn as_pair(&self) -> (&Element, &Element) {
        (self.left(), self.right())
    }

    fn reduce(&self) -> SnailfishNumber {
        todo!()
    }

    fn try_explode(&self, depth: usize) -> Option<ExplodeResult> {
        let pair_to_explode = if depth >= 4 {
            match (self.left(), self.right()) {
                (Element::Regular(left), Element::Regular(right)) => Some((left, right)),
                _ => None,
            }
        } else {
            None
        };

        if let Some((left, right)) = pair_to_explode {
            Some(ExplodeResult {
                new: element_literal!(0),
                left: Some(*left),
                right: Some(*right),
            })
        } else if let Some(exploded_on_left) = self.left().try_explode(depth + 1) {
            let new_right = if let Some(new_right) = exploded_on_left.right {
                self.right().try_receive_explosion_right(new_right)
            } else {
                None
            };
            if let Some(new_right) = new_right {
                let new_number = SnailfishNumber::new(exploded_on_left.new, new_right);
                Some(ExplodeResult {
                    new: Element::Pair(new_number),
                    left: exploded_on_left.left,
                    right: None,
                })
            } else {
                let new_number = SnailfishNumber::new(exploded_on_left.new, self.right().clone());
                Some(ExplodeResult {
                    new: Element::Pair(new_number),
                    left: exploded_on_left.left,
                    right: exploded_on_left.right,
                })
            }
        } else if let Some(exploded_on_right) = self.right().try_explode(depth + 1) {
            let new_left = if let Some(new_left) = exploded_on_right.left {
                self.left().try_receive_explosion_left(new_left)
            } else {
                None
            };
            if let Some(new_left) = new_left {
                let new_number = SnailfishNumber::new(new_left, exploded_on_right.new);
                Some(ExplodeResult {
                    new: Element::Pair(new_number),
                    left: None,
                    right: exploded_on_right.right,
                })
            } else {
                let new_number = SnailfishNumber::new(self.left().clone(), exploded_on_right.new);
                Some(ExplodeResult {
                    new: Element::Pair(new_number),
                    left: exploded_on_right.left,
                    right: exploded_on_right.right,
                })
            }
        } else {
            None
        }
    }
}

impl Add for SnailfishNumber {
    type Output = SnailfishNumber;

    fn add(self, rhs: Self) -> Self::Output {
        SnailfishNumber::new(Element::Pair(self.clone()), Element::Pair(rhs.clone()))
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
        write!(f, "[{},{}]", self.left(), self.right())
    }
}

impl Debug for SnailfishNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.to_string().as_str())
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Element {
    Pair(SnailfishNumber),
    Regular(Int),
}

impl Element {
    fn new_regular(number: Int) -> Element {
        Element::Regular(number)
    }

    fn new_pair(left: Element, right: Element) -> Element {
        Element::Pair(SnailfishNumber::new(left, right))
    }

    fn parse_from_stream(stream: &mut Peekable<impl Iterator<Item = char>>) -> Result<Element> {
        if stream.peek() == Some(&'[') {
            let snailfish_number = SnailfishNumber::parse_from_stream(stream)?;
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

    fn try_explode(&self, depth: usize) -> Option<ExplodeResult> {
        match self {
            Element::Pair(snailfish_number) => snailfish_number.try_explode(depth),
            Element::Regular(_) => None,
        }
    }

    fn try_receive_explosion_left(&self, number: Int) -> Option<Element> {
        match self {
            Element::Pair(pair) => {
                if let Some(result) = pair.right().try_receive_explosion_left(number) {
                    Some(Element::new_pair(pair.left().clone(), result))
                } else if let Some(result) = pair.left().try_receive_explosion_left(number) {
                    Some(Element::new_pair(result, pair.right().clone()))
                } else {
                    None
                }
            }
            Element::Regular(old_number) => Some(Element::Regular(old_number + number)),
        }
    }

    fn try_receive_explosion_right(&self, number: Int) -> Option<Element> {
        match self {
            Element::Pair(pair) => {
                if let Some(result) = pair.left().try_receive_explosion_right(number) {
                    Some(Element::new_pair(result, pair.right().clone()))
                } else if let Some(result) = pair.right().try_receive_explosion_right(number) {
                    Some(Element::new_pair(pair.left().clone(), result))
                } else {
                    None
                }
            }
            Element::Regular(old_number) => Some(Element::Regular(old_number + number)),
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

#[derive(Debug, Clone, PartialEq, Eq)]
struct ExplodeResult {
    new: Element,
    left: Option<Int>,
    right: Option<Int>,
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

    fn assert_reduce(input_str: &str, expected_str: &str) {
        let expected: SnailfishNumber = expected_str.parse().unwrap();
        let input: SnailfishNumber = input_str.parse().unwrap();
        assert_eq!(input.reduce(), expected);
    }

    #[test]
    fn test_single_explode() {
        fn assert_explode(input: SnailfishNumber, expected: SnailfishNumber) {
            assert_eq!(
                input.try_explode(0).unwrap().new,
                Element::Pair(expected.clone())
            )
        }
        assert_explode(
            snailfish_number!([[[[[9, 8], 1], 2], 3], 4]),
            snailfish_number!([[[[0, 9], 2], 3], 4]),
        );
        assert_explode(
            snailfish_number!([7, [6, [5, [4, [3, 2]]]]]),
            snailfish_number!([7, [6, [5, [7, 0]]]]),
        );
        assert_explode(
            snailfish_number!([[6, [5, [4, [3, 2]]]], 1]),
            snailfish_number!([[6, [5, [7, 0]]], 3]),
        );
        assert_explode(
            snailfish_number!([[3, [2, [1, [7, 3]]]], [6, [5, [4, [3, 2]]]]]),
            snailfish_number!([[3, [2, [8, 0]]], [9, [5, [4, [3, 2]]]]]),
        );
        assert_explode(
            snailfish_number!([[3, [2, [8, 0]]], [9, [5, [4, [3, 2]]]]]),
            snailfish_number!([[3, [2, [8, 0]]], [9, [5, [7, 0]]]]),
        );
    }

    #[test]
    fn part_one_answer() {
        let result = part_one();
        assert_eq!(result, "Hello world! (3)");
    }
}
