use crate::harness::Day;
use crate::harness::Part;
use std::collections::HashMap;
pub struct Part1;

pub fn day11() -> Day<usize, usize> {
    Day::new(11, Box::new(Part1 {}), Box::new(Part2 {}))
}

impl Part<usize> for Part1 {
    fn expect_test(&self) -> usize {
        55312
    }

    fn solve(&self, input: &[String]) -> usize {
        let mut input = Input::from(input[0].as_str());

        for _ in 0..25 {
            input.iterate_once();
        }

        input.map.into_values().sum()
    }
}

pub struct Part2;

impl Part<usize> for Part2 {
    fn expect_test(&self) -> usize {
        65601038650482
    }

    fn solve(&self, input: &[String]) -> usize {
        let mut input = Input::from(input[0].as_str());

        for _ in 0..75 {
            input.iterate_once();
        }

        input.map.into_values().sum()
    }
}

#[derive(Debug)]
struct Input {
    map: HashMap<u64, usize>,
}

impl From<&str> for Input {
    fn from(value: &str) -> Self {
        let map = value.split(" ").map(|e| e.parse::<u64>().unwrap()).fold(
            HashMap::new(),
            |mut acc, e| {
                let vec = acc.entry(e).or_insert(0);
                *vec += 1;
                acc
            },
        );

        Self { map }
    }
}

impl Input {
    fn iterate_once(&mut self) {
        let mut new_map = HashMap::new();

        for (&num, &count) in &self.map {
            let new_nums = if num == 0 {
                vec![1_u64]
            } else if num.to_string().len() % 2 == 0 {
                let s = num.to_string();
                vec![
                    s[0..s.len() / 2].parse().unwrap(),
                    s[s.len() / 2..s.len()].parse().unwrap(),
                ]
            } else {
                vec![num * 2024]
            };

            for new_num in new_nums {
                *new_map.entry(new_num).or_insert(0) += count;
            }
        }

        self.map = new_map;
    }
}
