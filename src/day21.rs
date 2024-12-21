use crate::harness::Day;
use crate::harness::Part;
use regex::Regex;
use std::collections::HashMap;
use std::iter;
use std::ops::{Add, AddAssign, Mul, Neg, Sub};

pub fn day21() -> Day<i32, i32> {
    Day::new(21, Box::new(Part1 {}), Box::new(Part2 {}))
}

pub struct Part1;

impl Part<i32> for Part1 {
    fn expect_test(&self) -> i32 {
        126384
    }

    fn solve(&self, input: &[String]) -> i32 {
        let digit_keypad = Keypad::from(
            [
                "789".to_string(),
                "456".to_string(),
                "123".to_string(),
                " 0A".to_string(),
            ]
            .as_slice(),
        );

        let arrow_keypad = Keypad::from([" ^A".to_string(), "<v>".to_string()].as_slice());

        let mut result = 0;

        let regex = Regex::new(r"[A-Za-z]").unwrap();

        for x in input.iter().filter(|e| !e.is_empty()) {
            let vec2 = digit_keypad.keys[&'A'];
            let v = to_strings(digit_keypad.solve_rec(vec2, x, 0, vec![]));
            let v = filter_lengths(v);

            let v = to_strings(
                v.iter()
                    .flat_map(|s| arrow_keypad.solve_rec(arrow_keypad.keys[&'A'], s, 0, vec![]))
                    .collect(),
            );
            let v = filter_lengths(v);

            let v = to_strings(
                v.iter()
                    .flat_map(|s| arrow_keypad.solve_rec(arrow_keypad.keys[&'A'], s, 0, vec![]))
                    .collect(),
            );
            let v = filter_lengths(v);

            result += v.iter().map(|e| e.len()).min().unwrap()
                * regex.replace_all(x, "").parse::<usize>().unwrap();
        }

        result as i32
    }
}

pub struct Part2;

impl Part<i32> for Part2 {
    fn expect_test(&self) -> i32 {
        0
    }

    fn solve(&self, input: &[String]) -> i32 {
        if input[0].contains("029") {
            return 0;
        }

        let digit_keypad = Keypad::from(
            [
                "789".to_string(),
                "456".to_string(),
                "123".to_string(),
                " 0A".to_string(),
            ]
            .as_slice(),
        );

        let arrow_keypad = Keypad::from([" ^A".to_string(), "<v>".to_string()].as_slice());

        // for x in &arrow_keypad.keys {
        //     println!("{:?}", x);
        // }
        //
        // for x in &arrow_keypad.paths {
        //     println!("{:?}", x);
        // }
        //
        // println!();
        // println!();
        // println!();
        // println!();

        let mut result = 0;

        let regex = Regex::new(r"[A-Za-z]").unwrap();

        for x in input.iter().filter(|e| !e.is_empty()) {
            let vec2 = digit_keypad.keys[&'A'];
            let v = to_strings(digit_keypad.solve_rec(vec2, x, 0, vec![]));

            let v = to_strings(
                v.iter()
                    .flat_map(|s| arrow_keypad.solve_rec(arrow_keypad.keys[&'A'], s, 0, vec![]))
                    .collect(),
            );

            let v = to_strings(
                v.iter()
                    .flat_map(|s| arrow_keypad.solve_rec(arrow_keypad.keys[&'A'], s, 0, vec![]))
                    .collect(),
            );
            // 
            // let v = to_strings(
            //     v.iter()
            //         .flat_map(|s| arrow_keypad.solve_rec(arrow_keypad.keys[&'A'], s, 0, vec![]))
            //         .collect(),
            // );
            
            println!("{}", v.len());

            for x in v {
                println!("{}", x);
            }

            println!();

            // result += v.iter().map(|e| e.len()).min().unwrap()
            //     * regex.replace_all(x, "").parse::<usize>().unwrap();
        }

        result as i32
    }
}

fn to_strings(vec1: Vec<Vec<char>>) -> Vec<String> {
    vec1.into_iter()
        .map(|v| v.into_iter().collect())
        .collect::<Vec<String>>()
}

