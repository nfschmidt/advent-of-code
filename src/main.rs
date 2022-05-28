use std::io;
use std::io::Read;
use std::error::Error;
use std::env;

fn main() -> Result<(), Box<dyn Error>> {
    let solver = get_solver(env::args())
        .ok_or("invalid problem".to_string())?;

    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let result = solver(&input);

    show_result(result);
    Ok(())
}

fn aoc2015_day01_part1(input: &str) -> Option<String> {
    Some(input
        .chars()
        .map(|c| {
            match c {
                '(' => 1,
                ')' => -1,
                _ => 0,
            }
        })
        .sum::<i32>()
        .to_string())
}

fn aoc2015_day01_part2(input: &str) -> Option<String> {
    let mut floor = 0;
    for (i, c) in input.chars().enumerate() {
        floor += match c {
            '(' => 1,
            ')' => -1,
            _ => 0,
        };

        if floor == -1 {
            return Some((i+1).to_string());
        }
    }

    None
}

fn get_solver(args: env::Args) -> Option<Box<dyn Fn(&str) -> Option<String>>> {
    let solver_id = args
        .skip(1)
        .map(|n| n.parse())
        .collect::<Result<Vec<u16>, _>>()
        .ok()?;

    match &solver_id[..] {
        &[2015, 1, 1] => Some(Box::new(aoc2015_day01_part1)),
        &[2015, 1, 2] => Some(Box::new(aoc2015_day01_part2)),
        _ => None,
    }
}

fn show_result(result: Option<String>) {
    println!("{}", match result {
        None => "result not found".to_string(),
        Some(r) => r,
    })
}
