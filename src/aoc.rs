pub mod y2015;

use std::fmt;
use std::error::Error as StdError;

pub enum Error {
    InvalidInput,
    ResultNotFound,
    GenericError{
        message: String,
        source: Box<dyn StdError>,
    },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::InvalidInput =>
                write!(f, "invalid input"),
            Error::ResultNotFound =>
                write!(f, "result not found"),
            Error::GenericError{message, ..} =>
                write!(f, "an unexpected error ocurred: {}", message),
        }
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::InvalidInput =>
                write!(f, "invalid input"),
            Error::ResultNotFound =>
                write!(f, "result not found"),
            Error::GenericError{message, ..} =>
                write!(f, "an unexpected error ocurred: {}", message),
        }
    }
}

impl StdError for Error {}

pub type Result<T> = std::result::Result<T, Error>;

pub trait DaySolution<T> {
    fn solve_part1(&self, input: &str) -> Result<T>;

    fn solve_part2(&self, input: &str) -> Result<T>;
}

#[cfg(test)]
mod test {
    use super::*;

    pub enum Part {
        One,
        Two,
    }

    pub fn test_case<T: fmt::Display>(part: Part, solution: impl DaySolution<T>, input: &str, expected: T) {
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
