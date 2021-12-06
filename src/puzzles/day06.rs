// Day 6: Lanternfish
use crate::prelude::*;

const CYCLE_LENGTH: u8 = 7;
const MATURITY: u8 = 2;
const TOTAL_LENGTH: usize = (CYCLE_LENGTH + MATURITY) as usize;

lazy_static! {
    static ref PUZZLE_INPUT: Box<[u8]> = include_str!("day06_input.txt")
        .split(",")
        .map(|it| it.trim().parse().unwrap())
        .collect();
}

pub fn part_one() -> Result<usize, String> {
    let mut all_fish = FishSimulation::from_fish_list(&PUZZLE_INPUT)?;
    Ok(all_fish.count_fish_after_days(80))
}

pub fn part_two() -> Result<usize, String> {
    let mut all_fish = FishSimulation::from_fish_list(&PUZZLE_INPUT)?;
    Ok(all_fish.count_fish_after_days(256))
}

struct FishSimulation {
    fish_by_days_until_spawn: [usize; TOTAL_LENGTH],
}

impl FishSimulation {
    fn from_fish_list(list: &[u8]) -> Result<Self, String> {
        let mut fish_by_days_until_spawn = [0; TOTAL_LENGTH];
        for &fish_state in list {
            if fish_state >= TOTAL_LENGTH as u8 {
                return Err(format!(
                    "Maximum value is {}, but received a {}",
                    TOTAL_LENGTH, fish_state
                ));
            }
            fish_by_days_until_spawn[fish_state as usize] += 1;
        }
        Ok(FishSimulation {
            fish_by_days_until_spawn,
        })
    }

    fn tick(&mut self) {
        let fish_to_spawn = self.fish_by_days_until_spawn[0];
        for i in 1..(TOTAL_LENGTH) {
            self.fish_by_days_until_spawn[i - 1] = self.fish_by_days_until_spawn[i];
        }
        // reset cycle
        self.fish_by_days_until_spawn[CYCLE_LENGTH as usize - 1] += fish_to_spawn;
        // add new fish
        self.fish_by_days_until_spawn[TOTAL_LENGTH - 1] = fish_to_spawn;
    }

    fn count_fish(&self) -> usize {
        self.fish_by_days_until_spawn.iter().sum()
    }

    fn count_fish_after_days(&mut self, days: usize) -> usize {
        for _ in 0..days {
            self.tick();
        }
        self.count_fish()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    lazy_static! {
        static ref EXAMPLE_INPUT: Box<[u8]> = Box::new([3, 4, 3, 1, 2]);
    }

    #[test]
    fn part_one_example() {
        let mut all_fish = FishSimulation::from_fish_list(&EXAMPLE_INPUT).unwrap();
        assert_eq!(all_fish.count_fish_after_days(18), 26);
        assert_eq!(all_fish.count_fish_after_days(80 - 18), 5934);
    }

    #[test]
    fn part_one_answer() {
        let result = part_one();
        assert_eq!(result, Ok(351092));
    }

    #[test]
    fn part_two_example() {
        let mut all_fish = FishSimulation::from_fish_list(&EXAMPLE_INPUT).unwrap();
        assert_eq!(all_fish.count_fish_after_days(256), 26984457539);
    }

    #[test]
    fn part_two_answer() {
        let result = part_two();
        assert_eq!(result, Ok(1595330616005));
    }
}
