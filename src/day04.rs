use crate::harness::Day;
use crate::harness::Part;
use std::ops::{Add, Mul, Neg};
pub struct Part1;

pub fn day04() -> Day<usize, usize> {
    Day::new(4, Box::new(Part1 {}), Box::new(Part2 {}))
}

impl Part<usize> for Part1 {
    fn expect_test(&self) -> usize {
        18
    }

    fn solve(&self, input: &[String]) -> usize {
        let input = Input::from(input);

        let mut count = 0;

        for y in 0..input.height {
            for x in 0..input.width {
                let p = v(x as i32, y as i32);

                for d in &Vec2::DIRECTIONS {
                    let word = [
                        input.get(&p),
                        input.get(&(p + *d * 1)),
                        input.get(&(p + *d * 2)),
                        input.get(&(p + *d * 3)),
                    ]
                    .map(|e| e.map(|c| c.to_string()).unwrap_or("".to_string()))
                    .join("");

                    if word == "XMAS" {
                        count += 1;
                    }
                }
            }
        }

        count
    }
}

pub struct Part2;

impl Part<usize> for Part2 {
    fn expect_test(&self) -> usize {
        9
    }

    fn solve(&self, input: &[String]) -> usize {
        let input = Input::from(input);

        let mut count = 0;

        for y in 0..input.height {
            for x in 0..input.width {
                let p = v(x as i32, y as i32);

                let word = [
                    input.get(&(p + Vec2::NORTH_WEST)),
                    input.get(&(p + Vec2::NORTH_EAST)),
                    input.get(&p),
                    input.get(&(p + Vec2::SOUTH_WEST)),
                    input.get(&(p + Vec2::SOUTH_EAST)),
                ]
                .map(|e| e.map(|c| c.to_string()).unwrap_or("".to_string()))
                .join("");

                if word == "MSAMS" || word == "MMASS" || word == "SMASM" || word == "SSAMM" {
                    count += 1;
                }
            }
        }

        count
    }
}

struct Input<'a> {
    raw: &'a [String],
    width: usize,
    height: usize,
}

impl<'a> From<&'a [String]> for Input<'a> {
    fn from(value: &'a [String]) -> Self {
        Input {
            width: value[0].len(),
            height: value.len(),
            raw: value,
        }
    }
}

impl<'a> Input<'a> {
    fn get(&self, p: &Vec2) -> Option<char> {
        self.raw
            .get(p.y as usize)
            .map(|s| s.chars().nth(p.x as usize))
            .flatten()
    }
}

const fn v(x: i32, y: i32) -> Vec2 {
    Vec2::new(x, y)
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct Vec2 {
    x: i32,
    y: i32,
}

impl Vec2 {
    pub const NORTH: Self = v(0, -1);
    pub const NORTH_EAST: Self = v(1, -1);
    pub const EAST: Self = v(1, 0);
    pub const SOUTH_EAST: Self = v(1, 1);
    pub const SOUTH: Self = v(0, 1);
    pub const SOUTH_WEST: Self = v(-1, 1);
    pub const WEST: Self = v(-1, 0);
    pub const NORTH_WEST: Self = v(-1, -1);

    pub const DIRECTIONS: [Self; 8] = [
        Self::NORTH,
        Self::NORTH_EAST,
        Self::EAST,
        Self::SOUTH_EAST,
        Self::SOUTH,
        Self::SOUTH_WEST,
        Self::WEST,
        Self::NORTH_WEST,
    ];

    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl Add<Vec2> for Vec2 {
    type Output = Self;

    fn add(self, rhs: Vec2) -> Self::Output {
        v(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Neg for Vec2 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        v(-self.x, -self.y)
    }
}

impl Mul<i32> for Vec2 {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        v(self.x * rhs, self.y * rhs)
    }
}
