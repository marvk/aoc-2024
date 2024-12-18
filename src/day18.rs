use crate::harness::Day;
use crate::harness::Part;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::ops::{Add, AddAssign, Mul, Neg, Sub};
use std::time::Instant;

pub fn day18() -> Day<i32, String> {
    Day::new(18, Box::new(Part1 {}), Box::new(Part2 {}))
}

pub struct Part1;

impl Part<i32> for Part1 {
    fn expect_test(&self) -> i32 {
        22
    }

    fn solve(&self, input: &[String]) -> i32 {
        let input = Input::from(input);

        let start = if input.width < 10 { 12 } else { 1024 };

        search(&input, input.build_grid(start)).unwrap()
    }
}

pub struct Part2;

impl Part<String> for Part2 {
    fn expect_test(&self) -> String {
        "6,1".to_string()
    }

    fn solve(&self, input: &[String]) -> String {
        let input = Input::from(input);

        let start = if input.width < 10 { 12 } else { 1024 };

        let mut min = start + 1;
        let mut max = input.blockers.len() - 1;

        loop {
            let add = (max - min) / 2;
            let current = min + add;

            if search(&input, input.build_grid(current)).is_some() {
                min = current + 1;
            } else {
                max = current;

                if min == max {
                    return format!(
                        "{},{}",
                        input.blockers[current - 1].x,
                        input.blockers[current - 1].y
                    );
                }
            }
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Tile {
    Empty,
    Wall,
}

#[derive(Debug)]
struct Input {
    blockers: Vec<Vec2>,
    width: usize,
    height: usize,
    start: Vec2,
    end: Vec2,
}

impl From<&[String]> for Input {
    fn from(value: &[String]) -> Self {
        let blockers = value
            .iter()
            .filter(|e| !e.is_empty())
            .map(|s| {
                let mut split = s.split(",").map(|e| e.parse().unwrap());
                v(split.next().unwrap(), split.next().unwrap())
            })
            .collect::<Vec<_>>();

        let end = v(
            blockers.iter().map(|e| e.x).max().unwrap(),
            blockers.iter().map(|e| e.y).max().unwrap(),
        );

        Self {
            blockers,
            width: end.x as usize + 1,
            height: end.y as usize + 1,
            start: v(0, 0),
            end,
        }
    }
}

impl Input {
    fn build_grid(&self, max: usize) -> Vec<Vec<Tile>> {
        let mut result = vec![vec![Tile::Empty; self.width]; self.height];

        for blocker in self.blockers.iter().take(max) {
            result[blocker.y as usize][blocker.x as usize] = Tile::Wall;
        }

        result
    }
}

#[derive(PartialEq, Eq)]
struct Node(Vec2, i32);

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.1.cmp(&other.1).reverse()
    }
}

fn search(input: &Input, grid: Vec<Vec<Tile>>) -> Option<i32> {
    let start = input.start;
    let mut open_heap = BinaryHeap::from([Node(start, 0)]);
    let mut closed_set = HashSet::new();

    let h = |vec: Vec2| vec.manhattan_dist(input.end) as i32;

    let mut g_scores = vec![vec![i32::MAX / 2; input.width]; input.height];
    g_scores[start.y as usize][start.x as usize] = 0;

    let mut f_scores = vec![vec![i32::MAX / 2; input.width]; input.height];
    f_scores[start.y as usize][start.x as usize] = h(start);

    while let Some(current) = open_heap.pop() {
        if current.0 == input.end {
            return Some(g_scores[input.end.y as usize][input.end.x as usize]);
        }

        if closed_set.contains(&current.0) {
            continue;
        }

        closed_set.insert(current.0);

        Vec2::CARDINAL_DIRECTIONS
            .iter()
            .map(|&d| current.0 + d)
            .filter(|neighbour| {
                matches!(
                    grid.get(neighbour.y as usize)
                        .and_then(|e| e.get(neighbour.x as usize)),
                    Some(Tile::Empty)
                )
            })
            .for_each(|neighbour| {
                let tentative_g_score = g_scores[current.0.y as usize][current.0.x as usize] + 1;

                if tentative_g_score <= g_scores[neighbour.y as usize][neighbour.x as usize] {
                    g_scores[neighbour.y as usize][neighbour.x as usize] = tentative_g_score;
                    let f_score = tentative_g_score + h(neighbour);
                    f_scores[neighbour.y as usize][neighbour.x as usize] = f_score;

                    open_heap.push(Node(neighbour, f_score));
                }
            });
    }

    None
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
