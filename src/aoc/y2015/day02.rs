use std::fmt::Display;
use crate::aoc::{DaySolution, Error};

pub struct Solution;

impl Solution {
    fn parse_input(&self, input: &str) -> Result<Vec<(u32, u32, u32)>, Error> {
        let mut data = Vec::new();

        for line in input.lines() {
            let present = line
                .split("x")
                .map(|n| n.parse::<u32>().map_err(|_| Error::InvalidInput))
                .collect::<Result<Vec<_>, _>>()?;

            if present.len() != 3 {
                return Err(Error::InvalidInput);
            }

            data.push((present[0], present[1], present[2]));
        }

        Ok(data)
    }
}

impl DaySolution for Solution {
    // TODO: refactor duplicated code
    fn solve_part1(&self, input: &str) -> Result<Box<dyn Display>, Error> {
        let result =
            self.parse_input(input)?
            .iter()
            .map(|dims| {
                let sides = [dims.0*dims.1, dims.0*dims.2, dims.1*dims.2];
                return 2*sides.iter().sum::<u32>() + sides.iter().min().unwrap();
            })
            .sum::<u32>();

        Ok(Box::new(result))
    }

    fn solve_part2(&self, input: &str) -> Result<Box<dyn Display>, Error> {
        let result =
            self.parse_input(input)?
            .iter()
            .map(|dims| {
                let mut ds = vec![dims.0, dims.1, dims.2];
                ds.sort();
                return 2*(ds[0]+ds[1]) + ds[0]*ds[1]*ds[2];
            })
            .sum::<u32>();

        Ok(Box::new(result))
    }
}
