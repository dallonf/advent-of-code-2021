use super::digit::{self, DigitDisplay, Segment, SegmentState, ALL_SEGMENTS, DIGITS};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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

impl Solution {
    pub fn solve(scrambled_examples: &[DigitDisplay]) -> Result<Self, String> {
        let mut partial_solution = PartialSolution {
            possibilities: [digit::ALL_ON; ALL_SEGMENTS.len()],
        };

        // First: use any simple digits to narrow down the search
        let simple_digit_examples = scrambled_examples.iter().filter(|&example| {
            example.is_simple_digit() &&
            // sorry 8, you're not helpful
             *example != digit::ALL_ON
        });
        for example in simple_digit_examples {
            let matching_digit = DIGITS
                .iter()
                .find(|&it| it.count_segments_on() == example.count_segments_on())
                .unwrap();
            let matching_digit_inverted = matching_digit.inverted();
            // each of the segments in the scrambled example must be a segment in the matching digit...
            // and each of the segments _off_ must be an off segment!
            for SegmentState(segment, on) in example.segment_states() {
                let possibilities_for_segment =
                    &mut partial_solution.possibilities[segment.as_index()];
                *possibilities_for_segment = possibilities_for_segment.intersect(if on {
                    matching_digit
                } else {
                    &matching_digit_inverted
                });
            }
        }

        // Finally, brute force possible solutions
        let possible_solutions = partial_solution.brute_force();

        // Find first possible solution that can turn all examples into recognizable digits
        possible_solutions
            .into_iter()
            .find(|solution| {
                scrambled_examples
                    .iter()
                    .all(|example| example.decode_digit(solution).is_ok())
            })
            .ok_or("Couldn't find a solution".to_string())
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct PartialSolution {
    possibilities: [DigitDisplay; ALL_SEGMENTS.len()],
}

impl PartialSolution {
    fn deduce_until_stable(&self) -> PartialSolution {
        let mut current = *self;
        let mut prev = current;
        loop {
            let solved: Box<[_]> = current
                .possibilities
                .iter()
                .enumerate()
                .filter_map(|(i, possibilities)| {
                    let segments: Box<[Segment]> = possibilities.segments_on().collect();
                    if segments.len() == 1 {
                        Some((i, segments[0]))
                    } else {
                        None
                    }
                })
                .collect();

            for (solution_segment_index, segment_possibilities) in
                current.possibilities.iter_mut().enumerate()
            {
                // remove possible segments that are solved for other solution segments
                *segment_possibilities =
                    DigitDisplay::from_segments(segment_possibilities.segments_on().filter(
                        |possible_segment| {
                            !solved
                                .iter()
                                .filter(|(solved_segment_index, _)| {
                                    *solved_segment_index != solution_segment_index
                                })
                                .any(|(_, solved_segment)| possible_segment == solved_segment)
                        },
                    ))
            }

            if current == prev {
                break;
            }
            prev = current;
        }
        current
    }

    // profile: is returning a vec faster than a boxed iterator?
    // also, why couldn't I use a boxed slice? (Box<[Solution]>)
    fn brute_force(&self) -> Vec<Solution> {
        let deduced = self.deduce_until_stable();
        // TODO: check if any segment has no possibilities
        if let Ok(solution) = deduced.into_solution() {
            return vec![solution];
        }

        let first_unsolved = deduced
            .possibilities
            .iter()
            .enumerate()
            .find(|(_, possibilities_for_segment)| {
                possibilities_for_segment.count_segments_on() > 1
            })
            .unwrap();

        first_unsolved
            .1
            .segments_on()
            .flat_map(|segment| {
                let mut option = deduced;
                option.possibilities[first_unsolved.0] = DigitDisplay::from_single_segment(segment);
                option.brute_force()
            })
            .collect()
    }

    fn into_solution(&self) -> Result<Solution, String> {
        let mapping_results: Box<[Result<Segment, _>]> = self
            .possibilities
            .into_iter()
            .enumerate()
            .map(|(i, segment_possibilities)| {
                let segments_on: Box<[Segment]> = segment_possibilities.segments_on().collect();
                let segment = Segment::try_from(i as u8).unwrap();
                match &segments_on[..] {
                    [single] => Ok(*single),
                    _ => Err(format!(
                        "Need exactly one possibility for {:?}: {:?}",
                        segment, segments_on
                    )),
                }
            })
            .collect();
        let errors: Box<[&str]> = mapping_results
            .iter()
            .filter_map(|result| match result {
                Ok(_) => None,
                Err(err) => Some(err.as_str()),
            })
            .collect();

        if errors.len() > 0 {
            return Err(format!(
                "Couldn't find a solution: {:?}",
                self.possibilities
            ));
        }

        let mut mapping = [Segment::A; ALL_SEGMENTS.len()];
        // since we checked for errors above, we know that we have the right size of results
        for (i, segment) in mapping_results
            .into_iter()
            .map(|it| it.as_ref().unwrap())
            .enumerate()
        {
            mapping[i] = *segment;
        }
        Ok(Solution { mapping })
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

    #[test]
    fn test_solve() {
        let examples = [
            "acedgfb", "cdfbe", "gcdfa", "fbcad", "dab", "cefabd", "cdfgeb", "eafb", "cagedb", "ab",
        ]
        .into_iter()
        .map(|it| DigitDisplay::from_str(it))
        .collect::<Result<Box<[DigitDisplay]>, _>>()
        .unwrap();
        let result = Solution::solve(&examples);
        assert_eq!(
            result,
            Ok(Solution {
                mapping: [
                    Segment::C,
                    Segment::F,
                    Segment::G,
                    Segment::A,
                    Segment::B,
                    Segment::D,
                    Segment::E,
                ],
            })
        );
    }
}
