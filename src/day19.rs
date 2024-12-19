use crate::harness::Day;
use crate::harness::Part;
use std::collections::HashMap;

pub fn day19() -> Day<u64, u64> {
    Day::new(19, Box::new(Part1 {}), Box::new(Part2 {}))
}

pub struct Part1;

impl Part<u64> for Part1 {
    fn expect_test(&self) -> u64 {
        6
    }

    fn solve(&self, input: &[String]) -> u64 {
        let input = Input::from(input);

        let mut result = 0;

        for design in input.designs {
            if backtrack(&input.patterns, design, &mut String::new()) {
                result += 1;
            };
        }

        result
    }
}

pub struct Part2;

impl Part<u64> for Part2 {
    fn expect_test(&self) -> u64 {
        16
    }

    fn solve(&self, input: &[String]) -> u64 {
        let input = Input::from(input);

        let mut result = 0;
        let mut map = HashMap::new();

        for design in input.designs {
            result += backtrack2(&input.patterns, design, &mut String::new(), &mut map);
            map.clear();
        }

        result as u64
    }
}

fn backtrack(patterns: &[&str], design: &str, running_result: &mut String) -> bool {
    if running_result.len() == design.len() {
        return true;
    }

    let current_len = running_result.len();
    for &pattern in patterns {
        let next_len = current_len + pattern.len();
        if design.len() >= next_len && pattern == &design[current_len..next_len] {
            running_result.push_str(pattern);
            if backtrack(patterns, design, running_result) {
                return true;
            }
            running_result.truncate(current_len);
        }
    }

    false
}

fn backtrack2(
    patterns: &[&str],
    design: &str,
    running_result: &mut String,
    cache: &mut HashMap<String, usize>,
) -> usize {
    if running_result.len() == design.len() {
        return 1;
    }

    if let Some(result) = cache.get(running_result) {
        return *result;
    }

    let mut result = 0;

    let current_len = running_result.len();
    for &pattern in patterns {
        let next_len = current_len + pattern.len();
        if design.len() >= next_len && pattern == &design[current_len..next_len] {
            running_result.push_str(pattern);
            result += backtrack2(patterns, design, running_result, cache);
            running_result.truncate(current_len);
        }
    }

    cache.insert(running_result.clone(), result);

    result
}

#[derive(Debug)]
struct Input<'a> {
    patterns: Vec<&'a str>,
    designs: Vec<&'a str>,
}

impl<'a> From<&'a [String]> for Input<'a> {
    fn from(value: &'a [String]) -> Self {
        let mut value = value.iter().map(|s| s.as_str());
        Self {
            patterns: value.next().unwrap().split(", ").collect(),
            designs: value.filter(|e| !e.is_empty()).collect(),
        }
    }
}
