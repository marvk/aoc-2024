use crate::harness::Day;
use crate::harness::Part;
use std::collections::HashMap;

pub struct Part1;

pub fn day05() -> Day<u32, u32> {
    Day::new(5, Box::new(Part1 {}), Box::new(Part2 {}))
}

impl Part<u32> for Part1 {
    fn expect_test(&self) -> u32 {
        143
    }

    fn solve(&self, input: &[String]) -> u32 {
        let (rules, changes) = parse(input);

        let (correct, _) = partition(changes, &rules);

        correct.iter().map(|e| e.center).sum()
    }
}

pub struct Part2;

impl Part<u32> for Part2 {
    fn expect_test(&self) -> u32 {
        123
    }

    fn solve(&self, input: &[String]) -> u32 {
        let (rules, changes) = parse(input);

        let (_, incorrect) = partition(changes, &rules);

        incorrect
            .into_iter()
            .map(|e| e.values)
            .map(|mut e| {
                fix(&mut e, &rules);
                e
            })
            .map(|e| mid(&e))
            .sum()
    }
}

#[derive(Debug)]
struct Rule {
    first: u32,
    second: u32,
}

impl From<&str> for Rule {
    fn from(value: &str) -> Self {
        let values = value.split("|").flat_map(|e| e.parse()).collect::<Vec<_>>();

        Self {
            first: values[0],
            second: values[1],
        }
    }
}

impl Rule {
    fn check(&self, change: &Change) -> bool {
        match (
            change.pages.get(&self.first),
            change.pages.get(&self.second),
        ) {
            (Some(i1), Some(i2)) => i1 < i2,
            _ => true,
        }
    }
}

#[derive(Debug)]
struct Change {
    values: Vec<u32>,
    pages: HashMap<u32, usize>,
    center: u32,
}

impl From<&str> for Change {
    fn from(value: &str) -> Self {
        value
            .split(",")
            .flat_map(|e| e.parse::<u32>())
            .collect::<Vec<_>>()
            .into()
    }
}

impl From<Vec<u32>> for Change {
    fn from(value: Vec<u32>) -> Self {
        let center = mid(&value);

        let pages = value
            .iter()
            .enumerate()
            .map(|(i, e)| (*e, i))
            .collect::<HashMap<_, _>>();

        Self {
            values: value,
            pages,
            center,
        }
    }
}

fn parse(input: &[String]) -> (Vec<Rule>, Vec<Change>) {
    let vec = input.split(|e| e.is_empty()).collect::<Vec<_>>();

    let rules = vec[0]
        .iter()
        .map(|e| Rule::from(e.as_str()))
        .collect::<Vec<_>>();
    let changes = vec[1]
        .iter()
        .map(|e| Change::from(e.as_str()))
        .collect::<Vec<_>>();

    (rules, changes)
}

fn partition(changes: Vec<Change>, rules: &[Rule]) -> (Vec<Change>, Vec<Change>) {
    changes
        .into_iter()
        .partition(|c| rules.iter().all(|r| r.check(c)))
}

fn fix(vec1: &mut [u32], rules: &Vec<Rule>) {
    loop {
        let mut swap = false;
        for rule in rules {
            let first_index = vec1.iter().position(|&e| e == rule.first);
            let second_index = vec1.iter().position(|&e| e == rule.second);

            if let (Some(first_index), Some(second_index)) = (first_index, second_index) {
                if first_index > second_index {
                    vec1.swap(first_index, second_index);
                    swap = true
                }
            }
        }
        if !swap {
            break;
        }
    }
}

fn mid(slice: &[u32]) -> u32 {
    slice[slice.len() / 2]
}
