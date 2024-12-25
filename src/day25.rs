use crate::harness::Day;
use crate::harness::Part;
use std::ops::{BitOr, Shl};

pub fn day25() -> Day<u32, ()> {
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
                if key.fits(lock) {
                    result += 1;
                }
            }
        }

        result
    }
}

pub struct Part2;

impl Part<()> for Part2 {
    fn expect_test(&self) {}

    fn solve(&self, _input: &[String]) {}
}

#[derive(Debug)]
struct Schematic {
    pattern: u32,
    ones: u32,
}

impl From<&[String]> for Schematic {
    fn from(value: &[String]) -> Self {
        let pattern = value[1..value.len() - 1]
            .iter()
            .flat_map(|s| s.chars())
            .enumerate()
            .map(|(i, c)| if c == '#' { 1_u32.shl(i) } else { 0 })
            .reduce(|a, b| a.bitor(b))
            .unwrap();

        let ones = pattern.count_ones();
        
        Self {
            pattern,
            ones,
        }
    }
}

impl Schematic {
    fn fits(&self, rhs: &Schematic) -> bool {
        self.pattern.bitor(rhs.pattern).count_ones() == self.ones + rhs.ones
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
