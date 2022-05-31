use std::fmt::Display;
use std::error::Error;
use crate::aoc::DaySolution;
use md5::Digest;

pub struct Solution;

impl Solution {
    fn solve_leading_zeros(&self, zeros_count: usize, input: &str) -> Option<u32> {
        for i in 0u32..std::u32::MAX {
            let data = format!("{}{}", input, i);
            let hash = md5::Md5::digest(data.as_bytes());

            if format!("{:x}", hash).starts_with(&"0".repeat(zeros_count)) {
                return Some(i)
            }
        }

        None
    }
}

impl DaySolution for Solution {
    fn solve_part1(&self, input: &str) -> Result<Box<dyn Display>, Box<dyn Error>> {
        let data = input.trim();
        Ok(Box::new(self.solve_leading_zeros(5, &data).ok_or("result not found".to_string())?))
    }

    fn solve_part2(&self, input: &str) -> Result<Box<dyn Display>, Box<dyn Error>> {
        let data = input.trim();
        Ok(Box::new(self.solve_leading_zeros(6, &data).ok_or("result not found".to_string())?))
    }
}

#[cfg(test)]
mod test {
    // these tests are ignored by default because they have a long running ti

    use super::*;
    use crate::aoc::test::{test_case, Part};

    #[test]
    #[ignore]
    fn solve_part1_case_1() {
        test_case(Part::One, Solution, "abcdef", 609043);
    }

    #[test]
    #[ignore]
    fn solve_part1_case_2() {
        test_case(Part::One, Solution, "pqrstuv", 1048970);
    }

    #[test]
    #[ignore]
    fn solve_part2_case_1() {
        test_case(Part::Two, Solution, "bgvyzdsv", 1038736);
    }
}
