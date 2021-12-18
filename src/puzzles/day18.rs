// Day 18: Snailfish
use crate::prelude::*;
use anyhow::{anyhow, bail, Error, Result};
use std::borrow::Cow;
use std::fmt::Debug;
use std::{fmt::Display, iter::Peekable, ops::Add, str::FromStr, sync::Arc};

lazy_static! {
    static ref PUZZLE_INPUT: Box<[SnailfishNumber]> = include_lines!("day18_input.txt")
        .map(|it| it.parse().unwrap())
        .collect();
}

pub fn part_one() -> u32 {
    SnailfishNumber::sum(PUZZLE_INPUT.iter())
        .unwrap()
        .magnitude()
}

type Digit = u8;

macro_rules! element_literal {
    ([$left:tt, $right:tt]) => {
        Element::Pair(snailfish_num!([$left, $right]))
    };
    ($num:tt) => {
        Element::Regular($num)
    };
}

macro_rules! snailfish_num {
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

    fn sum<'a>(numbers: impl IntoIterator<Item = &'a SnailfishNumber>) -> Option<SnailfishNumber> {
        numbers
            .into_iter()
            .map(|it| Cow::Borrowed(it))
            .reduce(|prev, next| Cow::Owned(prev.add_and_reduce(&next)))
            .map(|it| it.into_owned())
    }

    fn left(&self) -> &Element {
        &self.0 .0
    }
    fn right(&self) -> &Element {
        &self.0 .1
    }

    fn update_left(&self, new: Element) -> SnailfishNumber {
        SnailfishNumber::new(new, self.right().clone())
    }

    fn update_right(&self, new: Element) -> SnailfishNumber {
        SnailfishNumber::new(self.left().clone(), new)
    }

    fn as_pair(&self) -> (&Element, &Element) {
        (self.left(), self.right())
    }

    fn reduce(&self) -> Cow<SnailfishNumber> {
        let mut current = Cow::Borrowed(self);
        loop {
            if let Some(new) = current.try_explode(0) {
                let new = if let Element::Pair(new) = new.new {
                    new
                } else {
                    panic!("Somehow exploded so hard that the top pair became a single number!")
                };
                current = Cow::Owned(new)
            } else if let Some(new) = current.try_split() {
                current = Cow::Owned(new)
            } else {
                break;
            }
        }
        current
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

    fn try_split(&self) -> Option<SnailfishNumber> {
        if let Some(new_left) = self.left().try_split() {
            Some(self.update_left(new_left))
        } else if let Some(new_right) = self.right().try_split() {
            Some(self.update_right(new_right))
        } else {
            None
        }
    }

    fn add_and_reduce(&self, other: &SnailfishNumber) -> SnailfishNumber {
        (self + other).reduce().into_owned()
    }

    fn magnitude(&self) -> u32 {
        self.left().magnitude() * 3 + self.right().magnitude() * 2
    }
}

impl Add for &SnailfishNumber {
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
    Regular(Digit),
}

