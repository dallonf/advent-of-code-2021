// Day 17: Trick Shot
use crate::prelude::*;
use anyhow::{anyhow, Result};
use regex::Regex;
use std::ops::{Add, AddAssign, RangeInclusive};

lazy_static! {
    static ref PUZZLE_INPUT: BoxArea2D =
        parse_target_area(include_str!("day17_input.txt").trim()).unwrap();
    static ref INPUT_REGEX: Regex =
        Regex::new(r"^target area: x=(-?[0-9]+)\.\.(-?[0-9]+), y=(-?[0-9]+)\.\.(-?[0-9]+)$")
            .unwrap();
}

pub fn part_one() -> Int {
    let result = find_highest_trajectory(&PUZZLE_INPUT);
    result.highest_y
}

type Int = i32;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Vec2 {
    x: Int,
    y: Int,
}

impl Vec2 {
    fn new(x: Int, y: Int) -> Self {
        Vec2 { x, y }
    }
}

impl Add for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Self) -> Self::Output {
        Vec2::new(self.x + rhs.x, self.y + rhs.y)
    }
}
impl AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Probe {
    position: Vec2,
    velocity: Vec2,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum LaunchResult {
    Hit { highest_y: Int },
    Missed,
}

impl Probe {
    fn new(velocity: Vec2) -> Self {
        Probe {
            position: Vec2::new(0, 0),
            velocity,
        }
    }

    fn step(&mut self) {
        self.position += self.velocity;
        let x_sign = self.velocity.x.signum();
        self.velocity.x -= x_sign;
        self.velocity.y -= 1;
    }

    fn missed_target(&self, target: &BoxArea2D) -> bool {
        self.velocity.y < 0 && self.position.y < target.bottom_left.y
    }

    fn launch_hits_target(&mut self, target: &BoxArea2D) -> bool {
        return match self.launch(target) {
            LaunchResult::Hit { .. } => true,
            LaunchResult::Missed => false,
        };
    }

    fn launch(&mut self, target: &BoxArea2D) -> LaunchResult {
        let mut highest_y = self.position.y;
        loop {
            if target.contains(self.position) {
                return LaunchResult::Hit { highest_y };
            }

            self.step();

            if self.missed_target(target) {
                return LaunchResult::Missed;
            }

            if self.position.y > highest_y {
                highest_y = self.position.y;
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct BoxArea2D {
    bottom_left: Vec2,
    top_right: Vec2,
}

impl BoxArea2D {
    fn x_range(&self) -> RangeInclusive<Int> {
        RangeInclusive::new(self.bottom_left.x, self.top_right.x)
    }

    fn y_range(&self) -> RangeInclusive<Int> {
        RangeInclusive::new(self.bottom_left.y, self.top_right.y)
    }

    fn contains(&self, point: Vec2) -> bool {
        self.x_range().contains(&point.x) && self.y_range().contains(&point.y)
    }
}

fn parse_target_area(input: &str) -> Result<BoxArea2D> {
    let re_captures = INPUT_REGEX
        .captures(input)
        .ok_or(anyhow!("input '{}' does not match expected format", input))?;

    let read_int = |index: usize| -> Int {
        let read_string = re_captures.get(index).unwrap();
        let int = read_string.as_str().parse().unwrap();
        int
    };

    let ordered_range = |index1: usize, index2: usize| {
        let result = [read_int(index1), read_int(index2)];
        if result[0] < result[1] {
            result
        } else {
            [result[1], result[0]]
        }
    };

    let x_range = ordered_range(1, 2);
    let y_range = ordered_range(3, 4);

    let bottom_left = Vec2::new(x_range[0], y_range[0]);
    let top_right = Vec2::new(x_range[1], y_range[1]);

    Ok(BoxArea2D {
        bottom_left,
        top_right,
    })
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct HighestTrajectoryResult {
    highest_y: Int,
    starting_velocity: Vec2,
}

/// Assumes that there is at least one valid trajectory.
/// Will get caught in an infinite loop if not.
fn find_highest_trajectory(target: &BoxArea2D) -> HighestTrajectoryResult {
    let furthest_x = [target.bottom_left.x, target.top_right.x]
        .into_iter()
        .max_by_key(|x| x.abs())
        .unwrap();
    let possible_x_values = if furthest_x > 0 {
        RangeInclusive::new(0, furthest_x)
    } else {
        RangeInclusive::new(furthest_x, 0)
    };

    let launch_for_y = |y_vel: Int| -> Option<HighestTrajectoryResult> {
        let launches = possible_x_values.clone().filter_map(|x_yel| {
            let velocity = Vec2::new(x_yel, y_vel);
            match Probe::new(velocity).launch(target) {
                LaunchResult::Hit { highest_y } => Some(HighestTrajectoryResult {
                    highest_y,
                    starting_velocity: velocity,
                }),
                LaunchResult::Missed => None,
            }
        });
        match launches.max_by_key(|it| it.highest_y) {
            Some(result) => Some(result),
            None => None,
        }
    };

    let mut y_vel = target.bottom_left.y;
    let mut best_result: Option<HighestTrajectoryResult> = None;

    loop {
        let result = launch_for_y(y_vel);
        match result {
            Some(new_result) => {
                best_result = match best_result {
                    Some(best_result) => {
                        if best_result.highest_y < new_result.highest_y {
                            Some(new_result)
                        } else {
                            Some(best_result)
                        }
                    }
                    None => {
                        if new_result.highest_y > target.top_right.y {
                            Some(new_result)
                        } else {
                            // c'mon, you can do better than that
                            None
                        }
                    }
                };
            }
            None => {
                if let Some(best_result) = best_result {
                    // we've already found a result, and are now coming up dry
                    return best_result;
                }
            }
        }

        y_vel += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    lazy_static! {
        static ref EXAMPLE_INPUT: BoxArea2D =
            parse_target_area("target area: x=20..30, y=-10..-5").unwrap();
    }

    #[test]
    fn test_parse() {
        let result = parse_target_area("target area: x=20..30, y=-10..-5").unwrap();
        assert_eq!(
            result,
            BoxArea2D {
                bottom_left: Vec2::new(20, -10),
                top_right: Vec2::new(30, -5)
            }
        );
    }

    #[test]
    fn test_successful_launch() {
        assert!(Probe::new(Vec2::new(7, 2)).launch_hits_target(&EXAMPLE_INPUT));
        assert!(Probe::new(Vec2::new(6, 3)).launch_hits_target(&EXAMPLE_INPUT));
        assert!(Probe::new(Vec2::new(9, 0)).launch_hits_target(&EXAMPLE_INPUT));
    }

    #[test]
    fn test_missed_launch() {
        assert!(!Probe::new(Vec2::new(17, -4)).launch_hits_target(&EXAMPLE_INPUT));
    }

    #[test]
    fn test_highest_y() {
        assert_eq!(
            Probe::new(Vec2::new(6, 9)).launch(&EXAMPLE_INPUT),
            LaunchResult::Hit { highest_y: 45 }
        );
    }

    #[test]
    fn test_highest_trajectory() {
        assert_eq!(
            find_highest_trajectory(&EXAMPLE_INPUT),
            HighestTrajectoryResult {
                highest_y: 45,
                // I'd have expected a value of 6,9, but 7,9 also works and the requirements
                // aren't specific about x
                starting_velocity: Vec2::new(7, 9)
            }
        );
    }

    #[test]
    fn part_one_answer() {
        let result = part_one();
        assert!(result > 2211);
        assert_eq!(result, 0); // waiting to find actual answer
    }
}
