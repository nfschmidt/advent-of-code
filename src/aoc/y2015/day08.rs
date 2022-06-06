use crate::aoc::{DaySolution, Error, Result};

pub struct Solution;

impl DaySolution<u32> for Solution {
    fn solve_part1(&self, input: &str) -> Result<u32> {
        let mut result = 0;

        for line in input.trim().lines() {
            let chars = line.trim().chars().collect::<Vec<_>>();
            let code_len = chars.len();
            let mut memory_len = 0;
            let mut i = 0;

            while i < chars.len() {
                match chars.get(i) {
                    Some('\\') => {
                        i += 1;
                        match chars.get(i) {
                            Some('\\') => memory_len += 1,
                            Some('"') => memory_len += 1,
                            Some('x') => {
                                i += 2;
                                memory_len += 1
                            },
                            Some(_) => return Err(Error::InvalidInput),
                            None => return Err(Error::InvalidInput),
                        }
                    },
                    Some('"') => memory_len += 0,
                    Some(_) => memory_len += 1,
                    None => break,
                }

                i += 1;
            }

            result += code_len - memory_len;
        }

        Ok(result.try_into().unwrap())
    }

    fn solve_part2(&self, input: &str) -> Result<u32> {
        let mut result = 0;
        for line in input.trim().lines() {
            let code_len = line.len();
            let encoded_len: u32 =
               line
               .chars()
               .map(|c| match c {
                   '\\' => 2,
                   '"' => 2,
                   _ => 1,
               })
               .sum::<u32>() + 2;

           result += encoded_len - code_len as u32;
        }

        Ok(result)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::aoc::test::{test_case, Part};

    #[test]
    fn solve_part1_case_1() {
        test_case(Part::One, Solution, "\"\"", 2);
    }

    #[test]
    fn solve_part1_case_2() {
        test_case(Part::One, Solution, "\"abc\"", 2);
    }

    #[test]
    fn solve_part1_case_3() {
        test_case(Part::One, Solution, "\"aaa\\\"aaa\"", 3);
    }

    #[test]
    fn solve_part1_case_4() {
        test_case(Part::One, Solution, "\"\\x27\"", 5);
    }

    #[test]
    fn solve_part1_case_5() {
        let input = "
            \"\"
            \"abc\"
            \"aaa\\\"aaa\"
            \"\\x27\"
        ";

        test_case(Part::One, Solution, input, 12);
    }

    #[test]
    fn solve_part2_case_1() {
        test_case(Part::Two, Solution, "\"\"", 4);
    }

    #[test]
    fn solve_part2_case_2() {
        test_case(Part::Two, Solution, "\"abc\"", 4);
    }

    #[test]
    fn solve_part2_case_3() {
        test_case(Part::Two, Solution, "\"aaa\\\"aaa\"", 6);
    }

    #[test]
    fn solve_part2_case_4() {
        test_case(Part::Two, Solution, "\"\\x27\"", 5);
    }

    #[test]
    fn solve_part2_case_5() {
        let input = "
            \"\"
            \"abc\"
            \"aaa\\\"aaa\"
            \"\\x27\"
        ";

        test_case(Part::Two, Solution, input, 19);
    }
}
