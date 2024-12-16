use crate::harness::Day;
use crate::harness::Part;
use std::collections::{HashMap, HashSet};
use std::ops::{Add, AddAssign, Mul, Neg, Sub};

pub fn day16() -> Day<i32, i32> {
    Day::new(16, Box::new(Part1 {}), Box::new(Part2 {}))
}

pub struct Part1;

impl Part<i32> for Part1 {
    fn expect_test(&self) -> i32 {
        7036
    }

    fn solve(&self, input: &[String]) -> i32 {
        let input = Input::from(input);

        best_cost(&input).unwrap().0
    }
}

pub struct Part2;

impl Part<i32> for Part2 {
    fn expect_test(&self) -> i32 {
        45
    }

    fn solve(&self, input: &[String]) -> i32 {
        let input = Input::from(input);

        let (_, end_node, came_from) = best_cost(&input).unwrap();

        let mut open = vec![end_node];
        let mut closed = HashSet::new();

        while let Some(current) = open.pop() {
            if closed.insert(current) {
                if let Some(neighbours) = came_from.get(&current) {
                    for x in neighbours {
                        open.push(*x);
                    }
                }
            }
        }

        let closed = closed
            .into_iter()
            .map(|e| e.position)
            .collect::<HashSet<_>>();

        closed.len() as i32
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
struct Node {
    position: Vec2,
    direction: Vec2,
}

impl Node {
    fn new(position: Vec2, direction: Vec2) -> Self {
        Self {
            position,
            direction,
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
    map: Vec<Vec<Tile>>,
    start_position: Vec2,
    start_direction: Vec2,
    end_position: Vec2,
}

impl Input {
    fn get(&self, p: Vec2) -> Tile {
        self.map[p.y as usize][p.x as usize]
    }
}

impl From<&[String]> for Input {
    fn from(value: &[String]) -> Self {
        let mut start_position = None;
        let mut end_position = None;

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
                            start_position = Some(v(x as i32, y as i32));
                            Tile::Empty
                        }
                        'E' => {
                            end_position = Some(v(x as i32, y as i32));
                            Tile::Empty
                        }
                        _ => panic!(),
                    })
                    .collect()
            })
            .collect();

        Self {
            map,
            start_position: start_position.unwrap(),
            start_direction: Vec2::EAST,
            end_position: end_position.unwrap(),
        }
    }
}

fn best_cost(input: &Input) -> Option<(i32, Node, HashMap<Node, Vec<Node>>)> {
    let start = Node::new(input.start_position, input.start_direction);
    let mut open_set = HashSet::from([start]);
    let mut came_from = HashMap::<Node, Vec<Node>>::new();

    let h = |node: Node| node.position.manhattan_dist(input.end_position) as i32;

    let mut g_score = HashMap::new();
    g_score.insert(start, 0);

    let mut f_score = HashMap::new();
    f_score.insert(start, h(start));

    while !open_set.is_empty() {
        // this is very expensive, as in 80% of the runtime
        let current = *open_set
            .iter()
            .min_by_key(|e| f_score.get(e).cloned().unwrap_or(i32::MAX))
            .unwrap();

        if current.position == input.end_position {
            return g_score
                .iter()
                .find(|(&node, _)| node.position == input.end_position)
                .map(|(&node, &score)| (score, node, came_from));
        }

        open_set.remove(&current);

        Vec2::CARDINAL_DIRECTIONS
            .iter()
            .filter(|&&direction| matches!(input.get(current.position + direction), Tile::Empty))
            .map(|&direction| {
                (
                    Node::new(current.position + direction, direction),
                    if direction == current.direction {
                        1
                    } else {
                        1001
                    },
                )
            })
            .for_each(|(neighbour, d)| {
                let tentative_g_score = g_score.get(&current).cloned().unwrap_or(i32::MAX) + d;

                if tentative_g_score <= g_score.get(&neighbour).cloned().unwrap_or(i32::MAX) {
                    came_from.entry(neighbour).or_default().push(current);
                    g_score.insert(neighbour, tentative_g_score);
                    f_score.insert(neighbour, tentative_g_score + h(neighbour));

                    open_set.insert(neighbour);
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
