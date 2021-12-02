use std::num::ParseIntError;
use std::str::FromStr;

// Day 2: Dive!
use crate::prelude::*;

lazy_static! {
    static ref PUZZLE_INPUT: Box<[Command]> = include_str!("day02_input.txt")
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Direction {
    Forward,
    Down,
    Up,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
struct Command(Direction, i32);

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
struct Position {
    depth: i32,
    horizontal: i32,
}

impl FromStr for Command {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (direction_str, units_str): (&str, &str) = s
            .split_once(" ")
            .ok_or(format!("Expected space separator in '{}'", s))?;

        let direction = match direction_str {
            "forward" => Direction::Forward,
            "down" => Direction::Down,
            "up" => Direction::Up,
            no_match => return Err(format!("Unexpected direction: {}", no_match)),
        };

        let units: i32 = units_str
            .parse()
            .map_err(|err: ParseIntError| err.to_string())?;

        Ok(Command(direction, units))
    }
}

impl Position {
    fn follow_command(&self, &Command(direction, units): &Command) -> Position {
        match direction {
            Direction::Forward => Position {
                horizontal: self.horizontal + units,
                ..*self
            },
            Direction::Down => Position {
                depth: self.depth + units,
                ..*self
            },
            Direction::Up => Position {
                depth: self.depth - units,
                ..*self
            },
        }
    }

    fn output(&self) -> i32 {
        self.horizontal * self.depth
    }
}

pub fn part_one() -> i32 {
    follow_commands(PUZZLE_INPUT.iter()).output()
}

fn follow_commands<'a>(commands: impl Iterator<Item = &'a Command>) -> Position {
    commands.fold(
        Position {
            depth: 0,
            horizontal: 0,
        },
        |prev, command| prev.follow_command(command),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    lazy_static! {
        static ref EXAMPLE_INPUT: Box<[Command]> = [
            "forward 5",
            "down 5",
            "forward 8",
            "up 3",
            "down 8",
            "forward 2",
        ]
        .into_iter()
        .map(|it| it.parse().unwrap())
        .collect();
    }

    #[test]
    fn test_parse() {
        assert_eq!(
            Command::from_str("forward 5"),
            Ok(Command(Direction::Forward, 5))
        );
        assert_eq!(Command::from_str("down 8"), Ok(Command(Direction::Down, 8)));
        assert_eq!(Command::from_str("up 3"), Ok(Command(Direction::Up, 3)));
    }

    #[test]
    fn part_one_example() {
        let result = follow_commands(EXAMPLE_INPUT.iter());
        assert_eq!(
            result,
            Position {
                horizontal: 15,
                depth: 10
            }
        );
        assert_eq!(result.output(), 150);
    }

    #[test]
    fn part_one_answer() {
        let result = part_one();
        assert_eq!(result, 2039912);
    }
}
