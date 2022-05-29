use std::fmt::Display;
use std::error::Error;
use crate::aoc::Solution as AocSolution;
use std::str::FromStr;
use std::collections::HashSet;

enum Move {
    Up,
    Right,
    Down,
    Left,
}

impl FromStr for Move {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "^" => Ok(Move::Up),
            ">" => Ok(Move::Right),
            "v" => Ok(Move::Down),
            "<" => Ok(Move::Left),
            _ => Err(format!("Invalid move '{}'", s)),
        }
    }
}

pub struct Solution {
    data: Vec<Move>,
}

impl Solution {
    pub fn new() -> Solution {
        Solution {
            data: vec![],
        }
    }
}

impl AocSolution for Solution {
    fn parse_input(&mut self, input: &str) -> Result<(), Box<dyn Error>> {
        self.data =
            input
            .trim()
            .chars()
            .map(|c| c.to_string().parse())
            .collect::<Result<Vec<Move>, _>>()?;

        Ok(())
    }

    fn solve_part1(&self) -> Option<Box<dyn Display>> {
        let mut position = (0, 0);

        let mut seen = HashSet::new();
        seen.insert(position);

        for m in self.data.iter() {
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

    fn solve_part2(&self) -> Option<Box<dyn Display>> {
        let mut santa_pos = (0, 0);
        let mut robot_pos = (0, 0);

        let mut seen = HashSet::new();
        seen.insert(santa_pos);

        for (i, m) in self.data.iter().enumerate() {
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
    use crate::aoc::Part;
    use crate::aoc::Solution as AocSolution;

    #[test]
    fn solve_part1_case_1() {
        test_case(Part::One, ">", 2);
    }

    #[test]
    fn solve_part1_case_2() {
        test_case(Part::One, "^>v<", 4);
    }

    #[test]
    fn solve_part1_case_3() {
        test_case(Part::One, "^v^v^v^v^v", 2);
    }

    #[test]
    fn solve_part2_case_1() {
        test_case(Part::Two, "^v", 3);
    }

    #[test]
    fn solve_part2_case_2() {
        test_case(Part::Two, "^>v<", 3);
    }

    #[test]
    fn solve_part2_case_3() {
        test_case(Part::Two, "^v^v^v^v^v", 11);
    }

    fn test_case(part: Part, input: &str, expected: i32) {
        let mut solver = Solution::new();
        let got = solver.solve(part, input)
            .unwrap()
            .unwrap()
            .to_string();

        assert_eq!(
            expected.to_string(),
            got,
            "input '{}', got: '{}', expected: '{}'",
            input,
            got,
            expected);
    }
}
