use std::env;
use std::error::Error;
use std::fmt::Display;
use std::io;
use std::io::Read;
use advent_of_code_rust::aoc::{Part, Solution, y2015};

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
        (2015, 1) => Some(Box::new(y2015::day01::Solution::new())),
        (2015, 2) => Some(Box::new(y2015::day02::Solution::new())),
        (2015, 3) => Some(Box::new(y2015::day03::Solution::new())),
        (2015, 4) => Some(Box::new(y2015::day04::Solution::new())),
        (2015, 5) => Some(Box::new(y2015::day05::Solution::new())),
        _ => None,
    }
}

fn show_result(result: Option<Box::<dyn Display>>) {
    println!("{}", match result {
        None => "result not found".to_string(),
        Some(r) => r.to_string(),
    })
}
