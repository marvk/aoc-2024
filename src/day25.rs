use crate::harness::Day;
use crate::harness::Part;
use std::ops::{BitOr, Shl};

pub fn day25() -> Day<u32, i32> {
    Day::new(25, Box::new(Part1 {}), Box::new(Part2 {}))
}

pub struct Part1;

impl Part<u32> for Part1 {
    fn expect_test(&self) -> u32 {
        3
    }

    fn solve(&self, input: &[String]) -> u32 {
        let input = Input::from(input);

        let mut result = 0;

        for lock in &input.locks {
            for key in &input.keys {
                let ones = lock.0.count_ones() + key.0.count_ones();
                if lock.0.bitor(key.0).count_ones() == ones {
                    result += 1;
                }
            }
        }

        result
    }
}

pub struct Part2;

impl Part<i32> for Part2 {
    fn expect_test(&self) -> i32 {
        todo!()
    }

    fn solve(&self, input: &[String]) -> i32 {
        todo!()
    }
}

#[derive(Debug)]
struct Schematic(u32);

impl From<&[String]> for Schematic {
    fn from(value: &[String]) -> Self {
        let identifier = value[1..value.len() - 1]
            .iter()
            .flat_map(|s| s.chars())
            .enumerate()
            .map(|(i, c)| if c == '#' { 1_u32.shl(i) } else { 0 })
            .reduce(|a, b| a.bitor(b))
            .unwrap();

        Self(identifier)
    }
}

struct Input {
    locks: Vec<Schematic>,
    keys: Vec<Schematic>,
}

impl From<&[String]> for Input {
    fn from(value: &[String]) -> Self {
        let mut locks = vec![];
        let mut keys = vec![];

        for x in value.split(|e| e.is_empty()).filter(|s| !s.is_empty()) {
            if x[0].contains('#') {
                locks.push(x.into());
            } else {
                keys.push(x.into());
            }
        }

        Self { locks, keys }
    }
}
