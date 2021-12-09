use crate::puzzles::day08::digit::DIGITS;

use super::digit::{DigitDisplay, Segment, ALL_SEGMENTS};

pub struct Solution {
    mapping: [Segment; ALL_SEGMENTS.len()],
}

pub trait Decode {
    fn decode(&self, solution: &Solution) -> Self;
    fn decode_digit(&self, solution: &Solution) -> Result<u8, String>;
}

impl Decode for DigitDisplay {
    fn decode(&self, solution: &Solution) -> Self {
        DigitDisplay::from_segments(
            self.segments_on()
                .map(|from_segment| solution.mapping[from_segment.as_index()]),
        )
    }

    fn decode_digit(&self, solution: &Solution) -> Result<u8, String> {
        let decoded_display = self.decode(solution);
        let index = DIGITS.iter().enumerate().find_map(|(i, possible_digit)| {
            if decoded_display == *possible_digit {
                Some(i)
            } else {
                None
            }
        });
        if let Some(index) = index {
            Ok(index as u8)
        } else {
            Err(format!(
                "Couldn't find a matching digit for decoded display: {:?}",
                decoded_display
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test_decode() {
        let solution = Solution {
            mapping: [
                Segment::C,
                Segment::F,
                Segment::G,
                Segment::A,
                Segment::B,
                Segment::D,
                Segment::E,
            ],
        };
        let scrambled = DigitDisplay::from_str("cdfbe").unwrap();
        assert_eq!(scrambled.decode_digit(&solution), Ok(5));
    }
}
