use crate::harness::Day;
use crate::harness::Part;
use std::collections::HashMap;
use std::mem::swap;

pub struct Part1;

pub fn day11() -> Day<usize, usize> {
    Day::new(11, Box::new(Part1 {}), Box::new(Part2 {}))
}

impl Part<usize> for Part1 {
    fn expect_test(&self) -> usize {
        55312
    }

    fn solve(&self, input: &[String]) -> usize {
        solve(input, 25)
    }
}

pub struct Part2;

impl Part<usize> for Part2 {
    fn expect_test(&self) -> usize {
        65601038650482
    }

    fn solve(&self, input: &[String]) -> usize {
        solve(input, 75)
    }
}

fn solve(input: &[String], i: usize) -> usize {
    let mut input = Input::from(input[0].as_str());

    for _ in 0..i {
        input.iterate_once();
    }

    input.map.into_values().sum()
}

#[derive(Debug)]
struct Input {
    map: HashMap<u64, usize>,
    next_map: HashMap<u64, usize>,
}

impl From<&str> for Input {
    fn from(value: &str) -> Self {
        let map = value.split(" ").map(|e| e.parse::<u64>().unwrap()).fold(
            HashMap::new(),
            |mut acc, e| {
                *acc.entry(e).or_insert(0) += 1;
                acc
            },
        );

        Self {
            map,
            next_map: HashMap::new(),
        }
    }
}

impl Input {
    fn iterate_once(&mut self) {
        self.next_map.clear();

        #[inline(always)]
        fn insert(map: &mut HashMap<u64, usize>, num: u64, count: usize) {
            *map.entry(num).or_insert(0) += count;
        }

        for (&num, &count) in &self.map {
            if num == 0 {
                insert(&mut self.next_map, 1, count);
            } else {
                let digits = num.ilog10() + 1;

                if digits % 2 == 0 {
                    let divisor = 10_u64.pow(digits / 2);

                    insert(&mut self.next_map, num / divisor, count);
                    insert(&mut self.next_map, num % divisor, count);
                } else {
                    insert(&mut self.next_map, num * 2024, count);
                }
            }
        }

        swap(&mut self.map, &mut self.next_map)
    }
}
