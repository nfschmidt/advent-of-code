use std::fmt::Display;
use std::error::Error;
use crate::aoc::Solution as AocSolution;
use md5::Digest;

pub struct Solution {
    data: String,
}

impl Solution {
    pub fn new() -> Solution {
        Solution {
            data: String::new(),
        }
    }

    fn solve_leading_zeros(&self, zeros_count: usize) -> Option<u32> {
        for i in 0u32..std::u32::MAX {
            let data = format!("{}{}", self.data, i);
            let hash = md5::Md5::digest(data.as_bytes());

            if format!("{:x}", hash).starts_with(&"0".repeat(zeros_count)) {
                return Some(i)
            }
        }

        None
    }
}

impl AocSolution for Solution {
    fn parse_input(&mut self, input: &str) -> Result<(), Box<dyn Error>> {
        self.data = input.trim().to_string();
        Ok(())
    }

    fn solve_part1(&self) -> Option<Box<dyn Display>> {
        Some(Box::new(self.solve_leading_zeros(5)?))
    }

    fn solve_part2(&self) -> Option<Box<dyn Display>> {
        Some(Box::new(self.solve_leading_zeros(6)?))
    }
}

#[cfg(test)]
mod test {
    /* these tests are ignored by default because they have a long running time */

    use super::*;
    use crate::aoc::Part;

    #[test]
    #[ignore]
    fn solve_part1_case_1() {
        test_case(Part::One, "abcdef", 609043);
    }

    #[test]
    #[ignore]
    fn solve_part1_case_2() {
        test_case(Part::One, "pqrstuv", 1048970);
    }

    #[test]
    #[ignore]
    fn solve_part2_case_1() {
        test_case(Part::Two, "bgvyzdsv", 1038736);
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
