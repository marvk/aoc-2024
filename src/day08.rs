use crate::harness::Day;
use crate::harness::Part;
use std::collections::HashMap;
use std::ops::Sub;
use std::ops::{Add, Mul, Neg};
pub struct Part1;

pub fn day08() -> Day<i32, i32> {
    Day::new(8, Box::new(Part1 {}), Box::new(Part2 {}))
}

impl Part<i32> for Part1 {
    fn expect_test(&self) -> i32 {
        14
    }

    fn solve(&self, input: &[String]) -> i32 {
        let input = Input::from(input);

        solve(input, 1, 2)
    }
}

pub struct Part2;

impl Part<i32> for Part2 {
    fn expect_test(&self) -> i32 {
        34
    }

    fn solve(&self, input: &[String]) -> i32 {
        let input = Input::from(input);

        solve(input, 0, i32::MAX)
    }
}

fn solve(input: Input, min: i32, max: i32) -> i32 {
    let mut result = vec![vec![false; input.width]; input.height];

    for vec in input.map.values() {
        for &a in vec {
            for &b in vec {
                if a == b {
                    continue;
                };

                let diff = b - a;

                for i in min..max {
                    let pos = b + diff * i;

                    if input.in_bounds(pos) {
                        result[pos.y as usize][pos.x as usize] = true
                    } else {
                        break;
                    }
                }
            }
        }
    }

    result
        .into_iter()
        .map(|e| e.into_iter().filter(|&e2| e2).count() as i32)
        .sum()
}

#[derive(Debug)]
struct Input {
    map: HashMap<char, Vec<Vec2>>,
    width: usize,
    height: usize,
}

impl From<&[String]> for Input {
    fn from(value: &[String]) -> Self {
        let value = value.iter().filter(|e| !e.is_empty()).collect::<Vec<_>>();
        let map = value
            .iter()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .filter(|&(_, c)| c != '.')
                    .map(move |(x, c)| (c, v(x as i32, y as i32)))
            })
            .fold(HashMap::new(), |mut acc, (c, vec2)| {
                let vec = acc.entry(c).or_insert(Vec::new());
                vec.push(vec2);
                acc
            });

        Self {
            map,
            width: value[0].len(),
            height: value.len(),
        }
    }
}

impl Input {
    fn in_bounds(&self, vec2: Vec2) -> bool {
        vec2.x >= 0 && vec2.x < (self.width as i32) && vec2.y >= 0 && vec2.y < (self.height as i32)
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

impl Sub<Vec2> for Vec2 {
    type Output = Self;

    fn sub(self, rhs: Vec2) -> Self::Output {
        v(self.x - rhs.x, self.y - rhs.y)
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
