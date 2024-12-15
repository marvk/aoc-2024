use crate::harness::Day;
use crate::harness::Part;
use std::collections::HashMap;
use std::ops::{Add, AddAssign, Mul, Neg, Sub};

pub fn day15() -> Day<u32, u32> {
    Day::new(15, Box::new(Part1 {}), Box::new(Part2 {}))
}

pub struct Part1;

impl Part<u32> for Part1 {
    fn expect_test(&self) -> u32 {
        10092
    }

    fn solve(&self, input: &[String]) -> u32 {
        let mut input = Input::from(input);

        let mut robot_position = input.robot_position();

        for i in 0..input.instructions.len() {
            let instruction = input.instructions[i];

            if input.move1(robot_position, instruction) {
                robot_position += instruction;
            }
        }

        let mut result = 0;

        for y in 0..input.height {
            for x in 0..input.width {
                if let Some(Tile::Box) = input.get(v(x as i32, y as i32)) {
                    result += 100 * y + x;
                }
            }
        }

        result as u32
    }
}

pub struct Part2;

impl Part<u32> for Part2 {
    fn expect_test(&self) -> u32 {
        9021
    }

    fn solve(&self, input: &[String]) -> u32 {
        let mut input = Input::from(input);

        input.scale_x(2);

        let mut robot_position = input.robot_position();

        for i in 0..input.instructions.len() {
            let instruction = input.instructions[i];

            if input.move2(robot_position, instruction) {
                robot_position += instruction;
            }
        }

        input.result()
    }
}

#[derive(Debug, Copy, Clone)]
enum Tile {
    Robot,
    Box,
    Wall,
}

impl TryFrom<char> for Tile {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '#' => Ok(Tile::Wall),
            '@' => Ok(Tile::Robot),
            'O' => Ok(Tile::Box),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
struct Input {
    map: HashMap<Vec2, Tile>,
    width: usize,
    height: usize,
    instructions: Vec<Vec2>,
}

impl From<&[String]> for Input {
    fn from(value: &[String]) -> Self {
        let value = value.split(|e| e.is_empty()).collect::<Vec<_>>();

        let map_raw = value[0];

        let mut map = HashMap::new();

        for y in 0..map_raw.len() {
            for x in 0..map_raw[0].len() {
                if let Ok(tile) = Tile::try_from(map_raw[y].chars().nth(x).unwrap()) {
                    map.insert(v(x as i32, y as i32), tile);
                }
            }
        }

        let instructions = value[1]
            .iter()
            .flat_map(|e| {
                e.chars().map(|c| match c {
                    '^' => Vec2::NORTH,
                    '>' => Vec2::EAST,
                    'v' => Vec2::SOUTH,
                    '<' => Vec2::WEST,
                    _ => panic!(),
                })
            })
            .collect();

        Self {
            map,
            width: map_raw[0].len(),
            height: map_raw.len(),
            instructions,
        }
    }
}

impl Input {
    fn get(&self, position: Vec2) -> Option<Tile> {
        self.map.get(&position).cloned()
    }

    fn robot_position(&self) -> Vec2 {
        *self
            .map
            .iter()
            .find_map(|(position, tile)| {
                if matches!(tile, Tile::Robot) {
                    Some(position)
                } else {
                    None
                }
            })
            .unwrap()
    }

    fn move1(&mut self, position: Vec2, direction: Vec2) -> bool {
        let position_tile = self.get(position);
        if position_tile.is_none() {
            return true;
        }

        if matches!(position_tile, Some(Tile::Wall)) {
            return false;
        }

        let next = position + direction;
        self.move1(next, direction);

        if self.get(next).is_none() {
            let previous = *self.map.get(&position).unwrap();
            self.map.remove(&position);
            self.map.insert(next, previous);

            true
        } else {
            false
        }
    }

    fn move2(&mut self, position: Vec2, direction: Vec2) -> bool {
        let mut moves = self.move2_rec(position, direction, 0);

        if moves.iter().all(|e| e.is_some()) {
            moves.sort_by_key(|e| e.unwrap().2);

            moves
                .into_iter()
                .rev()
                .flatten()
                .for_each(|(a, b, _)| self.move_tile(a, b));

            true
        } else {
            false
        }
    }

    fn move2_rec(
        &mut self,
        position: Vec2,
        direction: Vec2,
        depth: usize,
    ) -> Vec<Option<(Vec2, Vec2, usize)>> {
        let neighbours = match self.get(position) {
            None => return vec![],
            Some(Tile::Wall) => return vec![None],
            Some(Tile::Robot) => match direction {
                Vec2::NORTH => vec![Vec2::NORTH, Vec2::NORTH_WEST],
                Vec2::EAST => vec![Vec2::EAST],
                Vec2::SOUTH => vec![Vec2::SOUTH, Vec2::SOUTH_WEST],
                Vec2::WEST => vec![Vec2::WEST * 2],
                _ => panic!(),
            },
            Some(Tile::Box) => match direction {
                Vec2::NORTH => vec![Vec2::NORTH, Vec2::NORTH_WEST, Vec2::NORTH_EAST],
                Vec2::EAST => vec![Vec2::EAST * 2],
                Vec2::SOUTH => vec![Vec2::SOUTH, Vec2::SOUTH_WEST, Vec2::SOUTH_EAST],
                Vec2::WEST => vec![Vec2::WEST * 2],
                _ => panic!(),
            },
        };

        let mut moves = neighbours
            .iter()
            .flat_map(|&e| self.move2_rec(position + e, direction, depth + 1))
            .collect::<Vec<_>>();

        if moves.iter().all(|e| e.is_some()) {
            moves.push(Some((position, position + direction, depth)));
        }

        moves
    }

    fn move_tile(&mut self, from: Vec2, to: Vec2) {
        if let Some(previous) = self.map.remove(&from) {
            self.map.insert(to, previous);
        }
    }

    fn result(&self) -> u32 {
        self.map
            .iter()
            .filter(|(_, tile)| matches!(tile, Tile::Box))
            .map(|(position, _)| position.y * 100 + position.x)
            .sum::<i32>() as u32
    }

    fn scale_x(&mut self, factor: usize) {
        self.width *= factor;

        self.map = self
            .map
            .iter()
            .map(|(&position, &tile)| {
                let mut next_position = position;
                next_position.x *= factor as i32;
                (next_position, tile)
            })
            .collect();
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
    pub const NORTH_EAST: Self = v(1, -1);
    pub const EAST: Self = v(1, 0);
    pub const SOUTH_EAST: Self = v(1, 1);
    pub const SOUTH: Self = v(0, 1);
    pub const SOUTH_WEST: Self = v(-1, 1);
    pub const WEST: Self = v(-1, 0);
    pub const NORTH_WEST: Self = v(-1, -1);
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
