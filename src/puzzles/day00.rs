// Day 00: Template

use crate::shared::input::include_lines;

pub fn part_one() -> String {
    let input = include_lines!("day00_input.txt");
    format!("Hello world! ({})", input.len())
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
