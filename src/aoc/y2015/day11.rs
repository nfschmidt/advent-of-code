use crate::aoc::{DaySolution, Result};

pub struct Solution;

impl DaySolution<String> for Solution {
    fn solve_part1(&self, input: &str) -> Result<String> {
        let mut password = input.trim().chars().collect::<Vec<_>>();

        to_next_valid_password(&mut password);

        Ok(password.into_iter().collect())
    }

    fn solve_part2(&self, input: &str) -> Result<String> {
        let mut password = input.trim().chars().collect::<Vec<_>>();

        to_next_valid_password(&mut password);
        to_next_valid_password(&mut password);

        Ok(password.into_iter().collect())
    }
}

fn to_next_valid_password(password: &mut [char]) {
    to_next_password(password);
    while !is_valid(&password) {
        to_next_password(password)
    }
}

fn to_next_password(input: &mut [char]) {
    let len = input.len();
    if len == 0 {
        return;
    }

    let last = input[len - 1];
    if last == 'z' {
        to_next_password(&mut input[0..len - 1]);
        input[len - 1] = 'a';
    } else {
        input[len - 1] = (input[len - 1] as u8 + 1) as char;
    }
}

fn is_valid(password: &[char]) -> bool {
    let mut straight = false;
    let mut pairs = 0;
    let mut prev1 = 0; // out of range initial char
    let mut prev2 = 0; // out of range initial char

    for i in 0..password.len() {
        let chr = password[i];

        if chr == 'i' || chr == 'o' || chr == 'l' {
            return false;
        }

        if prev2 + 1 == prev1 && prev1 + 1 == chr as u8 {
            straight = true;
        }

        if chr as u8 == prev1 && prev1 != prev2 {
            pairs += 1;
        }

        (prev1, prev2) = (chr as u8, prev1);
    }

    straight && pairs >= 2
}

#[cfg(test)]
mod test {
    use crate::aoc::test::{test_case, Part};

    use super::*;

    #[test]
    fn solve_part1_case1() {
        test_case(Part::One, Solution, "abcdefgh", "abcdffaa".to_owned())
    }

    #[test]
    #[ignore = "long running test"]
    fn solve_part1_case2() {
        test_case(Part::One, Solution, "ghijklmn", "ghjaabcc".to_owned())
    }
}