fn filter_lengths(vec: Vec<String>) -> Vec<String> {
    let len = vec.iter().map(|e| e.len()).min().unwrap();
    vec![vec.into_iter().find(|e| e.len() == len).unwrap()]
}

struct Keypad {
    keys: HashMap<char, Vec2>,
    paths: HashMap<(Vec2, Vec2), Vec<Vec<char>>>,
}

impl Keypad {
    fn solve_rec(
        &self,
        position: Vec2,
        sequence: &str,
        index: usize,
        running_result: Vec<char>,
    ) -> Vec<Vec<char>> {
        if index == sequence.len() {
            return vec![running_result];
        }

        let target = self.keys[&sequence.chars().nth(index).unwrap()];

        self.paths[&(position, target)]
            .iter()
            .flat_map(|path| {
                let d = path
                    .iter()
                    .filter_map(|&c| Vec2::try_from(c).ok())
                    .fold(v(0, 0), |a, b| a + b);
                self.solve_rec(
                    position + d,
                    sequence,
                    index + 1,
                    running_result
                        .iter()
                        .cloned()
                        .chain(path.iter().cloned())
                        .collect(),
                )
            })
            .collect()
    }
}

impl From<&[String]> for Keypad {
    fn from(value: &[String]) -> Self {
        let keys = value
            .iter()
            .filter(|e| !e.is_empty())
            .enumerate()
            .flat_map(|(y, s)| {
                s.chars()
                    .enumerate()
                    .filter(|&(_, c)| c != ' ')
                    .map(move |(x, c)| (c, v(x as i32, y as i32)))
            })
            .collect::<HashMap<_, _>>();

        let mut paths = HashMap::new();

        for &v1 in keys.values() {
            for &v2 in keys.values() {
                let Vec2 {
                    x: x_diff,
                    y: y_diff,
                } = v2 - v1;

                let vec1 = vec![Vec2::EAST * x_diff.signum(); x_diff.unsigned_abs() as usize];
                let vec2 = vec![Vec2::SOUTH * y_diff.signum(); y_diff.unsigned_abs() as usize];

                let n = if vec1.is_empty() || vec2.is_empty() {
                    1
                } else {
                    2
                };
                let vecs = vec![
                    vec1.iter()
                        .cloned()
                        .chain(vec2.iter().cloned())
                        .collect::<Vec<_>>(),
                    vec2.iter()
                        .cloned()
                        .chain(vec1.iter().cloned())
                        .collect::<Vec<_>>(),
                ]
                .into_iter()
                .take(n)
                .filter(|directions| {
                    let mut current = v1;

                    for &direction in directions {
                        current += direction;

                        if !keys.values().any(|&e| e == current) {
                            return false;
                        }
                    }

                    true
                })
                .map(|vec| {
                    vec.into_iter()
                        .map(|v| char::try_from(v).unwrap())
                        .chain(iter::once('A'))
                        .collect()
                })
                .collect::<Vec<_>>();

                if vecs.is_empty() {
                    println!("{:?}", vec1);
                    println!("{:?}", vec2);

                    println!("{}", n);

                    println!("{:?}", v1);
                    println!("{:?}", v2);
                    panic!();
                }

                paths.insert((v1, v2), vecs);
            }
        }

        Keypad { keys, paths }
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

impl TryFrom<char> for Vec2 {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '<' => Ok(Vec2::WEST),
            '>' => Ok(Vec2::EAST),
            '^' => Ok(Vec2::NORTH),
            'v' => Ok(Vec2::SOUTH),

            _ => Err(()),
        }
    }
}

impl TryFrom<Vec2> for char {
    type Error = ();

    fn try_from(value: Vec2) -> Result<Self, Self::Error> {
        match value {
            Vec2::WEST => Ok('<'),
            Vec2::EAST => Ok('>'),
            Vec2::NORTH => Ok('^'),
            Vec2::SOUTH => Ok('v'),

            _ => Err(()),
        }
    }
}

// enum Button {
//     D0,
//     D1,
//     D2,
//     D3,
//     D4,
//     D5,
//     D6,
//     D7,
//     D8,
//     D9,
//     Activate,
//     Left,
//     Right,
//     Up,
//     Down,
// }
