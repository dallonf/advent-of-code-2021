// Day 0: Template
use crate::prelude::*;

lazy_static! {
    static ref PUZZLE_INPUT: Box<[&'static str]> = include_lines!("day00_input.txt").collect();
}

pub fn part_one() -> String {
    format!("Hello world! ({})", PUZZLE_INPUT.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_answer() {
        let result = part_one();
        assert_eq!(result, "Hello world! (3)");
    }
}
