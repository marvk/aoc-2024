use crate::harness::Day;
use crate::harness::Part;
use std::ops::{BitAnd, BitOr, Shl};

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
                result += key.fits(lock) as u32;
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
struct Schematic(u32);

impl From<&[String]> for Schematic {
    fn from(value: &[String]) -> Self {
        let pattern = value[1..value.len() - 1]
            .iter()
            .flat_map(|s| s.chars())
            .enumerate()
            .map(|(i, c)| ((c == '#') as u32).shl(i))
            .fold(0, |a, b| a.bitor(b));

        Self(pattern)
    }
}

impl Schematic {
    fn fits(&self, rhs: &Schematic) -> bool {
        self.0.bitand(rhs.0) == 0
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

        for e in value.split(|s| s.is_empty()).filter(|e| !e.is_empty()) {
            if e[0].starts_with('#') {
                locks.push(e.into());
            } else {
                keys.push(e.into());
            }
        }

        Self { locks, keys }
    }
}
