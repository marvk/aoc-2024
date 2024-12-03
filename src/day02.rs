use crate::harness::Day;
use crate::harness::Part;

pub struct Part1;

pub fn day02() -> Day<usize, usize> {
    Day::new(2, Box::new(Part1 {}), Box::new(Part2 {}))
}

impl Part<usize> for Part1 {
    fn expect_test(&self) -> usize {
        2
    }

    fn solve(&self, input: &[String]) -> usize {
        parse(input).into_iter().filter(|e| is_safe(e)).count()
    }
}

pub struct Part2;

impl Part<usize> for Part2 {
    fn expect_test(&self) -> usize {
        4
    }

    fn solve(&self, input: &[String]) -> usize {
        parse(input)
            .into_iter()
            .filter(|e| {
                if is_safe(e) {
                    return true;
                }

                for i in 0..e.len() {
                    let iter = e
                        .iter()
                        .take(i)
                        .chain(e.iter().skip(i + 1))
                        .cloned()
                        .collect::<Vec<_>>();

                    if is_safe(&iter) {
                        return true;
                    }
                }

                return false;
            })
            .count()
    }
}

fn parse(input: &[String]) -> Vec<Vec<i32>> {
    input
        .iter()
        .filter(|e| !e.is_empty())
        .map(|e| e.split(" ").map(|e| e.parse().unwrap()).collect())
        .collect()
}

fn is_safe(e: &[i32]) -> bool
{
    check(e, 1, 3) || check(e, -3, -1)
}

fn check(e: &[i32], min: i32, max: i32) -> bool {
    e.windows(2)
        .map(|e| e[0] - e[1])
        .all(|e| e >= min && e <= max)
}
