use crate::harness::Day;
use crate::harness::Part;
use std::mem::swap;
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

        let mut robot_position = input
            .map
            .iter()
            .enumerate()
            .find_map(|(y, vec)| {
                vec.iter()
                    .enumerate()
                    .find(|(_, e)| matches!(e, Tile::Robot))
                    .map(|(x, _)| v(x as i32, y as i32))
            })
            .unwrap();

        for i in 0..input.instructions.len() {
            let instruction = input.instructions[i];

            if input.moove(robot_position, instruction) {
                robot_position += instruction;
            }

        }

        let mut result = 0;

        for y in 0..input.map.len() {
            for x in 0..input.map[0].len() {
                let tile = input.map[y][x];
                let char = match tile {
                    Tile::Empty => '.',
                    Tile::Wall => '#',
                    Tile::Robot => '@',
                    Tile::Box => 'O',
                };

                if matches!(tile, Tile::Box) {
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
        todo!()
    }

    fn solve(&self, input: &[String]) -> u32 {
        todo!()
    }
}

#[derive(Debug, Copy, Clone)]
enum Tile {
    Empty,
    Robot,
    Box,
    Wall,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '#' => Tile::Wall,
            '.' => Tile::Empty,
            '@' => Tile::Robot,
            'O' => Tile::Box,
            _ => panic!(),
        }
    }
}

#[derive(Debug)]
struct Input {
    map: Vec<Vec<Tile>>,
    instructions: Vec<Vec2>,
}

impl From<&[String]> for Input {
    fn from(value: &[String]) -> Self {
        let value = value.split(|e| e.is_empty()).collect::<Vec<_>>();

        let map = value[0]
            .iter()
            .map(|e| e.chars().map(Tile::from).collect())
            .collect();

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

        Self { map, instructions }
    }
}

impl Input {
    fn get(&self, position: Vec2) -> Tile {
        self.map[position.y as usize][position.x as usize]
    }

    fn moove(&mut self, position: Vec2, direction: Vec2) -> bool {
        let position_tile = self.get(position);
        if matches!(position_tile, Tile::Empty) {
            return true;
        }

        if matches!(position_tile, Tile::Wall) {
            return false;
        }


        let next = position + direction;
        self.moove(next, direction);

        if matches!(self.get(next), Tile::Empty) {
            let previous = self.map[position.y as usize][position.x as usize];
            self.map[position.y as usize][position.x as usize] = Tile::Empty;
            self.map[next.y as usize][next.x as usize] = previous;

            true
        } else {
            false
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
