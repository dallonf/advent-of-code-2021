// Day 6: Lanternfish
use crate::prelude::*;

const CYCLE_LENGTH: u8 = 7;
const MATURITY: u8 = 2;

lazy_static! {
    static ref PUZZLE_INPUT: Box<[u8]> = include_str!("day06_input.txt")
        .split(",")
        .map(|it| it.trim().parse().unwrap())
        .collect();
}

pub fn part_one() -> usize {
    let mut all_fish = PUZZLE_INPUT.clone().into_vec();
    count_fish_after_days(&mut all_fish, 80)
}

fn tick(all_fish: &mut Vec<u8>) {
    let count_at_zero = all_fish.iter().filter(|&&x| x == 0).count();
    for fish in all_fish.iter_mut() {
        *fish = match *fish {
            0 => CYCLE_LENGTH - 1,
            life => life - 1,
        };
    }
    let mut new_fish = vec![CYCLE_LENGTH + MATURITY - 1; count_at_zero];
    all_fish.append(&mut new_fish);
}

fn count_fish_after_days(all_fish: &mut Vec<u8>, days: usize) -> usize {
    for _ in 0..days {
        tick(all_fish);
    }
    all_fish.len()
}

#[cfg(test)]
mod tests {

    use super::*;

    lazy_static! {
        static ref EXAMPLE_INPUT: Box<[u8]> = Box::new([3, 4, 3, 1, 2]);
    }

    #[test]
    fn part_one_example() {
        let mut all_fish: Vec<u8> = EXAMPLE_INPUT.clone().into_vec();
        assert_eq!(count_fish_after_days(&mut all_fish, 18), 26);
        assert_eq!(count_fish_after_days(&mut all_fish, 80 - 18), 5934);
    }

    #[test]
    fn part_one_answer() {
        let result = part_one();
        assert_eq!(result, 351092);
    }
}
