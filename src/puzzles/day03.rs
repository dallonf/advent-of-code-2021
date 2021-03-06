use core::fmt::Debug;
use std::str::FromStr;

// Day 3: Binary Diagnostic
use crate::prelude::*;

lazy_static! {
    static ref PUZZLE_INPUT: BinaryNumbers = include_lines!("day03_input.txt")
        .map(|line| line.parse().unwrap())
        .collect::<Result<BinaryNumbers, _>>()
        .unwrap();
    static ref MAX_BITS: usize = u16::BITS.try_into().unwrap();
}

pub fn part_one() -> Result<u32, String> {
    PowerConsumption::compute(&PUZZLE_INPUT).map(|it| it.output())
}

pub fn part_two() -> Result<u32, String> {
    get_life_support_rating(&PUZZLE_INPUT)
}

fn bit_iterator(bits: u8) -> impl DoubleEndedIterator<Item = u16> {
    (0..bits).map(|i| (1 as u16) << i)
}

fn combine_bits(a: u16, b: u16) -> u16 {
    a | b
}

#[derive(PartialEq, Eq, Clone, Copy)]
struct BinaryNumber {
    number: u16,
    num_bits: u8,
}

impl FromStr for BinaryNumber {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let num_bits = input.len();
        // ordinarily it's not a safe assumption that .len() (bytes)
        // is equal to the length of .chars() - but since we only allow
        // '0' and '1', which are in the ASCII set, we're fine
        if num_bits > *MAX_BITS {
            return Err(format!(
                "'{}' is too long to be parsed as a 16-bit integer",
                input
            ));
        }
        let result: u16 = input
            .chars()
            .rev()
            .enumerate()
            .map(|(i, c)| {
                let bit: u16 = match c {
                    '0' => 0,
                    '1' => 1,
                    _ => return Err(format!("Unexpected character: {}", c)),
                };
                Ok(bit << i)
            })
            .reduce(|a, b| match (a, b) {
                (Ok(a), Ok(b)) => Ok(a | b),
                (Err(err), _) | (_, Err(err)) => Err(err),
            })
            .unwrap()?;

        Ok(BinaryNumber {
            number: result,
            num_bits: num_bits.try_into().unwrap(),
        })
    }
}

impl Debug for BinaryNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let binary_format: String = bit_iterator(self.num_bits)
            .map(|bit| if bit & self.number == bit { '1' } else { '0' })
            .rev()
            .collect();

        f.write_fmt(format_args!("{} ({})", self.number, binary_format))
    }
}

struct BinaryNumbers {
    num_bits: u8,
    numbers: Box<[u16]>,
}

