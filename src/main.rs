use std::io;
use std::io::Read;
use std::error::Error;
use std::env;
use std::fmt::Display;
use std::str::FromStr;

enum Part {
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

trait Solution {
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

struct Day01 {
    data: Vec<Step>
}

enum Step {
    Up,
    Down,
}

impl Day01 {
    fn new() -> Day01 {
        Day01 {
            data: vec![],
        }
    }
}

impl Solution for Day01 {
    fn parse_input(&mut self, input: &str) -> Result<(), Box<dyn Error>> {
        self.data = Vec::new();
        for c in input.chars() {
            self.data.push(match c {
                '(' => Step::Up,
                ')' => Step::Down,
                c => Err(format!("unexpected character: {}", c))?,
            });
        }

        Ok(())
    }

    fn solve_part1(&self) -> Option<Box<dyn Display>> {
        Some(Box::new(self.data
             .iter()
             .map(|s| match s {
                 Step::Up => 1,
                 Step::Down => -1,
             })
             .sum::<i32>()))
    }

    fn solve_part2(&self) -> Option<Box<dyn Display>> {
        let mut floor = 0;
        for (i, c) in self.data.iter().enumerate() {
            floor += match c {
                Step::Up => 1,
                Step::Down => -1,
            };

            if floor == -1 {
                return Some(Box::new(i+1));
            }
        }

        None
    }
}

struct Day02 {
    data: Vec<(u32, u32, u32)>,
}

impl Day02 {
    fn new() -> Day02 {
        Day02 {
            data: vec![],
        }
    }
}

impl Solution for Day02 {
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

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = env::args().skip(1);
    let year: u16 = args.next().ok_or("invalid year".to_string())?.parse()?;
    let day: u8 = args.next().ok_or("invalid day".to_string())?.parse()?;
    let part: Part = args.next().ok_or("invalid part".to_string())?.parse()?;

    let mut solver = get_solver(year, day)
        .ok_or("invalid problem".to_string())?;

    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let result = solver.solve(part, &input)?;

    show_result(result);
    Ok(())
}

fn get_solver(year: u16, day: u8) -> Option<Box<dyn Solution>> {
    match (year, day) {
        (2015, 1) => Some(Box::new(Day01::new())),
        (2015, 2) => Some(Box::new(Day02::new())),
        _ => None,
    }
}

fn show_result(result: Option<Box::<dyn Display>>) {
    println!("{}", match result {
        None => "result not found".to_string(),
        Some(r) => r.to_string(),
    })
}
