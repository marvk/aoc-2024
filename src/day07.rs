use crate::harness::Day;
use crate::harness::Part;

pub struct Part1;

pub fn day07() -> Day<u64, u64> {
    Day::new(7, Box::new(Part1 {}), Box::new(Part2 {}))
}

impl Part<u64> for Part1 {
    fn expect_test(&self) -> u64 {
        3749
    }

    fn solve(&self, input: &[String]) -> u64 {
        let vec = parse(input);

        vec.iter().filter(|e| can_solve1(e)).map(|e| e.result).sum()
    }
}

pub struct Part2;

impl Part<u64> for Part2 {
    fn expect_test(&self) -> u64 {
        11387
    }

    fn solve(&self, input: &[String]) -> u64 {
        let vec = parse(input);

        vec.iter().filter(|e| can_solve2(e)).map(|e| e.result).sum()
    }
}

#[derive(Debug)]
struct Input {
    result: u64,
    operands: Vec<u64>,
}

impl From<&str> for Input {
    fn from(value: &str) -> Self {
        let (first, second) = value.split_once(":").unwrap();

        let result = first.trim().parse().unwrap();
        let operands = second
            .trim()
            .split(" ")
            .map(|e| e.parse().unwrap())
            .collect();

        Self { result, operands }
    }
}

fn parse(input: &[String]) -> Vec<Input> {
    input
        .iter()
        .filter(|e| !e.is_empty())
        .map(|e| Input::from(e.as_str()))
        .collect::<Vec<_>>()
}

fn can_solve1(input: &Input) -> bool {
    solve_rec1(input, 1, input.operands[0])
}

fn solve_rec1(input: &Input, i: usize, running_result: u64) -> bool {
    if i == input.operands.len() {
        running_result == input.result
    } else {
        solve_rec1(input, i + 1, running_result * input.operands[i])
            || solve_rec1(input, i + 1, running_result + input.operands[i])
    }
}

fn can_solve2(input: &Input) -> bool {
    solve_rec2(input, 1, input.operands[0])
}

fn solve_rec2(input: &Input, i: usize, running_result: u64) -> bool {
    if i == input.operands.len() {
        running_result == input.result
    } else {
        solve_rec2(input, i + 1, running_result * input.operands[i])
            || solve_rec2(input, i + 1, running_result + input.operands[i])
            || solve_rec2(
                input,
                i + 1,
                running_result * factor(input.operands[i]) + input.operands[i],
            )
    }
}

fn factor(u: u64) -> u64 {
    match u {
        e if e < 10 => 10,
        e if e < 100 => 100,
        e if e < 1000 => 1000,
        _ => panic!(),
    }
}
