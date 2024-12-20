use crate::harness::Day;
use crate::harness::Part;
use std::cmp::max;
use std::cmp::min;
use std::collections::{HashMap, HashSet};
use std::ops::{Add, AddAssign, Mul, Neg, Sub};
use std::time::Instant;

pub fn day20() -> Day<i32, i32> {
    Day::new(20, Box::new(Part1 {}), Box::new(Part2 {}))
}

pub struct Part1;

impl Part<i32> for Part1 {
    fn expect_test(&self) -> i32 {
        0
    }

    fn solve(&self, input: &[String]) -> i32 {
        let input = Input::from(input);

        let dist = input.dijkstra();

        let get = |v: Vec2| {
            dist.get(v.y as usize)
                .and_then(|e| e.get(v.x as usize))
                .filter(|&&e| e < i32::MAX / 4)
                .cloned()
        };

        let mut result = 0;

        let directions = vec![Vec2::EAST, Vec2::SOUTH];

        for y in 1..input.height() - 1 {
            for x in 1..input.width() - 1 {
                let current = v(x as i32, y as i32);
                if let Some(current_dist) = get(current) {
                    for &direction in &directions {
                        let other = current + direction * 2;
                        if let Some(other_dist) = get(other) {
                            let diff = (current_dist.abs_diff(other_dist) as i32) - 2;

                            if diff >= 100 {
                                result += 1;
                            }
                        }
                    }
                }
            }
        }

        result
    }
}

pub struct Part2;

impl Part<i32> for Part2 {
    fn expect_test(&self) -> i32 {
        0
    }

    fn solve(&self, input: &[String]) -> i32 {
        let input = Input::from(input);

        let dist = input.dijkstra();

        let get = |v: Vec2| {
            dist.get(v.y as usize)
                .and_then(|e| e.get(v.x as usize))
                .filter(|&&e| e < i32::MAX / 4)
                .cloned()
        };

        let mut result = 0;

        for y1 in 1..input.height() - 1 {
            for x1 in 1..input.width() - 1 {
                for y2 in max(1, y1 - 20)..min(input.height() - 1, y1 + 20) {
                    for x2 in max(1, x1 - 20)..min(input.width() - 1, x1 + 20) {
                        let current = v(x1 as i32, y1 as i32);
                        if let Some(current_dist) = get(current) {
                            let other = v(x2 as i32, y2 as i32);
                            if let Some(other_dist) = get(other) {
                                let dist = current.manhattan_dist(other) as i32;
                                let diff = current_dist.abs_diff(other_dist) as i32 - dist;

                                if diff >= 100 {
                                    result += 1;
                                }
                            }
                        }
                    }
                }
            }
        }

        result / 2
    }
}

#[derive(Debug)]
enum Tile {
    Empty,
    Wall,
}

#[derive(Debug)]
struct Input {
    map: Vec<Vec<Tile>>,
    start: Vec2,
    end: Vec2,
}

impl Input {
    fn width(&self) -> usize {
        self.map[0].len()
    }
    fn height(&self) -> usize {
        self.map.len()
    }

    fn dijkstra(&self) -> Vec<Vec<i32>> {
        let mut dist = vec![vec![i32::MAX / 2; self.width()]; self.height()];
        dist[self.end.y as usize][self.end.x as usize] = 0;

        let mut open = HashSet::from([self.end]);
        let mut closed = HashSet::new();

        while let Some(&u) = open.iter().min_by_key(|e| dist[e.y as usize][e.x as usize]) {
            open.remove(&u);
            if !closed.insert(u) {
                continue;
            }

            for direction in Vec2::CARDINAL_DIRECTIONS {
                let v = u + direction;

                if let Tile::Wall = self.map[v.y as usize][v.x as usize] {
                    continue;
                }

                let alt = dist[u.y as usize][u.x as usize] + 1;

                if alt < dist[v.y as usize][v.x as usize] && !closed.contains(&v) {
                    dist[v.y as usize][v.x as usize] = alt;
                    open.insert(v);
                }
            }
        }

        dist
    }
}

impl From<&[String]> for Input {
    fn from(value: &[String]) -> Self {
        let mut start = None;
        let mut end = None;

        let map = value
            .iter()
            .filter(|e| !e.is_empty())
            .enumerate()
            .map(|(y, s)| {
                s.chars()
                    .enumerate()
                    .map(|(x, c)| match c {
                        '.' => Tile::Empty,
                        '#' => Tile::Wall,
                        'S' => {
                            start = Some(v(x as i32, y as i32));
                            Tile::Empty
                        }
                        'E' => {
                            end = Some(v(x as i32, y as i32));
                            Tile::Empty
                        }
                        _ => panic!(),
                    })
                    .collect()
            })
            .collect();

        Self {
            map,
            start: start.unwrap(),
            end: end.unwrap(),
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

    fn manhattan_dist(&self, rhs: Vec2) -> u32 {
        self.x.abs_diff(rhs.x) + self.y.abs_diff(rhs.y)
    }
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
