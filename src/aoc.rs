pub mod y2015;

use std::fmt::Display;
use std::error::Error;

pub trait DaySolution {
    fn solve_part1(&self, input: &str) -> Result<Box<dyn Display>, Box<dyn Error>>;

    fn solve_part2(&self, input: &str) -> Result<Box<dyn Display>, Box<dyn Error>>;
}

#[cfg(test)]
mod test {
    use super::*;

    pub enum Part {
        One,
        Two,
    }

    pub fn test_case<T: Display>(part: Part, solution: impl DaySolution, input: &str, expected: T) {
        let result = match part {
            Part::One => solution.solve_part1(input),
            Part::Two => solution.solve_part2(input),
        };

        let got =
            result
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