impl FromIterator<BinaryNumber> for Result<BinaryNumbers, String> {
    fn from_iter<T: IntoIterator<Item = BinaryNumber>>(input: T) -> Self {
        let mut iter = input.into_iter();
        let first = iter.next().ok_or("No items in list".to_string())?;
        let num_bits = first.num_bits;
        let mut numbers = vec![first.number];
        for it in iter {
            if it.num_bits != num_bits {
                return Err("Expected all input numbers to be the same bit length".to_string());
            }
            numbers.push(it.number);
        }
        Ok(BinaryNumbers {
            num_bits,
            numbers: numbers.into(),
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct PowerConsumption {
    gamma_rate: BinaryNumber,
    epsilon_rate: BinaryNumber,
}

impl PowerConsumption {
    fn compute(input: &BinaryNumbers) -> Result<Self, String> {
        let num_bits = input.num_bits;
        let numbers: &[u16] = &input.numbers;
        let gamma: u16 = bit_iterator(num_bits)
            .filter_map(|bit| {
                let on_count = numbers.iter().filter(|&&num| num & bit == bit).count();
                if on_count > (numbers.len() / 2) {
                    Some(bit)
                } else {
                    None
                }
            })
            .reduce(combine_bits)
            .unwrap();

        let epsilon: u16 = {
            let max: u16 = bit_iterator(num_bits).reduce(combine_bits).unwrap();
            !gamma & max
        };

        Ok(Self {
            gamma_rate: BinaryNumber {
                number: gamma,
                num_bits,
            },
            epsilon_rate: BinaryNumber {
                number: epsilon,
                num_bits,
            },
        })
    }

    fn output(&self) -> u32 {
        self.gamma_rate.number as u32 * self.epsilon_rate.number as u32
    }
}

fn get_life_support_rating(numbers: &BinaryNumbers) -> Result<u32, String> {
    let oxygen = get_oxygen_generator_rating(numbers)?;
    let co2 = get_co2_scrubber_rating(numbers)?;

    Ok(oxygen.number as u32 * co2.number as u32)
}

fn get_oxygen_generator_rating(numbers: &BinaryNumbers) -> Result<BinaryNumber, String> {
    find_with_bit_criteria(numbers, |num_on, num_off| num_on >= num_off)
}

fn get_co2_scrubber_rating(numbers: &BinaryNumbers) -> Result<BinaryNumber, String> {
    find_with_bit_criteria(numbers, |num_on, num_off| num_off > num_on)
}

// The expected bit value
type BitCriteria = fn(num_on: usize, num_off: usize) -> bool;

fn find_with_bit_criteria(
    numbers: &BinaryNumbers,
    bit_criteria: BitCriteria,
) -> Result<BinaryNumber, String> {
    let num_bits = numbers.num_bits;
    let mut remaining_items = numbers.numbers.clone();
    let mut bit_iterator = bit_iterator(num_bits).rev(); // a biterator, if you will
    while remaining_items.len() > 1 {
        let bit = bit_iterator
            .next()
            .ok_or("Couldn't find a single matching value")?;

        let count = remaining_items
            .iter()
            .filter(|&&num| num & bit == bit)
            .count();

        let expected_bit = bit_criteria(count, remaining_items.len() - count);
        // filter out remaining items that don't match
        remaining_items = remaining_items
            .into_iter()
            .copied()
            .filter(|&num| (num & bit == bit) == expected_bit)
            .collect();
    }
    let number = remaining_items[0];
    Ok(BinaryNumber { num_bits, number })
}

#[cfg(test)]
mod tests {
    use super::*;

    lazy_static! {
        static ref EXAMPLE_INPUT: BinaryNumbers = [
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ]
        .into_iter()
        .map(|line| line.parse().unwrap())
        .collect::<Result<BinaryNumbers, _>>()
        .unwrap();
    }

    #[test]
    fn test_parse_as_binary() {
        assert_eq!(
            "10110".parse(),
            Ok(BinaryNumber {
                number: 22,
                num_bits: 5
            })
        );
        assert_eq!(
            "01001".parse(),
            Ok(BinaryNumber {
                number: 9,
                num_bits: 5
            })
        );
    }

    #[test]
    fn test_compute() {
        let result = PowerConsumption::compute(&EXAMPLE_INPUT).unwrap();
        assert_eq!(
            result,
            PowerConsumption {
                gamma_rate: "10110".parse().unwrap(),
                epsilon_rate: "01001".parse().unwrap(),
            }
        );
        assert_eq!(result.output(), 198);
    }

    #[test]
    fn part_one_answer() {
        let result = part_one().unwrap();
        assert_eq!(result, 4006064);
    }

    #[test]
    fn oxygen_generator_rating() {
        let result = get_oxygen_generator_rating(&EXAMPLE_INPUT).unwrap();
        assert_eq!(
            result,
            BinaryNumber {
                num_bits: 5,
                number: 23
            }
        )
    }

    #[test]
    fn co2_scrubber_rating() {
        let result = get_co2_scrubber_rating(&EXAMPLE_INPUT).unwrap();
        assert_eq!(
            result,
            BinaryNumber {
                num_bits: 5,
                number: 10
            }
        )
    }

    #[test]
    fn life_support_rating() {
        let result = get_life_support_rating(&EXAMPLE_INPUT).unwrap();
        assert_eq!(result, 230)
    }

    #[test]
    fn part_two_answer() {
        let result = part_two().unwrap();
        assert_eq!(result, 5941884);
    }
}
