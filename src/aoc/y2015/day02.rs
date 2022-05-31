use std::error::Error;
use std::fmt::Display;
use crate::aoc::DaySolution;

pub struct Solution;

impl Solution {
    fn parse_input(&self, input: &str) -> Result<Vec<(u32, u32, u32)>, Box<dyn Error>> {
        let mut data = Vec::new();

        for (i, line) in input.lines().enumerate() {
            let present = line
                .split("x")
                .map(|n| n.parse::<u32>())
                .collect::<Result<Vec<_>, _>>()?;

            if present.len() != 3 {
                Err(format!("invalid present on line {}", i))?;
            }

            data.push((present[0], present[1], present[2]));
        }

        Ok(data)
    }
}

impl DaySolution for Solution {
    // TODO: refactor duplicated code
    fn solve_part1(&self, input: &str) -> Result<Box<dyn Display>, Box<dyn Error>> {
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

    fn solve_part2(&self, input: &str) -> Result<Box<dyn Display>, Box<dyn Error>> {
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
