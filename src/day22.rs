use crate::harness::Day;
use crate::harness::Part;
use std::collections::{HashMap, HashSet};
use std::ops::{BitXor, Shl};

pub fn day22() -> Day<u64, u64> {
    Day::new(22, Box::new(Part1 {}), Box::new(Part2 {}))
}

pub struct Part1;

impl Part<u64> for Part1 {
    fn expect_test(&self) -> u64 {
        37327623
    }

    fn solve(&self, input: &[String]) -> u64 {
        Input::from(input)
            .vec
            .into_iter()
            .map(|u| Secret(u).take(2001).last().unwrap())
            .sum::<u64>()
    }
}

pub struct Part2;

impl Part<u64> for Part2 {
    fn expect_test(&self) -> u64 {
        23
    }

    fn solve(&self, input: &[String]) -> u64 {
        let input = Input::from(input).vec;

        
        let result = input
            .into_iter()
            .map(|e| {
                // println!("--------");
                Secret(e)
                    .take(2001)
                    .map(|u| u % 10)
                    .collect::<Vec<_>>()
                    .windows(5)
                    .map(|window| {
                        let diffs = window
                            .windows(2)
                            .map(|pair| pair[1] as i32 - pair[0] as i32)
                            .collect::<Vec<i32>>();

                        let a: i32 = (diffs[0] + 9);
                        let b: i32 = (diffs[1] + 9) * 18;
                        let c: i32 = (diffs[2] + 9) * 18 * 18;
                        let d: i32 = (diffs[3] + 9) * 18 * 18 * 18;
                        let id = (a + b + c + d) as u32;
                        (id, window[4])
                    })
                    .fold(HashMap::new(), |mut acc, (id, result)| {
                        acc.entry(id).or_insert(result);
                        acc
                    })
            })
            .collect::<Vec<_>>();

        let keys = result
            .iter()
            .flat_map(|e| e.keys())
            .cloned()
            .collect::<HashSet<_>>();

        let mut best = 0;

        for id in keys {
            let r = result
                .iter()
                .map(|e| e.get(&id).cloned().unwrap_or_default())
                .sum::<u64>();

            if r >= best {
                best = r;
                best_seq = Some(id);
            }
        }

        best
    }
}

struct Secret(u64);

impl Iterator for Secret {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.0;
        self.0 = evolve(self.0);
        Some(result)
    }
}

fn evolve(mut secret: u64) -> u64 {
    secret = mix(secret, secret * 64);
    secret = prune(secret);
    secret = mix(secret, secret / 32);
    secret = prune(secret);
    secret = mix(secret, secret * 2048);
    secret = prune(secret);

    secret
}

fn mix(secret: u64, u: u64) -> u64 {
    u.bitxor(secret)
}

fn prune(u: u64) -> u64 {
    u % 16777216
}

struct Input {
    vec: Vec<u64>,
}

impl From<&[String]> for Input {
    fn from(value: &[String]) -> Self {
        let vec = value
            .iter()
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<u64>().unwrap())
            .collect();

        Self { vec }
    }
}
