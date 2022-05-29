use std::error::Error;
use std::fmt::Display;

enum Step {
    Up,
    Down,
}

pub struct Solution {
    data: Vec<Step>
}

impl Solution {
    pub fn new() -> Solution {
        Solution {
            data: vec![],
        }
    }
}

impl super::super::Solution for Solution {
    fn parse_input(&mut self, input: &str) -> Result<(), Box<dyn Error>> {
        self.data = Vec::new();
        for c in input.trim().chars() {
            self.data.push(match c {
                '(' => Step::Up,
                ')' => Step::Down,
                c => Err(format!("unexpected character: {}", c))?,
            });
        }

        Ok(())
    }

    fn solve_part1(&self) -> Option<Box<dyn Display>> {
        Some(Box::new(self.data
             .iter()
             .map(|s| match s {
                 Step::Up => 1,
                 Step::Down => -1,
             })
             .sum::<i32>()))
    }

    fn solve_part2(&self) -> Option<Box<dyn Display>> {
        let mut floor = 0;
        for (i, c) in self.data.iter().enumerate() {
            floor += match c {
                Step::Up => 1,
                Step::Down => -1,
            };

            if floor == -1 {
                return Some(Box::new(i+1));
            }
        }

        None
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::aoc::{Solution as AocSolution, Part};

    #[test]
    fn solve_part1_case_1() {
        test_case(Part::One, "(())", 0);
    }

    #[test]
    fn solve_part1_case_2() {
        test_case(Part::One, "()()", 0);
    }

    #[test]
    fn solve_part1_case_3() {
        test_case(Part::One, "(((", 3);
    }

    #[test]
    fn solve_part1_case_4() {
        test_case(Part::One, "(()(()(", 3);
    }

    #[test]
    fn solve_part1_case_5() {
        test_case(Part::One, "))(((((", 3);
    }

    #[test]
    fn solve_part1_case_6() {
        test_case(Part::One, "())", -1);
    }

    #[test]
    fn solve_part1_case_7() {
        test_case(Part::One, "))(", -1);
    }

    #[test]
    fn solve_part1_case_8() {
        test_case(Part::One, ")))", -3);
    }

    #[test]
    fn solve_part1_case_9() {
        test_case(Part::One, ")())())", -3);
    }

    #[test]
    fn solve_part2_case_1() {
        test_case(Part::Two, ")", 1);
    }

    #[test]
    fn solve_part2_case_2() {
        test_case(Part::Two, "()())", 5);
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
