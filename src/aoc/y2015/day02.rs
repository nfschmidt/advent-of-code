use std::error::Error;
use std::fmt::Display;
use crate::aoc::DaySolution;

pub struct Solution;

impl Solution {
    pub fn new() -> Solution {
        Solution
    }
}

impl DaySolution for Solution {
    type Data = Vec<(u32, u32, u32)>;

    fn parse_input(&self, input: &str) -> Result<Self::Data, Box<dyn Error>> {
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

    fn solve_part1(&self, data: Self::Data) -> Option<Box<dyn Display>> {
        let result =
            data
            .iter()
            .map(|dims| {
                let sides = [dims.0*dims.1, dims.0*dims.2, dims.1*dims.2];
                return 2*sides.iter().sum::<u32>() + sides.iter().min().unwrap();
            })
            .sum::<u32>();

        Some(Box::new(result))
    }

    fn solve_part2(&self, data: Self::Data) -> Option<Box<dyn Display>> {
        let result =
            data
            .iter()
            .map(|dims| {
                let mut ds = vec![dims.0, dims.1, dims.2];
                ds.sort();
                return 2*(ds[0]+ds[1]) + ds[0]*ds[1]*ds[2];
            })
            .sum::<u32>();

        Some(Box::new(result))
    }
}
