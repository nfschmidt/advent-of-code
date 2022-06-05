use crate::aoc::{DaySolution, Error, Result};

pub struct Solution;

enum Instruction {
    On{from: (u32, u32), to: (u32, u32)},
    Toggle{from: (u32, u32), to: (u32, u32)},
    Off{from: (u32, u32), to: (u32, u32)},
}

#[derive(Clone)]
enum Light {
    On,
    Off,
}

struct InstructionsIterator<'a> {
    lines: std::str::Lines<'a>,
}

impl<'a> InstructionsIterator<'a> {
    fn new(input: &'a str) -> Self {
        InstructionsIterator {
            lines: input.trim().lines(),
        }
    }

    fn parse_ranges(fields: &[&str], range1_idx: usize, range2_idx: usize) -> Result<((u32, u32), (u32, u32))> {
        let start_range = fields[range1_idx].split(',').collect::<Vec<_>>();
        if start_range.len() != 2 {
            return Err(Error::InvalidInput);
        }

        let start_row = start_range[0].parse::<u32>().map_err(|_| Error::InvalidInput)?;
        let start_column = start_range[1].parse::<u32>().map_err(|_| Error::InvalidInput)?;

        let end_range = fields[range2_idx].split(',').collect::<Vec<_>>();
        if end_range.len() != 2 {
            return Err(Error::InvalidInput);
        }

        let end_row = end_range[0].parse::<u32>().map_err(|_| Error::InvalidInput)?;
        let end_column = end_range[1].parse::<u32>().map_err(|_| Error::InvalidInput)?;

        Ok(((start_row, start_column), (end_row, end_column)))
    }
}

impl<'a> std::iter::Iterator for InstructionsIterator<'a> {
    type Item = Result<Instruction>;

    fn next(&mut self) -> Option<Self::Item> {
        let line = self.lines.next()?;

        let fields = line.split_whitespace().collect::<Vec<_>>();
        if fields.len() < 4 {
            return Some(Err(Error::InvalidInput));
        }

        match (fields[0], fields[1]) {
            ("turn", "on") =>
                return match Self::parse_ranges(&fields, 2, 4) {
                    Ok(((start_row, start_column), (end_row, end_column))) =>
                        Some(Ok(Instruction::On{
                            from: (start_row, start_column),
                            to: (end_row, end_column),
                        })),
                    _ => return Some(Err(Error::InvalidInput)),
                },
            ("turn", "off") =>
                return match Self::parse_ranges(&fields, 2, 4) {
                    Ok(((start_row, start_column), (end_row, end_column))) =>
                        Some(Ok(Instruction::Off{
                            from: (start_row, start_column),
                            to: (end_row, end_column),
                        })),
                    _ => return Some(Err(Error::InvalidInput)),
                },
            ("toggle", _) =>
                return match Self::parse_ranges(&fields, 1, 3) {
                    Ok(((start_row, start_column), (end_row, end_column))) =>
                        Some(Ok(Instruction::Toggle{
                            from: (start_row, start_column),
                            to: (end_row, end_column),
                        })),
                    _ => return Some(Err(Error::InvalidInput)),
                },
            (_, _) => return Some(Err(Error::InvalidInput)),
        }
    }
}

impl Solution {
    fn instructions_iter<'a>(&self, input: &'a str) -> InstructionsIterator<'a> {
        InstructionsIterator::new(input)
    }
}

impl DaySolution<usize> for Solution {
    fn solve_part1(&self, input: &str) -> Result<usize> {
        let mut lights = vec![Light::Off; 1_000_000];

        for ins in self.instructions_iter(input) {
            match ins? {
                Instruction::On{from: (fr, fc), to: (tr, tc)} => {
                    for row in fr..=tr {
                        for col in fc..=tc {
                            lights[(row*1000+col) as usize] = Light::On;
                        }
                    }
                },
                Instruction::Off{from: (fr, fc), to: (tr, tc)} => {
                    for row in fr..=tr {
                        for col in fc..=tc {
                            lights[(row*1000+col) as usize] = Light::Off;
                        }
                    }
                },
                Instruction::Toggle{from: (fr, fc), to: (tr, tc)} => {
                    for row in fr..=tr {
                        for col in fc..=tc {
                            lights[(row*1000+col) as usize] =
                                match lights[(row*1000+col) as usize] {
                                    Light::On => Light::Off,
                                    Light::Off => Light::On,
                                }
                        }
                    }
                },
            }
        }

        let result =
            lights
            .into_iter()
            .filter(|l| if let Light::On = l { true } else { false })
            .collect::<Vec<_>>()
            .len();

        Ok(result)
    }

    fn solve_part2(&self, input: &str) -> Result<usize> {
        let mut brightness: Vec::<u32> = vec![0; 1_000_000];

        for ins in self.instructions_iter(input) {
            match ins? {
                Instruction::On{from: (fr, fc), to: (tr, tc)} => {
                    for row in fr..=tr {
                        for col in fc..=tc {
                            brightness[(row*1000+col) as usize] += 1;
                        }
                    }
                },
                Instruction::Off{from: (fr, fc), to: (tr, tc)} => {
                    for row in fr..=tr {
                        for col in fc..=tc {
                            if brightness[(row*1000+col) as usize] > 0 {
                                brightness[(row*1000+col) as usize] -= 1;
                            }
                        }
                    }
                },
                Instruction::Toggle{from: (fr, fc), to: (tr, tc)} => {
                    for row in fr..=tr {
                        for col in fc..=tc {
                            brightness[(row*1000+col) as usize] += 2;
                        }
                    }
                },
            }
        }

        let result =
            brightness
            .into_iter()
            .sum::<u32>() as usize;

        Ok(result)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::aoc::test::{test_case, Part};

    #[test]
    fn solve_part1_case_1() {
        test_case(Part::One, Solution, "turn on 0,0 through 999,999", 1_000_000);
    }

    #[test]
    fn solve_part1_case_2() {
        // 4 on before toggling the first column
        // should result in 1_000 - 4 lights on
        let input = "
            turn on 0,0 through 3,0
            toggle 0,0 through 999,0
        ";

        test_case(Part::One, Solution, input, 1_000 - 4);
    }

    #[test]
    fn solve_part1_case_3() {
        // turn on everything except for 4 in the middle
        // should result in 1_000_000 - 4 lights on
        let input = "
            turn on 0,0 through 999,999
            turn off 499,499 through 500,500
        ";
        test_case(Part::One, Solution, input, 1_000_000 - 4);
    }

    #[test]
    fn solve_part2_case_1() {
        test_case(Part::Two, Solution, "turn on 0,0 through 0,0", 1);
    }

    #[test]
    fn solve_part2_case_2() {
        test_case(Part::Two, Solution, "toggle 0,0 through 999,999", 2_000_000);
    }
}
