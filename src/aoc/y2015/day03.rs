use std::fmt::Display;
use std::error::Error;
use crate::aoc::DaySolution;
use std::collections::HashSet;

pub enum Move {
    Up,
    Right,
    Down,
    Left,
}

impl TryFrom<char> for Move {
    type Error = String;

    fn try_from(value: char) -> Result<Move, Self::Error> {
        match value {
            '^' => Ok(Move::Up),
            '>' => Ok(Move::Right),
            'v' => Ok(Move::Down),
            '<' => Ok(Move::Left),
            _ => Err(format!("Invalid move '{}'", value)),
        }
    }
}

pub struct Solution;

impl DaySolution for Solution {
    type Data = Vec<Move>;

    fn parse_input(&self, input: &str) -> Result<Self::Data, Box<dyn Error>> {
        let data =
            input
            .trim()
            .chars()
            .map(|c| Move::try_from(c))
            .collect::<Result<Vec<Move>, _>>()?;

        Ok(data)
    }

    fn solve_part1(&self, data: Self::Data) -> Option<Box<dyn Display>> {
        let mut position = (0, 0);

        let mut seen = HashSet::new();
        seen.insert(position);

        for m in data.iter() {
            position = match m {
                Move::Up => (position.0, position.1+1),
                Move::Right => (position.0+1, position.1),
                Move::Down => (position.0, position.1-1),
                Move::Left => (position.0-1, position.1),
            };

            seen.insert(position);
        }

        Some(Box::new(seen.len().to_string()))
    }

    fn solve_part2(&self, data: Self::Data) -> Option<Box<dyn Display>> {
        let mut santa_pos = (0, 0);
        let mut robot_pos = (0, 0);

        let mut seen = HashSet::new();
        seen.insert(santa_pos);

        for (i, m) in data.iter().enumerate() {
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

        Some(Box::new(seen.len().to_string()))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::aoc::{Part, test};

    #[test]
    fn solve_part1_case_1() {
        test::test_case(Part::One, Solution, ">", 2);
    }

    #[test]
    fn solve_part1_case_2() {
        test::test_case(Part::One, Solution, "^>v<", 4);
    }

    #[test]
    fn solve_part1_case_3() {
        test::test_case(Part::One, Solution, "^v^v^v^v^v", 2);
    }

    #[test]
    fn solve_part2_case_1() {
        test::test_case(Part::Two, Solution, "^v", 3);
    }

    #[test]
    fn solve_part2_case_2() {
        test::test_case(Part::Two, Solution, "^>v<", 3);
    }

    #[test]
    fn solve_part2_case_3() {
        test::test_case(Part::Two, Solution, "^v^v^v^v^v", 11);
    }
}
