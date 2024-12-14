use crate::harness::Day;
use crate::harness::Part;
use image::{Rgb, RgbImage};
use std::collections::HashSet;
use std::ops::{Add, AddAssign, Mul, Neg, Sub};
use std::path::Path;

pub fn day14() -> Day<i32, i32> {
    Day::new(14, Box::new(Part1 {}), Box::new(Part2 {}))
}

pub struct Part1;

impl Part<i32> for Part1 {
    fn expect_test(&self) -> i32 {
        12
    }

    fn solve(&self, input: &[String]) -> i32 {
        let mut input = Input::from(input);

        for robot in &mut input.robots {
            robot.walk_n(input.width, input.height, 100);
        }

        let mut q1 = 0;
        let mut q2 = 0;
        let mut q3 = 0;
        let mut q4 = 0;

        let mid_x = input.width / 2;
        let mid_y = input.height / 2;

        for robot in input.robots {
            match robot.position {
                Vec2 { x, y } if x < mid_x && y < mid_y => q1 += 1,
                Vec2 { x, y } if x < mid_x && y > mid_y => q2 += 1,
                Vec2 { x, y } if x > mid_x && y < mid_y => q3 += 1,
                Vec2 { x, y } if x > mid_x && y > mid_y => q4 += 1,

                _ => (),
            }
        }

        q1 * q2 * q3 * q4
    }
}

pub struct Part2;

impl Part<i32> for Part2 {
    fn expect_test(&self) -> i32 {
        0
    }

    fn solve(&self, input: &[String]) -> i32 {
        let mut input = Input::from(input);

        if input.width < 50 {
            return 0;
        }

        for i in 1.. {
            for robot in &mut input.robots {
                robot.walk_once(input.width, input.height);
            }

            let robots = input
                .robots
                .iter()
                .map(|e| e.position)
                .collect::<HashSet<_>>();

            for robot in &input.robots {
                let mut open = vec![robot.position];
                let mut current_closed = vec![];

                while let Some(current) = open.pop() {
                    if current_closed.contains(&current) {
                        continue;
                    } else {
                        current_closed.push(current);
                    }

                    for &direction in &Vec2::CARDINAL_DIRECTIONS {
                        let next = current + direction;

                        if robots.contains(&next) {
                            open.push(next);
                        }
                    }
                }

                if current_closed.len() > 50 {
                    return i;
                }
            }
        }

        panic!()
    }
}

#[derive(Debug)]
struct Robot {
    position: Vec2,
    velocity: Vec2,
}

impl From<&str> for Robot {
    fn from(value: &str) -> Self {
        let mut nums = value
            .split(" ")
            .flat_map(|e| e.split("=").skip(1))
            .flat_map(|e| e.split(","))
            .map(|e| e.parse::<i32>().unwrap());

        let position = v(nums.next().unwrap(), nums.next().unwrap());
        let velocity = v(nums.next().unwrap(), nums.next().unwrap());

        Robot { position, velocity }
    }
}

impl Robot {
    fn walk_once(&mut self, width: i32, height: i32) {
        self.position += self.velocity;
        self.position.x = ((self.position.x % width) + width) % width;
        self.position.y = ((self.position.y % height) + height) % height;
    }

    fn walk_n(&mut self, width: i32, height: i32, n: usize) {
        self.position += self.velocity * n as i32;
        self.position.x = ((self.position.x % width) + width) % width;
        self.position.y = ((self.position.y % height) + height) % height;
    }
}

#[derive(Debug)]
struct Input {
    robots: Vec<Robot>,
    width: i32,
    height: i32,
}

impl From<&[String]> for Input {
    fn from(value: &[String]) -> Self {
        let robots = value
            .iter()
            .filter(|e| !e.is_empty())
            .map(|e| Robot::from(e.as_str()))
            .collect::<Vec<_>>();

        let width = robots.iter().map(|e| e.position.x).max().unwrap() + 1;
        let height = robots.iter().map(|e| e.position.y).max().unwrap() + 1;

        Input {
            robots,
            width,
            height,
        }
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

impl AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
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
