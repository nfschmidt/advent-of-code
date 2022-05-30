use std::error::Error;
use std::fmt::Display;
use crate::aoc::DaySolution;

pub enum Step {
    Up,
    Down,
}

pub struct Solution;

impl Solution {
    pub fn new() -> Solution {
        Solution
    }
}

impl DaySolution for Solution {
    type Data = Vec<Step>;

    fn parse_input(&self, input: &str) -> Result<Self::Data, Box<dyn Error>> {
        let mut data = Vec::new();
        for c in input.trim().chars() {
            data.push(match c {
                '(' => Step::Up,
                ')' => Step::Down,
                c => Err(format!("unexpected character: {}", c))?,
            });
        }

        Ok(data)
    }

    fn solve_part1(&self, data: Self::Data) -> Option<Box<dyn Display>> {
        Some(Box::new( data
             .iter()
             .map(|s| match s {
                 Step::Up => 1,
                 Step::Down => -1,
             })
             .sum::<i32>()))
    }

    fn solve_part2(&self, data: Self::Data) -> Option<Box<dyn Display>> {
        let mut floor = 0;
        for (i, c) in data.iter().enumerate() {
            floor += match c {
                Step::Up => 1,
                Step::Down => -1,
            };

            if floor == -1 {
                return Some(Box::new(i+1))
            }
        }

        None
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::aoc::{Part, test};

    #[test]
    fn solve_part1_case_1() {
        test::test_case(Part::One, Solution::new(), "(())", 0);
    }

    #[test]
    fn solve_part1_case_2() {
        test::test_case(Part::One, Solution::new(), "()()", 0);
    }

    #[test]
    fn solve_part1_case_3() {
        test::test_case(Part::One, Solution::new(), "(((", 3);
    }

    #[test]
    fn solve_part1_case_4() {
        test::test_case(Part::One, Solution::new(), "(()(()(", 3);
    }

    #[test]
    fn solve_part1_case_5() {
        test::test_case(Part::One, Solution::new(), "))(((((", 3);
    }

    #[test]
    fn solve_part1_case_6() {
        test::test_case(Part::One, Solution::new(), "())", -1);
    }

    #[test]
    fn solve_part1_case_7() {
        test::test_case(Part::One, Solution::new(), "))(", -1);
    }

    #[test]
    fn solve_part1_case_8() {
        test::test_case(Part::One, Solution::new(), ")))", -3);
    }

    #[test]
    fn solve_part1_case_9() {
        test::test_case(Part::One, Solution::new(), ")())())", -3);
    }

    #[test]
    fn solve_part2_case_1() {
        test::test_case(Part::Two, Solution::new(), ")", 1);
    }

    #[test]
    fn solve_part2_case_2() {
        test::test_case(Part::Two, Solution::new(), "()())", 5);
    }
}
