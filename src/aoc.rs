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

pub trait DaySolution {
    type Data;

    fn parse_input(&self, input: &str) -> Result<Self::Data, Box<dyn Error>>;

    fn solve_part1(&self, data: Self::Data) -> Option<Box<dyn Display>>;

    fn solve_part2(&self, data: Self::Data) -> Option<Box<dyn Display>>;

    fn solve(&self, part: Part, input: &str) -> Result<Option<Box<dyn Display>>, Box<dyn Error>> {
        let data = self.parse_input(input)?;

        match part {
            Part::One => Ok(self.solve_part1(data)),
            Part::Two => Ok(self.solve_part2(data)),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    pub fn test_case<T: Display>(part: Part, solver: impl DaySolution, input: &str, expected: T) {
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
