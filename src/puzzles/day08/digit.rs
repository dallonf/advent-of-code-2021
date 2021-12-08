use crate::prelude::*;
use std::{fmt::Debug, str::FromStr};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct DigitDisplay {
    bitwise: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Segment {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SegmentState(Segment, bool);

const ALL_SEGMENTS: [Segment; 7] = [
    Segment::A,
    Segment::B,
    Segment::C,
    Segment::D,
    Segment::E,
    Segment::F,
    Segment::G,
];

pub const ZERO: DigitDisplay = DigitDisplay {
    bitwise: 0b01110111,
};
pub const ONE: DigitDisplay = DigitDisplay {
    bitwise: 0b00100100,
};
pub const TWO: DigitDisplay = DigitDisplay {
    bitwise: 0b01011101,
};
pub const THREE: DigitDisplay = DigitDisplay {
    bitwise: 0b01101101,
};
pub const FOUR: DigitDisplay = DigitDisplay {
    bitwise: 0b00101110,
};
pub const FIVE: DigitDisplay = DigitDisplay {
    bitwise: 0b01101011,
};
pub const SIX: DigitDisplay = DigitDisplay {
    bitwise: 0b01111011,
};
pub const SEVEN: DigitDisplay = DigitDisplay {
    bitwise: 0b00100101,
};
pub const EIGHT: DigitDisplay = DigitDisplay {
    bitwise: 0b01111111,
};
pub const NINE: DigitDisplay = DigitDisplay {
    bitwise: 0b01101111,
};

// digits that have a unique number of segments from all other digits
const SIMPLE_DIGITS: [DigitDisplay; 4] = [ONE, FOUR, SEVEN, EIGHT];

impl DigitDisplay {
    pub fn states<'a>(&'a self) -> impl Iterator<Item = SegmentState> + 'a {
        ALL_SEGMENTS.clone().into_iter().map(|segment| {
            let bit = segment.as_bit();
            SegmentState(segment, self.bitwise & bit == bit)
        })
    }

    pub fn segments_on<'a>(&'a self) -> impl Iterator<Item = Segment> + 'a {
        self.states()
            .filter_map(|it| if it.1 { Some(it.0) } else { None })
    }

    pub fn count_segments_on(&self) -> usize {
        self.segments_on().count()
    }

    pub fn is_simple_digit(&self) -> bool {
        let count = self.count_segments_on();
        // this could be hardcoded
        for test in &SIMPLE_DIGITS {
            if count == test.count_segments_on() {
                return true
            }
        }
        false
    }
}

impl FromStr for DigitDisplay {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bitwise = s
            .chars()
            .map::<Result<u8, String>, _>(|c| Ok(Segment::from_char(c)?.as_bit()))
            .reduce(|a, b| match (a, b) {
                (Ok(a), Ok(b)) => Ok(a | b),
                (Err(err), _) | (_, Err(err)) => Err(err),
            })
            .ok_or("Attempted to parse empty string as digit display")??;
        Ok(DigitDisplay { bitwise })
    }
}

impl Debug for DigitDisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DigitDisplay")
            .field("bitwise", &format!("{:#010b}", self.bitwise))
            .field(
                "segments_on",
                &self.segments_on().collect::<Box<[Segment]>>(),
            )
            .finish()
    }
}

impl Segment {
    fn from_char(c: char) -> Result<Segment, String> {
        match c {
            'a' => Ok(Segment::A),
            'b' => Ok(Segment::B),
            'c' => Ok(Segment::C),
            'd' => Ok(Segment::D),
            'e' => Ok(Segment::E),
            'f' => Ok(Segment::F),
            'g' => Ok(Segment::G),
            other => Err(format!("Could not parse '{}' as digit segment", other)),
        }
    }

    fn as_index(&self) -> u8 {
        *self as u8
    }

    fn as_bit(&self) -> u8 {
        1 << self.as_index()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing() {
        assert_eq!(DigitDisplay::from_str("abcefg"), Ok(ZERO));
        assert_eq!(DigitDisplay::from_str("cf"), Ok(ONE));
        assert_eq!(DigitDisplay::from_str("acdeg"), Ok(TWO));
        assert_eq!(DigitDisplay::from_str("acdfg"), Ok(THREE));
        assert_eq!(DigitDisplay::from_str("bcdf"), Ok(FOUR));
        assert_eq!(DigitDisplay::from_str("abdfg"), Ok(FIVE));
        assert_eq!(DigitDisplay::from_str("abdefg"), Ok(SIX));
        assert_eq!(DigitDisplay::from_str("acf"), Ok(SEVEN));
        assert_eq!(DigitDisplay::from_str("abcdefg"), Ok(EIGHT));
        assert_eq!(DigitDisplay::from_str("abcdfg"), Ok(NINE));
    }
}
