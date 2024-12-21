use crate::harness::Day;
use crate::harness::Part;
use std::ops::{Add, AddAssign, Mul, Neg, Sub};

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
                for y2 in y1..=y1 + 20 {
                    let y_diff = (y2 - y1) as i32;
                    let x_min = (y_diff - 20) * y_diff.signum();
                    let x_max = 20 - y_diff;
                    for x_diff in x_min..=x_max {
                        let current = v(x1 as i32, y1 as i32);
                        if let Some(current_dist) = get(current) {
                            let other = v(x1 as i32 + x_diff, y2 as i32);
                            if let Some(other_dist) = get(other) {
                                let dist = y_diff + x_diff.abs();
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

        result
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

        let mut next = Some(self.end);
        let mut previous_direction = v(0, 0);

        let mut i = 1;

        while let Some(current) = next.take() {
            for &direction in Vec2::CARDINAL_DIRECTIONS
                .iter()
                .filter(|&&e| e * -1 != previous_direction)
            {
                let current_next = current + direction;

                if let Tile::Wall = self.map[current_next.y as usize][current_next.x as usize] {
                    continue;
                }

                dist[current_next.y as usize][current_next.x as usize] = i;
                next = Some(current_next);
                previous_direction = direction;
                break;
            }

            i += 1;
        }

        dist
    }
}

impl From<&[String]> for Input {
    fn from(value: &[String]) -> Self {
        let mut end = None;

        let map = value
            .iter()
            .filter(|e| !e.is_empty())
            .enumerate()
            .map(|(y, s)| {
                s.chars()
                    .enumerate()
                    .map(|(x, c)| match c {
                        '.' | 'S' => Tile::Empty,
                        '#' => Tile::Wall,
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
