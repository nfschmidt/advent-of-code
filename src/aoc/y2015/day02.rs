use std::error::Error;
use std::fmt::Display;

pub struct Solution {
    data: Vec<(u32, u32, u32)>,
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

        for (i, line) in input.lines().enumerate() {
            let present = line
                .split("x")
                .map(|n| n.parse::<u32>())
                .collect::<Result<Vec<_>, _>>()?;

            if present.len() != 3 {
                Err(format!("invalid present on line {}", i))?;
            }

            self.data.push((present[0], present[1], present[2]));
        }

        Ok(())
    }

    fn solve_part1(&self) -> Option<Box<dyn Display>> {
        let result = self.data
            .iter()
            .map(|dims| {
                let sides = [dims.0*dims.1, dims.0*dims.2, dims.1*dims.2];
                return 2*sides.iter().sum::<u32>() + sides.iter().min().unwrap();
            })
            .sum::<u32>();

        Some(Box::new(result))
    }

    fn solve_part2(&self) -> Option<Box<dyn Display>> {
        let result = self.data
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
