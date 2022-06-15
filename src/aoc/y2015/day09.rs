use std::collections::{HashMap, HashSet};

use crate::aoc::{DaySolution, Error, Result};

type City = String;

pub struct Solution;

impl DaySolution<u32> for Solution {
    fn solve_part1(&self, input: &str) -> Result<u32> {
        best_route(input, |x, y| y < x)
    }

    fn solve_part2(&self, input: &str) -> Result<u32> {
        best_route(input, |x, y| y > x)
    }
}

fn best_route(input: &str, cmp: impl Fn(u32, u32) -> bool) -> Result<u32> {
    let mut distances: HashMap<(City, City), u32> = HashMap::new();
    let mut cities: HashSet<City> = HashSet::new();
    for line in input.trim().lines() {
        let (city1, city2, distance) = parse_distance(line)?;
        cities.insert(city1.clone());
        cities.insert(city2.clone());
        distances.insert((city1.clone(), city2.clone()), distance);
        distances.insert((city2, city1), distance);
    }

    let perms = permutations(&mut cities.into_iter().collect::<Vec<_>>());
    let mut result = trip_distance(&perms[0], &distances);

    for perm in perms {
        let distance = trip_distance(&perm, &distances);
        if cmp(result, distance) {
            result = distance
        }
    }

    Ok(result)
}

fn trip_distance(perms: &[City], distances: &HashMap<(City, City), u32>) -> u32 {
    perms
        .iter()
        .zip(&perms[1..])
        .map(|(c1, c2)| distances.get(&(c1.clone(), c2.clone())).unwrap())
        .sum()
}

fn parse_distance(line: &str) -> Result<(City, City, u32)> {
    let fields = line.split_whitespace().collect::<Vec<_>>();
    let city1 = fields.get(0).ok_or(Error::InvalidInput)?;
    let city2 = fields.get(2).ok_or(Error::InvalidInput)?;
    let distance = fields
        .get(4)
        .ok_or(Error::InvalidInput)?
        .parse()
        .map_err(|_| Error::InvalidInput)?;

    Ok((city1.to_string(), city2.to_string(), distance))
}

fn permutations(cities: &mut [City]) -> Vec<Vec<City>> {
    let count = cities.len();

    if count == 0 {
        return vec![];
    }

    if count == 1 {
        let elem = cities[0].clone();
        return vec![vec![elem]];
    }

    let mut result = vec![];
    for idx in (0..count).rev() {
        let last = cities[count - 1].clone();

        let mut perms = permutations(&mut cities[0..count - 1]);
        while let Some(mut perm) = perms.pop() {
            perm.push(last.clone());
            result.push(perm);
        }

        cities.swap(count - 1, idx);
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::aoc::test::{test_case, Part};

    #[test]
    fn solve_part1_case_1() {
        let input = "
            London to Dublin = 464
            London to Belfast = 518
            Dublin to Belfast = 141
        ";
        test_case(Part::One, Solution, input, 605);
    }

    #[test]
    fn solve_part2_case_1() {
        let input = "
            London to Dublin = 464
            London to Belfast = 518
            Dublin to Belfast = 141
        ";
        test_case(Part::Two, Solution, input, 982);
    }
}