impl Element {
    fn new_regular(number: Digit) -> Element {
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

    fn try_split(&self) -> Option<Element> {
        match self {
            Element::Pair(snailfish_number) => {
                snailfish_number.try_split().map(|it| Element::Pair(it))
            }
            &Element::Regular(number) if number >= 10 => {
                let left = number / 2;
                let right = number - left;
                Some(element_literal!([left, right]))
            }
            Element::Regular(_) => None,
        }
    }

    fn try_receive_explosion_left(&self, number: Digit) -> Option<Element> {
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

    fn try_receive_explosion_right(&self, number: Digit) -> Option<Element> {
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

    fn magnitude(&self) -> u32 {
        match self {
            Element::Pair(snailfish_num) => snailfish_num.magnitude(),
            Element::Regular(number) => *number as u32,
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
    left: Option<Digit>,
    right: Option<Digit>,
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
    fn test_single_explode() {
        fn assert_explode(input: SnailfishNumber, expected: SnailfishNumber) {
            assert_eq!(
                input.try_explode(0).unwrap().new,
                Element::Pair(expected.clone())
            )
        }
        assert_explode(
            snailfish_num!([[[[[9, 8], 1], 2], 3], 4]),
            snailfish_num!([[[[0, 9], 2], 3], 4]),
        );
        assert_explode(
            snailfish_num!([7, [6, [5, [4, [3, 2]]]]]),
            snailfish_num!([7, [6, [5, [7, 0]]]]),
        );
        assert_explode(
            snailfish_num!([[6, [5, [4, [3, 2]]]], 1]),
            snailfish_num!([[6, [5, [7, 0]]], 3]),
        );
        assert_explode(
            snailfish_num!([[3, [2, [1, [7, 3]]]], [6, [5, [4, [3, 2]]]]]),
            snailfish_num!([[3, [2, [8, 0]]], [9, [5, [4, [3, 2]]]]]),
        );
        assert_explode(
            snailfish_num!([[3, [2, [8, 0]]], [9, [5, [4, [3, 2]]]]]),
            snailfish_num!([[3, [2, [8, 0]]], [9, [5, [7, 0]]]]),
        );
    }

    #[test]
    fn test_reduce() {
        let start = snailfish_num!([[[[[4, 3], 4], 4], [7, [[8, 4], 9]]], [1, 1]]);
        let expected = snailfish_num!([[[[0, 7], 4], [[7, 8], [6, 0]]], [8, 1]]);
        assert_eq!(start.reduce(), Cow::Borrowed(&expected));
    }

    fn test_sum() {
        assert_eq!(
            SnailfishNumber::sum(
                [
                    snailfish_num!([1, 1]),
                    snailfish_num!([2, 2]),
                    snailfish_num!([3, 3]),
                    snailfish_num!([4, 4]),
                ]
                .iter(),
            ),
            Some(snailfish_num!([[[[1, 1], [2, 2]], [3, 3]], [4, 4]])),
        );
        assert_eq!(
            SnailfishNumber::sum(
                [
                    snailfish_num!([1, 1]),
                    snailfish_num!([2, 2]),
                    snailfish_num!([3, 3]),
                    snailfish_num!([4, 4]),
                    snailfish_num!([5, 5]),
                ]
                .iter(),
            ),
            Some(snailfish_num!([[[[3, 0], [5, 3]], [4, 4]], [5, 5]])),
        );
        assert_eq!(
            SnailfishNumber::sum(
                [
                    snailfish_num!([1, 1]),
                    snailfish_num!([2, 2]),
                    snailfish_num!([3, 3]),
                    snailfish_num!([4, 4]),
                    snailfish_num!([5, 5]),
                    snailfish_num!([6, 6]),
                ]
                .iter()
            ),
            Some(snailfish_num!([[[[5, 0], [7, 4]], [5, 5]], [6, 6]])),
        );
    }

    #[test]
    fn test_sum_large() {
        assert_eq!(
            SnailfishNumber::sum(
                [
                    snailfish_num!([[[0, [4, 5]], [0, 0]], [[[4, 5], [2, 6]], [9, 5]]]),
                    snailfish_num!([7, [[[3, 7], [4, 3]], [[6, 3], [8, 8]]]]),
                    snailfish_num!([[2, [[0, 8], [3, 4]]], [[[6, 7], 1], [7, [1, 6]]]]),
                    snailfish_num!([
                        [[[2, 4], 7], [6, [0, 5]]],
                        [[[6, 8], [2, 8]], [[2, 1], [4, 5]]]
                    ]),
                    snailfish_num!([7, [5, [[3, 8], [1, 4]]]]),
                    snailfish_num!([[2, [2, 2]], [8, [8, 1]]]),
                    snailfish_num!([2, 9]),
                    snailfish_num!([1, [[[9, 3], 9], [[9, 0], [0, 7]]]]),
                    snailfish_num!([[[5, [7, 4]], 7], 1]),
                    snailfish_num!([[[[4, 2], 2], 6], [8, 7]]),
                ]
                .iter()
            ),
            Some(snailfish_num!([
                [[[8, 7], [7, 7]], [[8, 6], [7, 7]]],
                [[[0, 7], [6, 6]], [8, 7]]
            ])),
        );
    }

    #[test]
    fn test_magnitude() {
        assert_eq!(snailfish_num!([9, 1]).magnitude(), 29);
        assert_eq!(snailfish_num!([[9, 1], [1, 9]]).magnitude(), 129);
        assert_eq!(snailfish_num!([[1, 2], [[3, 4], 5]]).magnitude(), 143);
        assert_eq!(
            snailfish_num!([[[[0, 7], 4], [[7, 8], [6, 0]]], [8, 1]]).magnitude(),
            1384
        );
        assert_eq!(
            snailfish_num!([[[[1, 1], [2, 2]], [3, 3]], [4, 4]]).magnitude(),
            445
        );
        assert_eq!(
            snailfish_num!([[[[3, 0], [5, 3]], [4, 4]], [5, 5]]).magnitude(),
            791
        );
        assert_eq!(
            snailfish_num!([[[[5, 0], [7, 4]], [5, 5]], [6, 6]]).magnitude(),
            1137
        );
        assert_eq!(
            snailfish_num!([
                [[[8, 7], [7, 7]], [[8, 6], [7, 7]]],
                [[[0, 7], [6, 6]], [8, 7]]
            ])
            .magnitude(),
            3488
        );
    }

    #[test]
    fn example_magnitude() {
        let numbers = [
            "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]",
            "[[[5,[2,8]],4],[5,[[9,9],0]]]",
            "[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]",
            "[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]",
            "[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]",
            "[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]",
            "[[[[5,4],[7,7]],8],[[8,3],8]]",
            "[[9,3],[[9,9],[6,[4,9]]]]",
            "[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]",
            "[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]",
        ]
        .into_iter()
        .map(|it| SnailfishNumber::from_str(it).unwrap())
        .collect_vec();

        let sum = SnailfishNumber::sum(numbers.iter());
        assert_eq!(
            sum,
            Some(snailfish_num!([
                [[[6, 6], [7, 6]], [[7, 7], [7, 0]]],
                [[[7, 7], [7, 7]], [[7, 8], [9, 9]]]
            ]))
        );
        assert_eq!(sum.unwrap().magnitude(), 4140);
    }

    #[test]
    fn part_one_answer() {
        let result = part_one();
        assert_eq!(result, 2907);
    }
}
