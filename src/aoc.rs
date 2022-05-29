pub mod y2015;

use std::str::FromStr;
use std::fmt::Display;
use std::error::Error;

pub enum Part {
    One,
    Two
}

impl FromStr for Part {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(Part::One),
            "2" => Ok(Part::Two),
            _ => Err(format!("Invalid part '{}'", s)),
        }
    }
}

pub trait Solution {
    fn parse_input(&mut self, input: &str) -> Result<(), Box<dyn Error>>;

    fn solve_part1(&self) -> Option<Box<dyn Display>>;

    fn solve_part2(&self) -> Option<Box<dyn Display>>;

    fn solve(&mut self, part: Part, input: &str) -> Result<Option<Box<dyn Display>>, Box<dyn Error>> {
        self.parse_input(input)?;

        match part {
            Part::One => Ok(self.solve_part1()),
            Part::Two => Ok(self.solve_part2()),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    pub fn test_case<T: Display>(part: Part, solver: &mut impl Solution, input: &str, expected: T) {
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
