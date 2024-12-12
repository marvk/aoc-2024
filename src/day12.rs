use crate::harness::Day;
use crate::harness::Part;
use std::collections::HashSet;
use std::ops::{Add, Mul, Neg, Sub};

pub fn day12() -> Day<u32, u32> {
    Day::new(12, Box::new(Part1 {}), Box::new(Part2 {}))
}

pub struct Part1;

impl Part<u32> for Part1 {
    fn expect_test(&self) -> u32 {
        1930
    }

    fn solve(&self, input: &[String]) -> u32 {
        let input = Input::from(input);

        let regions = input.build_regions();

        regions
            .into_iter()
            .map(|r| {
                r.nodes
                    .iter()
                    .map(|&e| {
                        Vec2::CARDINAL_DIRECTIONS
                            .map(|d| d + e)
                            .iter()
                            .filter(|&&e| input.get(e).map(|p| p != r.plant).unwrap_or(true))
                            .count()
                    })
                    .sum::<usize>()
                    * r.nodes.len()
            })
            .sum::<usize>() as u32
    }
}

pub struct Part2;

impl Part<u32> for Part2 {
    fn expect_test(&self) -> u32 {
        1206
    }

    fn solve(&self, input: &[String]) -> u32 {
        let input = Input::from(input);

        let regions = input.build_regions();

        let mut result = 0;

        for region in regions {
            let mut visited = HashSet::new();

            let mut fences = 0;

            for &position in &region.nodes {
                for &direction in &Vec2::CARDINAL_DIRECTIONS {
                    let neighbour = position + direction;

                    if Some(region.plant) != input.get(neighbour)
                        && visited.insert((position, direction))
                    {
                        fences += 1;

                        for perpendicular in direction.perpendicular_directions() {
                            for i in 1.. {
                                let next_perpendicular = position + perpendicular * i;
                                let perpendicular_neighbour = next_perpendicular + direction;

                                if Some(region.plant) == input.get(next_perpendicular)
                                    && Some(region.plant) != input.get(perpendicular_neighbour)
                                {
                                    visited.insert((next_perpendicular, direction));
                                } else {
                                    break;
                                }
                            }
                        }
                    }
                }
            }

            result += fences * region.nodes.len();
        }

        result as u32
    }
}

struct Input {
    raw: Vec<Vec<char>>,
}

impl From<&[String]> for Input {
    fn from(value: &[String]) -> Self {
        let raw = value
            .iter()
            .filter(|e| !e.is_empty())
            .map(|e| e.chars().collect())
            .collect();

        Input { raw }
    }
}

impl Input {
    fn width(&self) -> usize {
        self.raw[0].len()
    }

    fn height(&self) -> usize {
        self.raw.len()
    }
    
    fn get(&self, v: Vec2) -> Option<char> {
        self.raw
            .get(v.y as usize)
            .and_then(|e| e.get(v.x as usize))
            .copied()
    }

    fn flood_fill(&self, v: Vec2) -> Option<Region> {
        let plant = self.get(v)?;

        let mut open = vec![v];
        let mut closed = HashSet::from([v]);

        while let Some(next) = open.pop() {
            for direction in Vec2::CARDINAL_DIRECTIONS {
                let next = next + direction;

                if let Some(next_plant) = self.get(next) {
                    if next_plant == plant && closed.insert(next) {
                        open.push(next);
                        closed.insert(next);
                    }
                }
            }
        }

        Some(Region {
            plant,
            nodes: closed.into_iter().collect(),
        })
    }

    fn build_regions(&self) -> Vec<Region> {
        let mut visited = HashSet::with_capacity(self.width() * self.height());
        let mut regions = vec![];

        for y in 0..self.height() {
            for x in 0..self.width() {
                let current = v(x as i32, y as i32);

                if visited.insert(current) {
                    if let Some(region) = self.flood_fill(current) {
                        region.nodes.iter().for_each(|&e| {
                            visited.insert(e);
                        });
                        regions.push(region);
                    }
                }
            }
        }
        regions
    }
}

#[derive(Debug)]
struct Region {
    plant: char,
    nodes: Vec<Vec2>,
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

impl Vec2 {
    fn perpendicular_directions(&self) -> Vec<Vec2> {
        match self {
            &Vec2::NORTH => vec![Vec2::EAST, Vec2::WEST],
            &Vec2::SOUTH => vec![Vec2::EAST, Vec2::WEST],
            &Vec2::EAST => vec![Vec2::NORTH, Vec2::SOUTH],
            &Vec2::WEST => vec![Vec2::NORTH, Vec2::SOUTH],
            _ => panic!(),
        }
    }
}
