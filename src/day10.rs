use crate::harness::Day;
use crate::harness::Part;
use std::collections::{HashSet, VecDeque};
use std::ops::{Add, Mul, Neg, Sub};
pub struct Part1;

pub fn day10() -> Day<i32, i32> {
    Day::new(10, Box::new(Part1 {}), Box::new(Part2 {}))
}

impl Part<i32> for Part1 {
    fn expect_test(&self) -> i32 {
        36
    }

    fn solve(&self, input: &[String]) -> i32 {
        let input = Input::from(input);

        let mut result = 0;

        for &trailhead in &input.trailheads {
            let mut visited = HashSet::new();
            let mut queue = VecDeque::from(vec![trailhead]);

            while let Some(current) = queue.pop_back() {
                if visited.contains(&current) {
                    continue;
                }

                let current_value = input.get(current).unwrap();

                visited.insert(current);

                for direction in Vec2::CARDINAL_DIRECTIONS {
                    let next = current + direction;

                    if let Some(next_value) = input.get(next) {
                        if !visited.contains(&next) && next_value == current_value + 1 {
                            queue.push_back(next);
                        }
                    }
                }
            }

            result += visited
                .into_iter()
                .filter(|&e| input.get(e).unwrap() == 9)
                .count();
        }

        result as i32
    }
}

pub struct Part2;

impl Part<i32> for Part2 {
    fn expect_test(&self) -> i32 {
        81
    }

    fn solve(&self, input: &[String]) -> i32 {
        let input = Input::from(input);

        let mut result = 0;

        for &trailhead in &input.trailheads {
            let mut queue = VecDeque::from(vec![trailhead]);

            while let Some(current) = queue.pop_back() {
                let current_value = input.get(current).unwrap();

                for &direction in &Vec2::CARDINAL_DIRECTIONS {
                    let next = current + direction;

                    if let Some(next_value) = input.get(next) {
                        if next_value == current_value + 1 {
                            if next_value == 9 {
                                result += 1;
                            } else {
                                queue.push_back(next);
                            }
                        }
                    }
                }
            }
        }

        result
    }
}

#[derive(Debug)]
struct Input {
    map: Vec<Vec<u8>>,
    trailheads: Vec<Vec2>,
}

impl From<&[String]> for Input {
    fn from(value: &[String]) -> Self {
        let mut trailheads = vec![];

        let map = value
            .iter()
            .filter(|e| !e.is_empty())
            .enumerate()
            .map(|(y, s)| {
                s.chars()
                    .enumerate()
                    .map(|(x, c)| {
                        if c == '0' {
                            trailheads.push(v(x as i32, y as i32))
                        }
                        c.to_digit(10).unwrap() as u8
                    })
                    .collect()
            })
            .collect();

        Self { map, trailheads }
    }
}

impl Input {
    fn get(&self, v: Vec2) -> Option<u8> {
        self.map
            .get(v.y as usize)
            .and_then(|e| e.get(v.x as usize))
            .cloned()
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

    pub const NORTH: Self = v(0, -1);
    pub const EAST: Self = v(1, 0);
    pub const SOUTH: Self = v(0, 1);
    pub const WEST: Self = v(-1, 0);

    pub const CARDINAL_DIRECTIONS: [Self; 4] = [Self::NORTH, Self::EAST, Self::SOUTH, Self::WEST];
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
