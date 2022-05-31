use std::fmt::Display;
use crate::aoc::{DaySolution, Error};

enum Step {
    Up,
    Down,
}

pub struct Solution;

impl Solution {
    fn parse_input(&self, input: &str) -> Result<Vec<Step>, Error> {
        let mut data = Vec::new();
        for c in input.trim().chars() {
            data.push(match c {
                '(' => Step::Up,
                ')' => Step::Down,
                _ => return Err(Error::InvalidInput),
            });
        }

        Ok(data)
    }
}

impl DaySolution for Solution {
    fn solve_part1(&self, input: &str) -> Result<Box<dyn Display>, Error> {
        let data = self.parse_input(input)?;
        Ok(Box::new(data
             .iter()
             .map(|s| match s {
                 Step::Up => 1,
                 Step::Down => -1,
             })
             .sum::<i32>()))
    }

    fn solve_part2(&self, input: &str) -> Result<Box<dyn Display>, Error> {
        let data = self.parse_input(input)?;

        let mut floor = 0;
        for (i, c) in data.iter().enumerate() {
            floor += match c {
                Step::Up => 1,
                Step::Down => -1,
            };

            if floor == -1 {
                return Ok(Box::new(i+1));
            }
        }

        Err(Error::ResultNotFound)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::aoc::test::{test_case, Part};

    #[test]
    fn solve_part1_case_1() {
        test_case(Part::One, Solution, "(())", 0);
    }

    #[test]
    fn solve_part1_case_2() {
        test_case(Part::One, Solution, "()()", 0);
    }

    #[test]
    fn solve_part1_case_3() {
        test_case(Part::One, Solution, "(((", 3);
    }

    #[test]
    fn solve_part1_case_4() {
        test_case(Part::One, Solution, "(()(()(", 3);
    }

    #[test]
    fn solve_part1_case_5() {
        test_case(Part::One, Solution, "))(((((", 3);
    }

    #[test]
    fn solve_part1_case_6() {
        test_case(Part::One, Solution, "())", -1);
    }

    #[test]
    fn solve_part1_case_7() {
        test_case(Part::One, Solution, "))(", -1);
    }

    #[test]
    fn solve_part1_case_8() {
        test_case(Part::One, Solution, ")))", -3);
    }

    #[test]
    fn solve_part1_case_9() {
        test_case(Part::One, Solution, ")())())", -3);
    }

    #[test]
    fn solve_part2_case_1() {
        test_case(Part::Two, Solution, ")", 1);
    }

    #[test]
    fn solve_part2_case_2() {
        test_case(Part::Two, Solution, "()())", 5);
    }
}
