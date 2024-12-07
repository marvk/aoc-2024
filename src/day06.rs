use crate::harness::Day;
use crate::harness::Part;
use std::collections::HashSet;
use std::ops::{Add, Mul, Neg};

pub struct Part1;

pub fn day06() -> Day<i32, i32> {
    Day::new(6, Box::new(Part1 {}), Box::new(Part2 {}))
}

impl Part<i32> for Part1 {
    fn expect_test(&self) -> i32 {
        41
    }

    fn solve(&self, input: &[String]) -> i32 {
        let map = Map::from(input);

        let visited = do_the_thing(&map);

        visited
            .0
            .iter()
            .map(|(e, _)| *e)
            .collect::<HashSet<_>>()
            .len() as i32
    }
}

pub struct Part2;

impl Part<i32> for Part2 {
    fn expect_test(&self) -> i32 {
        6
    }

    fn solve(&self, input: &[String]) -> i32 {
        let mut map = Map::from(input);

        let mut result = 0;

        let set = do_the_thing(&map)
            .0
            .iter()
            .map(|(e, _)| *e)
            .collect::<HashSet<_>>();

        for vec in set {
            if vec == map.start_position {
                continue;
            }

            let x = vec.x as usize;
            let y = vec.y as usize;

            let saved = map.raw[y][x];

            if matches!(saved, Tile::Obstacle) {
                continue;
            }

            map.raw[y][x] = Tile::Obstacle;

            if let ExitStatus::Looped = do_the_thing2(&map) {
                result += 1;
            }

            map.raw[y][x] = saved;
        }

        result
    }
}

#[derive(Copy, Clone, Debug)]
enum Tile {
    Empty,
    Obstacle,
    Robot,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '^' => Tile::Robot,
            '.' => Tile::Empty,
            '#' => Tile::Obstacle,
            _ => panic!(),
        }
    }
}

#[derive(Debug, Clone)]
struct Map {
    raw: Vec<Vec<Tile>>,
    start_position: Vec2,
}

impl Map {
    fn get(&self, v: Vec2) -> Option<Tile> {
        let option = self.raw.get(v.y as usize).and_then(|e| e.get(v.x as usize));

        match option {
            Some(Tile::Robot) => Some(Tile::Empty),
            Some(e) => Some(*e),
            _ => None,
        }
    }
}

impl From<&[String]> for Map {
    fn from(value: &[String]) -> Self {
        let raw: Vec<Vec<_>> = value
            .iter()
            .filter(|e| !e.is_empty())
            .map(|s| s.chars().map(Tile::from).collect())
            .collect();

        let start_position = raw
            .iter()
            .enumerate()
            .map(|(y, vec)| {
                vec.iter()
                    .enumerate()
                    .find(|(_, e)| matches!(e, Tile::Robot))
                    .map(|(x, _)| v(x as i32, y as i32))
            })
            .find(|e| e.is_some())
            .flatten()
            .unwrap();

        Self {
            raw,
            start_position,
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
    pub const NORTH: Self = v(0, -1);
    pub const EAST: Self = v(1, 0);
    pub const SOUTH: Self = v(0, 1);
    pub const WEST: Self = v(-1, 0);

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

fn do_the_thing(map: &Map) -> (HashSet<(Vec2, Vec2)>, ExitStatus) {
    let mut visited = HashSet::new();

    let mut current = map.start_position;

    let mut direction = Vec2::NORTH;

    loop {
        let next = current + direction;

        match map.get(next) {
            Some(Tile::Empty) => {
                current = next;

                let x = visited.insert((current, direction));

                if !x {
                    return (visited, ExitStatus::Looped);
                }
            }
            None => return (visited, ExitStatus::Exited),
            _ => {
                direction = match direction {
                    Vec2::NORTH => Vec2::EAST,
                    Vec2::EAST => Vec2::SOUTH,
                    Vec2::SOUTH => Vec2::WEST,
                    Vec2::WEST => Vec2::NORTH,
                    _ => panic!(),
                }
            }
        }
    }
}

fn do_the_thing2(map: &Map) -> ExitStatus {
    let mut current = map.start_position;
    let mut direction = Vec2::NORTH;

    for _ in 0..7000 {
        let next = current + direction;

        match map.get(next) {
            Some(Tile::Empty) => {
                current = next;
            }
            None => return ExitStatus::Exited,
            _ => {
                direction = match direction {
                    Vec2::NORTH => Vec2::EAST,
                    Vec2::EAST => Vec2::SOUTH,
                    Vec2::SOUTH => Vec2::WEST,
                    Vec2::WEST => Vec2::NORTH,
                    _ => panic!(),
                }
            }
        }
    }

    ExitStatus::Looped
}

enum ExitStatus {
    Exited,
    Looped,
}
