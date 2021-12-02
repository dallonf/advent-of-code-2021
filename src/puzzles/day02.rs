// Day 2: Dive!
use crate::prelude::*;
use std::num::ParseIntError;
use std::str::FromStr;

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
struct PositionMk1 {
    depth: i32,
    horizontal: i32,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
struct PositionMk2 {
    depth: i32,
    horizontal: i32,
    aim: i32,
}

trait Position {
    fn follow_command(&mut self, command: &Command);
    fn with_followed_command(&self, command: &Command) -> Self
    where
        Self: Copy,
    {
        let mut new = *self;
        new.follow_command(command);
        new
    }
    fn output(&self) -> i32;
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

impl Position for PositionMk1 {
    fn follow_command(&mut self, &Command(direction, units): &Command) {
        match direction {
            Direction::Forward => {
                self.horizontal += units;
            }
            Direction::Down => {
                self.depth += units;
            }
            Direction::Up => {
                self.depth -= units;
            }
        }
    }

    fn output(&self) -> i32 {
        self.horizontal * self.depth
    }
}

pub fn part_one() -> i32 {
    follow_commands(PUZZLE_INPUT.iter()).output()
}

fn follow_commands<'a>(commands: impl Iterator<Item = &'a Command>) -> PositionMk1 {
    commands.fold(
        PositionMk1 {
            depth: 0,
            horizontal: 0,
        },
        |prev, command| prev.with_followed_command(command),
    )
}

pub fn part_two() -> i32 {
    follow_commands_mk2(PUZZLE_INPUT.iter()).output()
}

impl Position for PositionMk2 {
    fn follow_command(&mut self, &Command(direction, units): &Command) {
        match direction {
            Direction::Forward => {
                self.horizontal += units;
                self.depth += self.aim * units;
            }
            Direction::Down => {
                self.aim += units;
            }
            Direction::Up => {
                self.aim -= units;
            }
        }
    }

    fn output(&self) -> i32 {
        self.horizontal * self.depth
    }
}

fn follow_commands_mk2<'a>(commands: impl Iterator<Item = &'a Command>) -> PositionMk2 {
    commands.fold(
        PositionMk2 {
            depth: 0,
            horizontal: 0,
            aim: 0,
        },
        |prev, command| prev.with_followed_command(command),
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
            PositionMk1 {
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

    #[test]
    fn part_two_example() {
        let result = follow_commands_mk2(EXAMPLE_INPUT.iter());
        assert_eq!(
            result,
            PositionMk2 {
                horizontal: 15,
                depth: 60,
                aim: 10
            },
        );
        assert_eq!(result.output(), 900);
    }

    #[test]
    fn part_two_answer() {
        let result = part_two();
        assert_eq!(result, 1942068080);
    }
}
