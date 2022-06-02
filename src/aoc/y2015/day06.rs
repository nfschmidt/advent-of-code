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

impl DaySolution<usize> for Solution {
    fn solve_part1(&self, input: &str) -> Result<usize> {
        let mut lights = vec![Light::Off; 1_000_000];
        let mut instructions = Vec::new();

        for line in input.lines() {
            let fields = line.split_whitespace().collect::<Vec<_>>();
            if fields.len() < 4 {
                return Err(Error::InvalidInput);
            }

            match (fields[0], fields[1]) {
                ("turn", "on") => {
                    let start_range = fields[2].split(',').collect::<Vec<_>>();
                    if start_range.len() != 2 {
                        return Err(Error::InvalidInput);
                    }

                    let start_row = start_range[0].parse::<u32>().map_err(|_| Error::InvalidInput)?;
                    let start_column = start_range[1].parse::<u32>().map_err(|_| Error::InvalidInput)?;

                    let end_range = fields[4].split(',').collect::<Vec<_>>();
                    if end_range.len() != 2 {
                        return Err(Error::InvalidInput);
                    }

                    let end_row = end_range[0].parse::<u32>().map_err(|_| Error::InvalidInput)?;
                    let end_column = end_range[1].parse::<u32>().map_err(|_| Error::InvalidInput)?;

                    instructions.push(Instruction::On{
                        from: (start_row, start_column),
                        to: (end_row, end_column),
                    });
                },
                ("turn", "off") => {
                    let start_range = fields[2].split(',').collect::<Vec<_>>();
                    if start_range.len() != 2 {
                        return Err(Error::InvalidInput);
                    }

                    let start_row = start_range[0].parse::<u32>().map_err(|_| Error::InvalidInput)?;
                    let start_column = start_range[1].parse::<u32>().map_err(|_| Error::InvalidInput)?;

                    let end_range = fields[4].split(',').collect::<Vec<_>>();
                    if end_range.len() != 2 {
                        return Err(Error::InvalidInput);
                    }

                    let end_row = end_range[0].parse::<u32>().map_err(|_| Error::InvalidInput)?;
                    let end_column = end_range[1].parse::<u32>().map_err(|_| Error::InvalidInput)?;

                    instructions.push(Instruction::Off{
                        from: (start_row, start_column),
                        to: (end_row, end_column),
                    });
                },
                ("toggle", _) => {
                    let start_range = fields[1].split(',').collect::<Vec<_>>();
                    if start_range.len() != 2 {
                        return Err(Error::InvalidInput);
                    }

                    let start_row = start_range[0].parse::<u32>().map_err(|_| Error::InvalidInput)?;
                    let start_column = start_range[1].parse::<u32>().map_err(|_| Error::InvalidInput)?;

                    let end_range = fields[3].split(',').collect::<Vec<_>>();
                    if end_range.len() != 2 {
                        return Err(Error::InvalidInput);
                    }

                    let end_row = end_range[0].parse::<u32>().map_err(|_| Error::InvalidInput)?;
                    let end_column = end_range[1].parse::<u32>().map_err(|_| Error::InvalidInput)?;

                    instructions.push(Instruction::Toggle{
                        from: (start_row, start_column),
                        to: (end_row, end_column),
                    });
                }
                (_, _) => return Err(Error::InvalidInput),
            }
        }

        for ins in instructions {
            match ins {
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
        let mut instructions = Vec::new();

        for line in input.lines() {
            let fields = line.split_whitespace().collect::<Vec<_>>();
            if fields.len() < 4 {
                return Err(Error::InvalidInput);
            }

            match (fields[0], fields[1]) {
                ("turn", "on") => {
                    let start_range = fields[2].split(',').collect::<Vec<_>>();
                    if start_range.len() != 2 {
                        return Err(Error::InvalidInput);
                    }

                    let start_row = start_range[0].parse::<u32>().map_err(|_| Error::InvalidInput)?;
                    let start_column = start_range[1].parse::<u32>().map_err(|_| Error::InvalidInput)?;

                    let end_range = fields[4].split(',').collect::<Vec<_>>();
                    if end_range.len() != 2 {
                        return Err(Error::InvalidInput);
                    }

                    let end_row = end_range[0].parse::<u32>().map_err(|_| Error::InvalidInput)?;
                    let end_column = end_range[1].parse::<u32>().map_err(|_| Error::InvalidInput)?;

                    instructions.push(Instruction::On{
                        from: (start_row, start_column),
                        to: (end_row, end_column),
                    });
                },
                ("turn", "off") => {
                    let start_range = fields[2].split(',').collect::<Vec<_>>();
                    if start_range.len() != 2 {
                        return Err(Error::InvalidInput);
                    }

                    let start_row = start_range[0].parse::<u32>().map_err(|_| Error::InvalidInput)?;
                    let start_column = start_range[1].parse::<u32>().map_err(|_| Error::InvalidInput)?;

                    let end_range = fields[4].split(',').collect::<Vec<_>>();
                    if end_range.len() != 2 {
                        return Err(Error::InvalidInput);
                    }

                    let end_row = end_range[0].parse::<u32>().map_err(|_| Error::InvalidInput)?;
                    let end_column = end_range[1].parse::<u32>().map_err(|_| Error::InvalidInput)?;

                    instructions.push(Instruction::Off{
                        from: (start_row, start_column),
                        to: (end_row, end_column),
                    });
                },
                ("toggle", _) => {
                    let start_range = fields[1].split(',').collect::<Vec<_>>();
                    if start_range.len() != 2 {
                        return Err(Error::InvalidInput);
                    }

                    let start_row = start_range[0].parse::<u32>().map_err(|_| Error::InvalidInput)?;
                    let start_column = start_range[1].parse::<u32>().map_err(|_| Error::InvalidInput)?;

                    let end_range = fields[3].split(',').collect::<Vec<_>>();
                    if end_range.len() != 2 {
                        return Err(Error::InvalidInput);
                    }

                    let end_row = end_range[0].parse::<u32>().map_err(|_| Error::InvalidInput)?;
                    let end_column = end_range[1].parse::<u32>().map_err(|_| Error::InvalidInput)?;

                    instructions.push(Instruction::Toggle{
                        from: (start_row, start_column),
                        to: (end_row, end_column),
                    });
                }
                (_, _) => return Err(Error::InvalidInput),
            }
        }

        for ins in instructions {
            match ins {
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
