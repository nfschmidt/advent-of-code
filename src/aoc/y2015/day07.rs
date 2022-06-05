use crate::aoc::{DaySolution, Error, Result};
use std::collections::HashMap;

pub struct Solution;

#[derive(Debug)]
enum Instruction<'a> {
    ValueSignal(u16, &'a str),
    WireSignal(&'a str, &'a str),
    NOT(&'a str, &'a str),
    AND(&'a str, &'a str, &'a str),
    AND1(&'a str, &'a str),
    OR(&'a str, &'a str, &'a str),
    LSHIFT(&'a str, u16, &'a str),
    RSHIFT(&'a str, u16, &'a str),
}

impl<'a> Instruction<'a> {
    fn output(&self) -> &'a str {
        match self {
            Instruction::ValueSignal(_, output) => output,
            Instruction::WireSignal(_, output) => output,
            Instruction::AND(_, _, output) => output,
            Instruction::AND1(_, output) => output,
            Instruction::OR(_, _, output) => output,
            Instruction::NOT(_, output) => output,
            Instruction::LSHIFT(_, _, output) => output,
            Instruction::RSHIFT(_, _, output) => output,
        }
    }
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
}

impl<'a> std::iter::Iterator for InstructionsIterator<'a> {
    type Item = Result<Instruction<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        let line = self.lines.next()?;
        let fields = line.split_whitespace().collect::<Vec<_>>();

        match &fields[..] {
            &[signal, "->", wire] =>
                match signal.parse::<u16>() {
                    Ok(v) => Some(Ok(Instruction::ValueSignal(v, wire))),

                    // If it cannot be parse as a number, it is treated as
                    // a wire name
                    Err(_) => Some(Ok(Instruction::WireSignal(signal, wire))),
                },
            &["NOT", input, "->", output] =>
                Some(Ok(Instruction::NOT(
                            input,
                            output))),
            &["1", "AND", input, "->", output] =>
                Some(Ok(Instruction::AND1(
                            input,
                            output))),
            &[input1, "AND", input2, "->", output] =>
                Some(Ok(Instruction::AND(
                            input1,
                            input2,
                            output))),
            &[input1, "OR", input2, "->", output] =>
                Some(Ok(Instruction::OR(
                            input1,
                            input2,
                            output))),
            &[input1, "LSHIFT", input2, "->", output] =>
                match input2.parse::<u16>() {
                    Ok(v) => Some(Ok(Instruction::LSHIFT(
                                input1,
                                v,
                                output))),
                    Err(_) => Some(Err(Error::InvalidInput)),
                },
            &[input1, "RSHIFT", input2, "->", output] =>
                match input2.parse::<u16>() {
                    Ok(v) => Some(Ok(Instruction::RSHIFT(
                                input1,
                                v,
                                output))),
                    Err(_) => Some(Err(Error::InvalidInput)),
                },
            _ => Some(Err(Error::InvalidInput)),
        }
    }
}

impl DaySolution<u16> for Solution {
    fn solve_part1(&self, input: &str) -> Result<u16> {
        let mut out_inst: HashMap<&str, Instruction> = HashMap::new();
        for instruction in InstructionsIterator::new(input) {
            let inst = instruction?;
            out_inst.insert(inst.output(), inst);
        }

        let mut memo: HashMap<&str, u16> = HashMap::new();
        value_for("a", &out_inst, &mut memo)
    }

    fn solve_part2(&self, input: &str) -> Result<u16> {
        let value_for_a = self.solve_part1(input)?;         

        let mut out_inst: HashMap<&str, Instruction> = HashMap::new();
        for instruction in InstructionsIterator::new(input) {
            let inst = instruction?;
            out_inst.insert(inst.output(), inst);
        }

        out_inst.insert("b", Instruction::ValueSignal(value_for_a, "b"));

        let mut memo: HashMap<&str, u16> = HashMap::new();
        value_for("a", &out_inst, &mut memo)
    }
}

fn value_for<'a>(
    wire: &'a str,
    out_inst: &'a HashMap<&str, Instruction>,
    memo: &mut HashMap<&'a str, u16>,
) -> Result<u16> {
    if let Some(val) = memo.get(wire) {
        return Ok(*val);
    }

    let val = match out_inst.get(wire) {
        // the last field is the output, same as wire
        Some(Instruction::ValueSignal(value, _)) => Ok(*value),

        Some(Instruction::WireSignal(input, _)) =>
            Ok(value_for(input, out_inst, memo)?),

        Some(Instruction::NOT(input, _)) =>
            Ok(!(value_for(input, out_inst, memo)?)),

        Some(Instruction::AND(input1, input2, _)) =>
            Ok(value_for(input1, out_inst, memo)? & value_for(input2, out_inst, memo)?),

        Some(Instruction::AND1(input, _)) =>
            Ok(1 & value_for(input, out_inst, memo)?),

        Some(Instruction::OR(input1, input2, _)) =>
            Ok(value_for(input1, out_inst, memo)? | value_for(input2, out_inst, memo)?),

        Some(Instruction::LSHIFT(input1, input2, _)) =>
            Ok(value_for(input1, out_inst, memo)? << input2),

        Some(Instruction::RSHIFT(input1, input2, _)) =>
            Ok(value_for(input1, out_inst, memo)? >> input2),

        None => Err(Error::ResultNotFound),
    }?;

    memo.insert(wire, val);

    Ok(val)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::aoc::test::{test_case, Part};

    #[test]
    fn solve_part1_case_1() {
        // changed the wire "i" in the example to "a" so that it matches
        // the requested wire
        let input = "
            123 -> x
            456 -> y
            x AND y -> d
            x OR y -> e
            x LSHIFT 2 -> f
            y RSHIFT 2 -> g
            NOT x -> h
            NOT y -> a
        ";
        test_case(Part::One, Solution, input, 65079);
    }

    #[test]
    fn solve_part1_signal_with_reference() {
        let input = "
            123 -> x
            x -> a
        ";
        test_case(Part::One, Solution, input, 123);
    }

    #[test]
    fn solve_part2_simple_example() {
        let input = "
            1 -> b
            b LSHIFT 1 -> x
            x -> a
        ";
        test_case(Part::Two, Solution, input, 4);
    }
}
