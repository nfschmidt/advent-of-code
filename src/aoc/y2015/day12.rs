use crate::aoc::DaySolution;
use crate::aoc::Result;
use regex::Regex;
use serde_json::Value;

pub struct Solution;

impl DaySolution<i64> for Solution {
    fn solve_part1(&self, input: &str) -> Result<i64> {
        let re = Regex::new(r"-?\d+").unwrap();

        let mut result = 0;
        for cap in re.captures_iter(input) {
            result += cap[0].parse::<i64>().unwrap();
        }

        Ok(result)
    }

    fn solve_part2(&self, input: &str) -> Result<i64> {
        let value: Value = serde_json::from_str(input).unwrap();

        Ok(json_sum(&value))
    }
}

fn json_sum(value: &Value) -> i64 {
    match value {
        Value::Number(n) => n.as_i64().unwrap(),
        Value::Object(kvs) => {
            let mut result = 0;
            for (_, v) in kvs.iter() {
                match v {
                    Value::String(s) => {
                        if s == "red" {
                            result = 0;
                            break;
                        }
                    }
                    _ => result += json_sum(v),
                };
            }

            result
        }
        Value::Array(arr) => arr.iter().map(|v| json_sum(v)).sum(),
        _ => 0,
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::aoc::test::{test_case, Part};

    #[test]
    fn solve_part1_case1() {
        test_case(Part::One, Solution, "[1,2,3]", 6);
    }

    #[test]
    fn solve_part1_case2() {
        test_case(Part::One, Solution, "{\"a\":2,\"b\":4}", 6);
    }

    #[test]
    fn solve_part1_case3() {
        test_case(Part::One, Solution, "[[[3]]]", 3);
    }

    #[test]
    fn solve_part1_case4() {
        test_case(Part::One, Solution, "{\"a\":{\"b\":4},\"c\":-1}", 3);
    }

    #[test]
    fn solve_part1_case5() {
        test_case(Part::One, Solution, "{\"a\":[-1,1]}", 0);
    }

    #[test]
    fn solve_part1_case6() {
        test_case(Part::One, Solution, "[-1,{\"a\":1}]", 0);
    }

    #[test]
    fn solve_part2_case1() {
        test_case(Part::Two, Solution, "[1,2,3]", 6);
    }

    #[test]
    fn solve_part2_case2() {
        test_case(Part::Two, Solution, "[1,{\"c\":\"red\",\"b\":2},3]", 4);
    }

    #[test]
    fn solve_part2_case3() {
        test_case(
            Part::Two,
            Solution,
            "{\"d\":\"red\",\"e\":[1,2,3,4],\"f\":5}",
            0,
        );
    }

    #[test]
    fn solve_part2_case4() {
        test_case(Part::Two, Solution, "[1,\"red\",5]", 6);
    }
}
