// Day 4: Giant Squid
use crate::prelude::*;

fn parse_puzzle_input() -> Game {
    Game::parse_from_iter(&mut include_lines!("day04_input.txt")).unwrap()
}

pub fn part_one() -> Option<u32> {
    parse_puzzle_input().get_winning_score()
}

pub fn part_two() -> Option<u32> {
    parse_puzzle_input().get_last_winning_score()
}

const ROW_SIZE: usize = 5;
const COLUMN_SIZE: usize = 5;
const GRID_SIZE: usize = ROW_SIZE * COLUMN_SIZE;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct BoardTile(u8, bool);

#[derive(Copy, Clone)]
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

    fn tile_at(&self, x: usize, y: usize) -> &BoardTile {
        &self.grid[y * ROW_SIZE + x]
    }

    fn mark(&mut self, next_num: u8) {
        for tile in self.grid.iter_mut() {
            if tile.0 == next_num {
                tile.1 = true
            }
        }
    }

    fn rows(&self) -> impl Iterator<Item = impl Iterator<Item = &BoardTile>> {
        (0..COLUMN_SIZE).map(move |y| (0..ROW_SIZE).map(move |x| self.tile_at(x, y)))
    }

    fn columns(&self) -> impl Iterator<Item = impl Iterator<Item = &BoardTile>> {
        (0..ROW_SIZE).map(move |x| (0..COLUMN_SIZE).map(move |y| self.tile_at(x, y)))
    }

    fn is_winning(&self) -> bool {
        let mut lines = self
            .rows()
            .map(|row| row.collect::<Box<[&BoardTile]>>())
            .chain(self.columns().map(|column| column.collect()));

        lines.any(|line| line.into_iter().all(|BoardTile(_, marked)| *marked))
    }

    fn mark_and_get_score_if_winning(&mut self, next_num: u8) -> Option<u32> {
        self.mark(next_num);
        if self.is_winning() {
            let unmarked: u32 = self
                .grid
                .iter()
                .filter(|tile| !tile.1)
                .map(|tile| tile.0 as u32)
                .sum();
            Some(unmarked * next_num as u32)
        } else {
            None
        }
    }
}

#[derive(Clone)]
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

    fn get_winning_scores(self) -> WinningScoreIterator {
        WinningScoreIterator {
            sequence: self.sequence,
            sequence_index: 0,
            remaining_boards: self.boards,
        }
    }

    fn get_winning_score(self) -> Option<u32> {
        self.get_winning_scores().next()
    }

    fn get_last_winning_score(self) -> Option<u32> {
        self.get_winning_scores().last()
    }
}

struct WinningScoreIterator {
    sequence: Box<[u8]>,
    sequence_index: usize,
    remaining_boards: Box<[Board]>,
}

impl Iterator for WinningScoreIterator {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        while self.sequence_index < self.sequence.len() {
            let next_num = self.sequence[self.sequence_index];
            self.sequence_index += 1;

            enum BoardResult {
                Winning(u32),
                Continue(Board),
            }

            let board_results: Box<[_]> = self
                .remaining_boards
                .into_iter()
                .map(
                    |&(mut board)| match board.mark_and_get_score_if_winning(next_num) {
                        Some(score) => BoardResult::Winning(score),
                        None => BoardResult::Continue(board),
                    },
                )
                .collect();

            self.remaining_boards = board_results
                .into_iter()
                .filter_map(|it| match it {
                    &BoardResult::Continue(board) => Some(board),
                    _ => None,
                })
                .collect();

            // assumption: there will only be one winner per round
            let winner = board_results
                .into_iter()
                .find_map(|it| match it {
                    BoardResult::Winning(score) => Some(score),
                    _ => None,
                })
                .copied();

            if let Some(score) = winner {
                return Some(score);
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    lazy_static! {
        static ref EXAMPLE_INPUT: Game =
            Game::parse_from_iter(&mut include_lines!("day04_example.txt")).unwrap();
    }

    #[test]
    fn parse() {
        let result = Game::parse_from_iter(&mut include_lines!("day04_example.txt")).unwrap();
        assert_eq!(result.sequence.len(), 27);
        assert_eq!(result.boards.len(), 3);
    }

    #[test]
    fn board_does_not_win() {
        let mut board = Board::parse_from_iter(
            &mut [
                "22 13 17 11  0",
                " 8  2 23  4 24",
                "21  9 14 16  7",
                " 6 10  3 18  5",
                " 1 12 20 15 19",
            ]
            .into_iter(),
        )
        .unwrap();

        for num in [7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24] {
            board.mark(num);
        }

        assert_eq!(board.is_winning(), false);
    }

    #[test]
    fn board_wins() {
        let mut board = Board::parse_from_iter(
            &mut [
                "14 21 17 24  4",
                "10 16 15  9 19",
                "18  8 23 26 20",
                "22 11 13  6  5",
                " 2  0 12  3  7",
            ]
            .into_iter(),
        )
        .unwrap();

        for num in [7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24] {
            board.mark(num);
        }

        assert_eq!(board.is_winning(), true);
    }

    #[test]
    fn winning_score() {
        let game = EXAMPLE_INPUT.clone();
        let result = game.get_winning_score();
        assert_eq!(result, Some(4512));
    }

    #[test]
    fn part_one_answer() {
        let result = part_one();
        assert_eq!(result, Some(5685));
    }

    #[test]
    fn last_winner() {
        let game = EXAMPLE_INPUT.clone();
        let result = game.get_last_winning_score();
        assert_eq!(result, Some(1924));
    }

    #[test]
    fn part_two_answer() {
        let result = part_two();
        assert_eq!(result, Some(21070));
    }
}
