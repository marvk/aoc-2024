use crate::harness::Day;
use crate::harness::Part;
use regex::Regex;

pub struct Part1;

pub fn day03() -> Day<i32, i32> {
    Day::new(3, Box::new(Part1 {}), Box::new(Part2 {}))
}

impl Part<i32> for Part1 {
    fn expect_test(&self) -> i32 {
        161
    }

    fn solve(&self, input: &[String]) -> i32 {
        solve(input, true)
    }
}

pub struct Part2;

impl Part<i32> for Part2 {
    fn expect_test(&self) -> i32 {
        48
    }

    fn solve(&self, input: &[String]) -> i32 {
        solve(input, false)
    }
}

fn solve(input: &[String], all: bool) -> i32 {
    let input = input.join("\n");

    let regex = Regex::new("(mul\\((\\d+),(\\d+)\\))|(don't\\(\\))|(do\\(\\))");

    let mut enabled = true;

    regex
        .unwrap()
        .captures_iter(&input)
        .map(|c| {
            if c.get(4).is_some() {
                enabled = false
            } else if c.get(5).is_some() {
                enabled = true
            } else if enabled || all {
                let group_as_i32 = |i| c.get(i).unwrap().as_str().parse::<i32>().unwrap();
                return group_as_i32(2) * group_as_i32(3);
            }

            return 0;
        })
        .sum()
}
