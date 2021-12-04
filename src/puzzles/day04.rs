// Day 4: Giant Squid
use crate::prelude::*;

// lazy_static! {
//     static ref PUZZLE_INPUT: Box<[&'static str]> = include_lines!("day00_input.txt");
// }

pub fn part_one() -> String {
    "Hello world".to_string()
}

const ROW_SIZE: usize = 5;
const COLUMN_SIZE: usize = 5;
const GRID_SIZE: usize = ROW_SIZE * COLUMN_SIZE;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct BoardTile(u8, bool);

struct Board {
    grid: [BoardTile; GRID_SIZE],
}

impl Board {
    fn parse_from_iter<'a, 'b>(
        iter: &'a mut impl Iterator<Item = &'b str>,
    ) -> Result<Self, String> {
        let mut grid = [BoardTile(0, false); GRID_SIZE];
        for y in 0..COLUMN_SIZE {
            let line = iter
                .next()
                .ok_or(format!("Expected {} lines", COLUMN_SIZE))?;
            let numbers = line
                .split_whitespace()
                .map(|it| it.parse())
                .collect::<Result<Box<[u8]>, _>>()
                .map_err(|_| format!("Expected '{}' to be a row of numbers", line))?;
            if numbers.len() != ROW_SIZE {
                return Err(format!(
                    "Expected {} numbers in a row, but got {}",
                    ROW_SIZE,
                    numbers.len()
                ));
            }

            for x in 0..ROW_SIZE {
                grid[y * ROW_SIZE + x] = BoardTile(numbers[x], false);
            }
        }
        Ok(Board { grid })
    }
}

struct Game {
    sequence: Box<[u8]>,
    boards: Box<[Board]>,
}

impl Game {
    fn parse_from_iter<'a, 'b>(
        iter: &'a mut impl Iterator<Item = &'b str>,
    ) -> Result<Self, String> {
        let first_line = iter.next().ok_or("Empty iterator")?;
        let sequence: Box<[u8]> = first_line
            .split(",")
            .map(|it| it.parse())
            .collect::<Result<Box<[u8]>, _>>()
            .map_err(|_| {
                format!(
                    "Expected first line ({}) to be a comma-separated list of numbers",
                    first_line
                )
            })?;

        let mut boards: Vec<Board> = Vec::new();
        let mut iter = iter.peekable();
        // each board is preceded by an empty line
        while let Some(_) = iter.next_if_eq(&"") {
            boards.push(Board::parse_from_iter(&mut iter)?);
        }

        Ok(Game {
            sequence,
            boards: boards.into(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        let result = Game::parse_from_iter(&mut include_str!("day04_example.txt").lines()).unwrap();
        assert_eq!(result.sequence.len(), 27);
        assert_eq!(result.boards.len(), 3);
    }

    // #[test]
    // fn part_one_answer() {
    //     let result = part_one();
    //     assert_eq!(result, "Hello world! (3)");
    // }
}
