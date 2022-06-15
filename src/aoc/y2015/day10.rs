use crate::aoc::{DaySolution, Result};

pub struct Solution;

impl DaySolution<u32> for Solution {
    fn solve_part1(&self, input: &str) -> Result<u32> {
        Ok(repeated_look_and_say(input, 40))
    }

    fn solve_part2(&self, input: &str) -> Result<u32> {
        Ok(repeated_look_and_say(input, 50))
    }
}

fn look_and_say(input: &str) -> String {
    if input.len() == 0 {
        return "".to_owned();
    }

    let mut result = String::new();
    let mut count = 1;

    let mut chars = input.trim().chars();
    let mut prev_char = chars.next().unwrap();

    while let Some(c) = chars.next() {
        if c == prev_char {
            count += 1;
        } else {
            result.push_str(&format!("{}{}", count, prev_char));
            count = 1;
        }

        prev_char = c;
    }
    result.push_str(&format!("{}{}", count, prev_char));

    result
}

fn repeated_look_and_say(input: &str, count: u8) -> u32 {
        let mut result_string = input.to_owned();

        for _ in 0..count {
            result_string = look_and_say(&result_string);
        }

        result_string.len() as u32
}

#[cfg(test)]
mod test {
    use crate::aoc::test::{test_case, Part};

    use super::*;

    #[test]
    fn look_and_say_part1_case1() {
        let result = look_and_say("1");
        assert_eq!(result, "11");
    }

    #[test]
    fn look_and_say_part1_case2() {
        let result = look_and_say("11");
        assert_eq!(result, "21");
    }

    #[test]
    fn look_and_say_part1_case3() {
        let result = look_and_say("21");
        assert_eq!(result, "1211");
    }

    #[test]
    fn look_and_say_part1_case4() {
        let result = look_and_say("1211");
        assert_eq!(result, "111221");
    }

    #[test]
    fn look_and_say_part1_case5() {
        let result = look_and_say("111221");
        assert_eq!(result, "312211");
    }

    #[test]
    fn solve_part1() {
        test_case(Part::One, Solution, "1", 82350)
    }

    #[test]
    #[ignore = "long running test"]
    fn solve_part2() {
        test_case(Part::Two, Solution, "1", 1166642)
    }
}
