use std::collections::HashSet;
use crate::aoc::{DaySolution, Result};

pub struct Solution;

impl Solution {
    fn nice_count(&self, is_nice: impl Fn(&str) -> bool, data: Vec<String>) -> u32 {
        data
            .iter()
            .filter(|s| is_nice(s.as_str()))
            .collect::<Vec<_>>()
            .len() as u32
    }

    fn parse_input(&self, input: &str) -> Result<Vec<String>> {
        Ok(input
           .trim()
           .lines()
           .map(|s| s.to_string())
           .collect())
    }
}

impl DaySolution<u32> for Solution {
    fn solve_part1(&self, input: &str) -> Result<u32> {
        let vowels = HashSet::from(['a', 'e', 'i', 'o', 'u']);
        let invalids = HashSet::from(["ab", "cd", "pq", "xy"]);

        let data = self.parse_input(input)?;
        let nice = self.nice_count(|s| {
            let mut vowel = 0;
            let mut twice = false;

            let mut last_char = ' '; // set starting value with invalid char
            for c in s.chars() {
                if vowels.contains(&c) {
                    vowel += 1;
                }

                if last_char == c {
                    twice = true;
                }

                let st = format!("{}{}", last_char, c);
                if invalids.contains(st.as_str()) {
                    return false;
                }

                last_char = c;
            }

            return vowel >= 3 && twice;
        }, data);

        Ok(nice)
    }

    fn solve_part2(&self, input: &str) -> Result<u32> {
        let data = self.parse_input(input)?;
        let nice = self.nice_count(|s| {
            if s.len() < 2 {
                return false
            }

            let mut pairs = HashSet::new();

            let mut twice = false;
            let mut repeat = false;

            let mut chars = s.chars();
            let mut last_2 = chars.next().unwrap();
            let mut last_1 = chars.next().unwrap();

            for c in chars {
                if c == last_2 {
                    repeat = true;
                }

                if pairs.contains(format!("{}{}", last_1, c).as_str()) {
                    twice = true;
                }

                pairs.insert(format!("{}{}", last_2, last_1));

                (last_2, last_1) = (last_1, c);
            }

            return twice && repeat;
        }, data);

        Ok(nice)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::aoc::test::{test_case, Part};

    #[test]
    fn solve_part1_case_1() {
        test_case(Part::One, Solution, "ugknbfddgicrmopn", 1);
    }

    #[test]
    fn solve_part1_case_2() {
        test_case(Part::One, Solution, "aaa", 1);
    }

    #[test]
    fn solve_part1_case_3() {
        test_case(Part::One, Solution, "jchzalrnumimnmhp", 0);
    }

    #[test]
    fn solve_part1_case_4() {
        test_case(Part::One, Solution, "haegwjzuvuyypxyu", 0);
    }

    #[test]
    fn solve_part1_case_5() {
        test_case(Part::One, Solution, "dvszwmarrgswjxmb", 0);
    }

    #[test]
    fn solve_part2_case_1() {
        test_case(Part::Two, Solution, "qjhvhtzxzqqjkmpb", 1);
    }

    #[test]
    fn solve_part2_case_2() {
        test_case(Part::Two, Solution, "xxyxx", 1);
    }

    #[test]
    fn solve_part2_case_3() {
        test_case(Part::Two, Solution, "uurcxstgmygtbstg", 0);
    }

    #[test]
    fn solve_part2_case_4() {
        test_case(Part::Two, Solution, "ieodomkazucvgmuy", 0);
    }
}
