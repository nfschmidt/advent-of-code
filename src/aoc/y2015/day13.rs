use std::collections::{HashMap, HashSet};

use crate::aoc::{DaySolution, Error, Result};

pub struct Solution;

impl DaySolution<i32> for Solution {
    fn solve_part1(&self, input: &str) -> Result<i32> {
        let (people, relations) = parse_input(input)?;

        best_arrangement_score(&people, &relations)
    }

    fn solve_part2(&self, input: &str) -> Result<i32> {
        let (mut people, mut relations) = parse_input(input)?;

        for p in people.iter() {
            relations.insert(("", p), 0);
            relations.insert((p, ""), 0);
        }
        people.push("");

        best_arrangement_score(&people, &relations)
    }
}

fn parse_input(input: &str) -> Result<(Vec<&str>, HashMap<(&str, &str), i32>)> {
    let mut people = HashSet::new();
    let mut relations = HashMap::new();

    for line in input.trim().lines() {
        let fields = line.split_whitespace().collect::<Vec<_>>();
        let p1 = fields[0];
        let p2 = fields[10].trim_end_matches('.');
        let score = match fields[2] {
            "gain" => fields[3].parse().map_err(|_| Error::InvalidInput),
            "lose" => fields[3]
                .parse()
                .map(|x: i32| -x)
                .map_err(|_| Error::InvalidInput),
            _ => Err(Error::InvalidInput),
        }?;

        relations.insert((p1, p2), score);

        people.insert(p1);
        people.insert(p2);
    }

    Ok((people.into_iter().collect(), relations))
}

fn best_arrangement_score(people: &[&str], relations: &HashMap<(&str, &str), i32>) -> Result<i32> {
    let perms = permutations(&people);

    let mut result = happiness(&perms[0], &relations)?;
    for perm in perms.into_iter().skip(1) {
        let score = happiness(&perm, &relations)?;
        if score > result {
            result = score;
        }
    }

    Ok(result)
}

// TODO: this logic was copypasted from day 09. Remove duplicated code
fn permutations<'a, T: ?Sized>(items: &[&'a T]) -> Vec<Vec<&'a T>> {
    let mut items = items.iter().map(|i| *i).collect::<Vec<_>>();
    let count = items.len();

    if count == 0 {
        return vec![];
    }

    if count == 1 {
        let elem = items[0].clone();
        return vec![vec![elem]];
    }

    let mut result = vec![];
    for idx in (0..count).rev() {
        let last = items[count - 1].clone();

        let mut perms = permutations(&mut items[0..count - 1]);
        while let Some(mut perm) = perms.pop() {
            perm.push(last.clone());
            result.push(perm);
        }

        items.swap(count - 1, idx);
    }

    result
}

fn happiness(arrangement: &[&str], relations: &HashMap<(&str, &str), i32>) -> Result<i32> {
    let mut result = 0;
    for i in 0..arrangement.len() - 1 {
        result += relations
            .get(&(arrangement[i], arrangement[i + 1]))
            .ok_or(Error::InvalidInput)?;
        result += relations
            .get(&(arrangement[i + 1], arrangement[i]))
            .ok_or(Error::InvalidInput)?;
    }

    result += relations
        .get(&(arrangement[arrangement.len() - 1], arrangement[0]))
        .ok_or(Error::InvalidInput)?;
    result += relations
        .get(&(arrangement[0], arrangement[arrangement.len() - 1]))
        .ok_or(Error::InvalidInput)?;

    Ok(result)
}

#[cfg(test)]
mod test {
    use crate::aoc::test::{test_case, Part};

    use super::*;

    #[test]
    fn test_solve_part1_case1() {
        let input = "
            Alice would gain 54 happiness units by sitting next to Bob.
            Alice would lose 79 happiness units by sitting next to Carol.
            Alice would lose 2 happiness units by sitting next to David.
            Bob would gain 83 happiness units by sitting next to Alice.
            Bob would lose 7 happiness units by sitting next to Carol.
            Bob would lose 63 happiness units by sitting next to David.
            Carol would lose 62 happiness units by sitting next to Alice.
            Carol would gain 60 happiness units by sitting next to Bob.
            Carol would gain 55 happiness units by sitting next to David.
            David would gain 46 happiness units by sitting next to Alice.
            David would lose 7 happiness units by sitting next to Bob.
            David would gain 41 happiness units by sitting next to Carol.
        ";

        test_case(Part::One, Solution, input, 330);
    }

    #[test]
    fn test_solve_part2_case1() {
        let input = "
            Alice would gain 54 happiness units by sitting next to Bob.
            Alice would lose 79 happiness units by sitting next to Carol.
            Alice would lose 2 happiness units by sitting next to David.
            Bob would gain 83 happiness units by sitting next to Alice.
            Bob would lose 7 happiness units by sitting next to Carol.
            Bob would lose 63 happiness units by sitting next to David.
            Carol would lose 62 happiness units by sitting next to Alice.
            Carol would gain 60 happiness units by sitting next to Bob.
            Carol would gain 55 happiness units by sitting next to David.
            David would gain 46 happiness units by sitting next to Alice.
            David would lose 7 happiness units by sitting next to Bob.
            David would gain 41 happiness units by sitting next to Carol.
        ";

        test_case(Part::Two, Solution, input, 286);
    }
}
