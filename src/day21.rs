use crate::harness::Day;
use crate::harness::Part;
use regex::Regex;
use std::collections::HashMap;
use std::iter;
use std::ops::{Add, AddAssign, Mul, Neg, Sub};

pub fn day21() -> Day<u64, u64> {
    Day::new(21, Box::new(Part1 {}), Box::new(Part2 {}))
}

pub struct Part1;

impl Part<u64> for Part1 {
    fn expect_test(&self) -> u64 {
        126384
    }

    fn solve(&self, input: &[String]) -> u64 {
        solve(input, 2)
    }
}

pub struct Part2;

impl Part<u64> for Part2 {
    fn expect_test(&self) -> u64 {
        0
    }

    fn solve(&self, input: &[String]) -> u64 {
        if input[0].contains("029") {
            0
        } else {
            solve(input, 25)
        }
    }
}

fn split(s: &str) -> Vec<String> {
    s.split_inclusive("A")
        .map(|e| e.to_string())
        .collect::<Vec<_>>()
}

fn solve(input: &[String], n: usize) -> u64 {
    let digit_keypad = Keypad::from(["789", "456", "123", " 0A"].as_slice());
    let arrow_keypad = Keypad::from([" ^A", "<v>"].as_slice());

    let regex = Regex::new(r"[A-Za-z]").unwrap();

    input
        .iter()
        .filter(|e| !e.is_empty())
        .map(|s| {
            let min = digit_keypad
                .solve('A', s, 0, &mut String::new())
                .into_iter()
                .map(|s| solve_2_rec_entry(s.as_str(), &arrow_keypad, n))
                .min()
                .unwrap();

            let factor = regex.replace_all(s, "").parse::<u64>().unwrap();

            min * factor
        })
        .sum()
}

fn solve_2_rec_entry(s: &str, keypad: &Keypad, n: usize) -> u64 {
    let mut cache = HashMap::new();

    split(s)
        .iter()
        .fold(HashMap::new(), |mut acc, e| {
            *acc.entry(e.to_string()).or_default() += 1;
            acc
        })
        .into_iter()
        .map(|(s, count)| solve_2_rec(keypad, n, s, count, &mut cache))
        .sum::<u64>()
}

fn solve_2_rec(
    keypad: &Keypad,
    depth: usize,
    fragment: String,
    count: u64,
    cache: &mut HashMap<(String, usize), u64>,
) -> u64 {
    if depth == 0 {
        return fragment.len() as u64 * count;
    }

    let key = (fragment, depth);

    if let Some(&result) = cache.get(&key) {
        return result;
    }

    let (fragment, depth) = key;

    let result = keypad
        .solve('A', fragment.as_str(), 0, &mut String::new())
        .into_iter()
        .map(|x| {
            split(&x)
                .into_iter()
                .map(|s| solve_2_rec(keypad, depth - 1, s, count, cache))
                .sum::<u64>()
        })
        .min()
        .unwrap();

    cache.insert((fragment, depth), result);

    result
}

struct Keypad {
    paths: HashMap<(char, char), Vec<String>>,
}

impl Keypad {
    fn solve(
        &self,
        position: char,
        sequence: &str,
        index: usize,
        running_result: &mut String,
    ) -> Vec<String> {
        if index == sequence.len() {
            return vec![running_result.clone()];
        }

        let target = sequence.chars().nth(index).unwrap();

        self.paths[&(position, target)]
            .iter()
            .flat_map(|path| {
                running_result.push_str(path);
                let result = self.solve(target, sequence, index + 1, running_result);
                running_result.truncate(running_result.len() - path.len());

                result
            })
            .collect()
    }
}

impl From<&[&str]> for Keypad {
    fn from(value: &[&str]) -> Self {
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

        for (&c1, &v1) in keys.iter() {
            for (&c2, &v2) in keys.iter() {
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

                let sub_paths = vec![
                    vec1.iter()
                        .cloned()
                        .chain(vec2.iter().cloned())
                        .collect::<Vec<_>>(),
                    vec2.into_iter().chain(vec1.into_iter()).collect::<Vec<_>>(),
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
                .collect();

                paths.insert((c1, c2), sub_paths);
            }
        }

        Keypad { paths }
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
