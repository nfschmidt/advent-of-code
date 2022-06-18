use std::env;
use std::error::Error;
use std::fmt::Display;
use std::io;
use std::io::Read;
use std::str::FromStr;
use advent_of_code_rust::aoc;
use advent_of_code_rust::aoc::{y2015, DaySolution};

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = env::args().skip(1);
    let year: u16 = args.next().ok_or("invalid year".to_string())?.parse()?;
    let day: u8 = args.next().ok_or("invalid day".to_string())?.parse()?;
    let part: Part = args.next().ok_or("invalid part".to_string())?.parse()?;

    let solution = get_solution(year, day, part)
        .ok_or("invalid problem".to_string())?;

    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let result = solution(&input)?;

    show_result(&result);
    Ok(())
}

fn get_solution(year: u16, day: u8, part: Part) -> Option<Box<dyn Fn(&str) -> Result<Box::<dyn Display>, aoc::Error>>> {
    match (year, day) {
        (2015, 1) => Some(transform(&y2015::day01::Solution, part)),
        (2015, 2) => Some(transform(&y2015::day02::Solution, part)),
        (2015, 3) => Some(transform(&y2015::day03::Solution, part)),
        (2015, 4) => Some(transform(&y2015::day04::Solution, part)),
        (2015, 5) => Some(transform(&y2015::day05::Solution, part)),
        (2015, 6) => Some(transform(&y2015::day06::Solution, part)),
        (2015, 7) => Some(transform(&y2015::day07::Solution, part)),
        (2015, 8) => Some(transform(&y2015::day08::Solution, part)),
        (2015, 9) => Some(transform(&y2015::day09::Solution, part)),
        (2015, 10) => Some(transform(&y2015::day10::Solution, part)),
        (2015, 11) => Some(transform(&y2015::day11::Solution, part)),
        (2015, 12) => Some(transform(&y2015::day12::Solution, part)),
        _ => None,
    }
}

fn transform<T>(day_solution: &dyn DaySolution<T>, part: Part) -> Box<dyn Fn(&str) -> Result<Box::<dyn Display>, aoc::Error> + '_>
    where T: Display + 'static {

    match part {
        Part::One => Box::new(|s| {
            match day_solution.solve_part1(s) {
                Ok(r) => Ok(Box::new(r)),
                Err(e) => Err(e),
            }
        }),
        Part::Two => Box::new(|s| {
            match day_solution.solve_part2(s) {
                Ok(r) => Ok(Box::new(r)),
                Err(e) => Err(e),
            }
        }),
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
