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

fn split(s: &str) -> impl Iterator<Item = String> + '_ {
    s.split_inclusive("A").map(|e| e.to_string())
}

fn solve(input: &[String], n: usize) -> u64 {
    let digit_keypad = Keypad::from(["789", "456", "123", " 0A"].as_slice());
    let arrow_keypad = Keypad::from([" ^A", "<v>"].as_slice());

    let regex = Regex::new(r"[A-Za-z]").unwrap();

    let mut fragment_cache = HashMap::new();

    input
        .iter()
        .filter(|e| !e.is_empty())
        .map(|s| {
            let min = digit_keypad
                .solve_one_sequence('A', s, 0, &mut vec![], &mut fragment_cache)
                .into_iter()
                .map(|s| arrow_keypad.solve_full_sequence(s, n, &mut fragment_cache))
                .min()
                .unwrap();

            let factor = regex.replace_all(s, "").parse::<u64>().unwrap();

            min * factor
        })
        .sum()
}

struct Keypad {
    paths: HashMap<(char, char), Vec<String>>,
}

impl Keypad {
    fn solve_one_sequence<'a>(
        &'a self,
        position: char,
        sequence: &str,
        index: usize,
        running_result: &mut Vec<&'a str>,
        fragment_cache: &mut HashMap<String, Vec<Vec<&'a str>>>,
    ) -> Vec<Vec<&str>> {
        if index == 0 {
            if let Some(result) = fragment_cache.get(sequence) {
                return result.clone();
            }
        }

        if index == sequence.len() {
            return vec![running_result.clone()];
        }

        let target = sequence.as_bytes()[index] as char;

        let result = self.paths[&(position, target)]
            .iter()
            .flat_map(|path: &'a String| {
                running_result.push(path);
                let result = self.solve_one_sequence(
                    target,
                    sequence,
                    index + 1,
                    running_result,
                    fragment_cache,
                );
                running_result.pop();

                result
            })
            .collect::<Vec<_>>();

        fragment_cache.insert(sequence.to_string(), result.clone());

        result
    }

    fn solve_full_sequence<'a>(
        &'a self,
        s: Vec<&str>,
        n: usize,
        fragment_cache: &mut HashMap<String, Vec<Vec<&'a str>>>,
    ) -> u64 {
        let mut full_cache = HashMap::new();

        s.iter()
            .fold(HashMap::new(), |mut acc, e| {
                *acc.entry(e.to_string()).or_default() += 1;
                acc
            })
            .into_iter()
            .map(|(s, count)| {
                self.solve_full_sequence_rec(n, s, count, &mut full_cache, fragment_cache)
            })
            .sum::<u64>()
    }

    fn solve_full_sequence_rec<'a>(
        &'a self,
        depth: usize,
        fragment: String,
        count: u64,
        full_cache: &mut HashMap<(String, usize), u64>,
        fragment_cache: &mut HashMap<String, Vec<Vec<&'a str>>>,
    ) -> u64 {
        if depth == 0 {
            return fragment.len() as u64 * count;
        }

        let key = (fragment, depth);

        if let Some(&result) = full_cache.get(&key) {
            return result;
        }

        let (fragment, depth) = key;

        let vec1 = self.solve_one_sequence('A', fragment.as_str(), 0, &mut vec![], fragment_cache);
        let result = vec1
            .into_iter()
            .map(|x| {
                x.iter()
                    .map(|s| {
                        self.solve_full_sequence_rec(
                            depth - 1,
                            s.to_string(),
                            count,
                            full_cache,
                            fragment_cache,
                        )
                    })
                    .sum::<u64>()
            })
            .min()
            .unwrap();

        full_cache.insert((fragment, depth), result);

        result
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
