use std::fmt::Display;
use crate::aoc::{DaySolution, Error, Result};
use std::collections::HashSet;

enum Move {
    Up,
    Right,
    Down,
    Left,
}

impl TryFrom<char> for Move {
    type Error = Error;

    fn try_from(value: char) -> Result<Move> {
        match value {
            '^' => Ok(Move::Up),
            '>' => Ok(Move::Right),
            'v' => Ok(Move::Down),
            '<' => Ok(Move::Left),
            _ => Err(Error::InvalidInput),
        }
    }
}

pub struct Solution;

impl Solution {
    fn parse_input(&self, input: &str) -> Result<Vec<Move>> {
        let data =
            input
            .trim()
            .chars()
            .map(|c| Move::try_from(c))
            .collect::<Result<Vec<Move>>>()?;

        Ok(data)
    }
}

impl DaySolution for Solution {
    fn solve_part1(&self, input: &str) -> Result<Box<dyn Display>> {
        let mut position = (0, 0);

        let mut seen = HashSet::new();
        seen.insert(position);

        for m in self.parse_input(input)?.iter() {
            position = match m {
                Move::Up => (position.0, position.1+1),
                Move::Right => (position.0+1, position.1),
                Move::Down => (position.0, position.1-1),
                Move::Left => (position.0-1, position.1),
            };

            seen.insert(position);
        }

        Ok(Box::new(seen.len().to_string()))
    }

    fn solve_part2(&self, input: &str) -> Result<Box<dyn Display>> {
        let mut santa_pos = (0, 0);
        let mut robot_pos = (0, 0);

        let mut seen = HashSet::new();
        seen.insert(santa_pos);

        for (i, m) in self.parse_input(input)?.iter().enumerate() {
            let current_pos =
                if i % 2 == 0 {
                    &mut santa_pos
                } else {
                    &mut robot_pos
                };

            *current_pos = match m {
                Move::Up => (current_pos.0, current_pos.1+1),
                Move::Right => (current_pos.0+1, current_pos.1),
                Move::Down => (current_pos.0, current_pos.1-1),
                Move::Left => (current_pos.0-1, current_pos.1),
            };

            seen.insert(*current_pos);
        }

        Ok(Box::new(seen.len().to_string()))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::aoc::test::{test_case, Part};

    #[test]
    fn solve_part1_case_1() {
        test_case(Part::One, Solution, ">", 2);
    }

    #[test]
    fn solve_part1_case_2() {
        test_case(Part::One, Solution, "^>v<", 4);
    }

    #[test]
    fn solve_part1_case_3() {
        test_case(Part::One, Solution, "^v^v^v^v^v", 2);
    }

    #[test]
    fn solve_part2_case_1() {
        test_case(Part::Two, Solution, "^v", 3);
    }

    #[test]
    fn solve_part2_case_2() {
        test_case(Part::Two, Solution, "^>v<", 3);
    }

    #[test]
    fn solve_part2_case_3() {
        test_case(Part::Two, Solution, "^v^v^v^v^v", 11);
    }
}
