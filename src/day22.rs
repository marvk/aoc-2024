use crate::harness::Day;
use crate::harness::Part;
use std::ops::BitXor;

pub fn day22() -> Day<u64, u32> {
    Day::new(22, Box::new(Part1 {}), Box::new(Part2 {}))
}

pub struct Part1;

impl Part<u64> for Part1 {
    fn expect_test(&self) -> u64 {
        37327623
    }

    fn solve(&self, input: &[String]) -> u64 {
        parse(input)
            .map(|u| Secret(u).take(2001).last().unwrap())
            .map(|u| u as u64)
            .sum::<u64>()
    }
}

pub struct Part2;

impl Part<u32> for Part2 {
    fn expect_test(&self) -> u32 {
        23
    }

    fn solve(&self, input: &[String]) -> u32 {
        const MAX_ID: usize = 19 * 19 * 19 * 19;

        let mut result = [0; MAX_ID];

        for e in parse(input) {
            let mut closed = [false; MAX_ID];

            Secret(e)
                .take(2001)
                .map(|u| u % 10)
                .collect::<Vec<_>>()
                .windows(5)
                .for_each(|window| {
                    let id = window
                        .windows(2)
                        .map(|pair| pair[1] + 9 - pair[0])
                        .enumerate()
                        .map(|(i, e)| e * 19_u32.pow(i as u32))
                        .sum::<u32>() as usize;

                    if !closed[id] {
                        closed[id] = true;
                        result[id] += window[4] as u16;
                    }
                })
        }

        result.into_iter().max().unwrap() as u32
    }
}

struct Secret(u32);

impl Iterator for Secret {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.0;
        self.0 = evolve(self.0);
        Some(result)
    }
}

fn evolve(mut secret: u32) -> u32 {
    secret = evolve_step(secret, secret * 64);
    secret = evolve_step(secret, secret / 32);
    secret = evolve_step(secret, secret * 2048);

    secret
}

fn evolve_step(secret: u32, u: u32) -> u32 {
    u.bitxor(secret) % 16777216
}

fn parse(value: &[String]) -> impl Iterator<Item = u32> + '_ {
    value
        .iter()
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<u32>().unwrap())
}
