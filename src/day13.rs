use crate::harness::Day;
use crate::harness::Part;
use regex::Regex;
use std::sync::OnceLock;

pub fn day13() -> Day<u64, u64> {
    Day::new(13, Box::new(Part1 {}), Box::new(Part2 {}))
}

pub struct Part1;

impl Part<u64> for Part1 {
    fn expect_test(&self) -> u64 {
        480
    }

    fn solve(&self, input: &[String]) -> u64 {
        Input::from(input)
            .machines
            .iter()
            .filter_map(|e| e.solve())
            .sum()
    }
}

pub struct Part2;

impl Part<u64> for Part2 {
    fn expect_test(&self) -> u64 {
        875318608908
    }

    fn solve(&self, input: &[String]) -> u64 {
        let mut input = Input::from(input);

        for x in &mut input.machines {
            x.prize.x += 10000000000000.0;
            x.prize.y += 10000000000000.0;
        }

        input.machines.iter().filter_map(|e| e.solve()).sum()
    }
}

struct Button {
    x: f64,
    y: f64,
}

impl From<&str> for Button {
    fn from(value: &str) -> Self {
        static REGEX: OnceLock<Regex> = OnceLock::new();
        let regex =
            REGEX.get_or_init(|| Regex::new("Button [AB]: X\\+(\\d+), Y\\+(\\d+)").unwrap());

        let vec = parse_two_numbers(value, regex);

        Self {
            x: vec[0],
            y: vec[1],
        }
    }
}

struct Prize {
    x: f64,
    y: f64,
}

impl From<&str> for Prize {
    fn from(value: &str) -> Self {
        static REGEX: OnceLock<Regex> = OnceLock::new();
        let regex = REGEX.get_or_init(|| Regex::new("Prize: X=(\\d+), Y=(\\d+)").unwrap());

        let vec = parse_two_numbers(value, regex);

        Self {
            x: vec[0],
            y: vec[1],
        }
    }
}

struct Machine {
    button_a: Button,
    button_b: Button,
    prize: Prize,
}

impl Machine {
    fn solve(&self) -> Option<u64> {
        let ax = self.button_a.x;
        let bx = self.button_b.x;
        let ay = self.button_a.y;
        let by = self.button_b.y;

        let x = self.prize.x;
        let y = self.prize.y;

        let a = (y * bx - x * by) / (bx * ay - ax * by);
        let b = (-y * ax + x * ay) / (bx * ay - ax * by);

        if a.fract() == 0.0 && b.fract() == 0.0 {
            Some((a * 3.0 + b) as u64)
        } else {
            None
        }
    }
}

impl From<&[String]> for Machine {
    fn from(value: &[String]) -> Self {
        Self {
            button_a: value[0].as_str().into(),
            button_b: value[1].as_str().into(),
            prize: value[2].as_str().into(),
        }
    }
}

struct Input {
    machines: Vec<Machine>,
}

impl From<&[String]> for Input {
    fn from(value: &[String]) -> Self {
        Self {
            machines: value
                .split(|l| l.is_empty())
                .filter(|e| e.len() == 3)
                .map(Machine::from)
                .collect(),
        }
    }
}

fn parse_two_numbers(value: &str, regex: &Regex) -> Vec<f64> {
    regex
        .captures(value)
        .unwrap()
        .iter()
        .skip(1)
        .map(|e| e.unwrap().as_str().parse().unwrap())
        .collect::<Vec<_>>()
}
