use crate::harness::Day;
use crate::harness::Part;
use std::cmp::max;
use std::cmp::min;
use std::collections::HashMap;

pub struct Part1;

pub fn day01() -> Day<i32, i32> {
    Day::new(1, Box::new(Part1 {}), Box::new(Part2 {}))
}

impl Part<i32> for Part1 {
    fn expect_test(&self) -> i32 {
        11
    }

    fn solve(&self, input: &[String]) -> i32 {
        let (mut vec1, mut vec2) = parse(input);

        vec1.sort();
        vec2.sort();

        vec1.iter()
            .zip(vec2)
            .map(|(&a, b)| max(a, b) - min(a, b))
            .sum()
    }
}

pub struct Part2;

impl Part<i32> for Part2 {
    fn expect_test(&self) -> i32 {
        31
    }

    fn solve(&self, input: &[String]) -> i32 {
        let (vec1, vec2) = parse(input);

        let frequencies = vec2.into_iter().fold(HashMap::new(), |mut map, e| {
            map.entry(e).and_modify(|v| *v += 1).or_insert(1);

            map
        });

        vec1.into_iter()
            .map(|e| {
                let factor = frequencies.get(&e).copied().unwrap_or(0);
                factor * e
            })
            .sum()
    }
}

fn get_list(vec: &[Vec<&str>], i: usize) -> Vec<i32> {
    vec.iter()
        .map(|e| e[i])
        .map(|e| e.parse::<i32>().unwrap())
        .collect::<Vec<_>>()
}

fn parse(input: &[String]) -> (Vec<i32>, Vec<i32>) {
    let vec = input
        .iter()
        .filter(|e| !e.is_empty())
        .map(|e| e.split("   ").collect::<Vec<_>>())
        .collect::<Vec<_>>();

    (get_list(&vec, 0), get_list(&vec, 1))
}
