use std::env;
use std::error::Error;
use std::fmt::Display;
use std::io;
use std::io::Read;
use std::str::FromStr;
use advent_of_code_rust::aoc::{y2015, DaySolution};

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = env::args().skip(1);
    let year: u16 = args.next().ok_or("invalid year".to_string())?.parse()?;
    let day: u8 = args.next().ok_or("invalid day".to_string())?.parse()?;
    let part: Part = args.next().ok_or("invalid part".to_string())?.parse()?;

    let solution = get_solution(year, day)
        .ok_or("invalid problem".to_string())?;

    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let result = match part{
        Part::One => solution.solve_part1(&input),
        Part::Two => solution.solve_part2(&input),
    }?;

    show_result(&result);
    Ok(())
}

fn get_solution(year: u16, day: u8) -> Option<Box<dyn DaySolution>> {
    match (year, day) {
        (2015, 1) => Some(Box::new(y2015::day01::Solution)),
        (2015, 2) => Some(Box::new(y2015::day02::Solution)),
        (2015, 3) => Some(Box::new(y2015::day03::Solution)),
        (2015, 4) => Some(Box::new(y2015::day04::Solution)),
        (2015, 5) => Some(Box::new(y2015::day05::Solution)),
        _ => None,
    }
}

fn show_result(result: &dyn Display) {
    println!("{}", result.to_string());
}

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

