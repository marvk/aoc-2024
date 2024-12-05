use crate::harness::Day;
use crate::harness::Part;
use std::cmp::Ordering;
use std::collections::HashMap;

pub struct Part1;

pub fn day05() -> Day<u32, u32> {
    Day::new(5, Box::new(Part1 {}), Box::new(Part2 {}))
}

impl Part<u32> for Part1 {
    fn expect_test(&self) -> u32 {
        143
    }

    fn solve(&self, input: &[String]) -> u32 {
        let (rules, changes) = parse(input);

        let (correct, _) = partition(changes, &rules);

        correct.iter().map(|e| e.center).sum()
    }
}

pub struct Part2;

impl Part<u32> for Part2 {
    fn expect_test(&self) -> u32 {
        123
    }

    fn solve(&self, input: &[String]) -> u32 {
        let (rules, changes) = parse(input);

        let rules = vec![
            Rule {
                first: 1,
                second: 2,
            },
            Rule {
                first: 1,
                second: 3,
            },
            Rule {
                first: 3,
                second: 4,
            },
        ];
        let changes: Vec<Change> = vec![vec![3, 2, 4, 1].into()];

        let (_, incorrect) = partition(changes, &rules);

        let get_rule = |a, b| {
            rules
                .iter()
                .find(|r| (r.first == a && r.second == b) || r.first == b && r.second == a)
        };

        let failed_sorted = incorrect
            .iter()
            .map(|c| c.values.clone())
            .map(|mut e| {
                for i in 0..10 {
                    e.sort_by(|&a, &b| {
                        println!("{}", a);
                        println!("{}", b);

                        let rule = get_rule(a, b);
                        println!("{:?}", rule);

                        if let Some(rule) = get_rule(a, b) {
                            let result = if a == rule.first {
                                println!("{}<>{}", a, b);
                                a.cmp(&b)
                            } else {
                                println!("{}<>{}", b, a);
                                b.cmp(&a)
                            };
                            println!("{:?}", result);
                            result
                        } else {
                            Ordering::Equal
                        }
                    });
                }

                println!();
                println!("{:?}", e);
                println!();
                println!();
                println!();

                e
            })
            .collect::<Vec<_>>();

        let vec = failed_sorted
            .into_iter()
            .map(|e| e.into())
            .collect::<Vec<Change>>();

        let (correct, incorrect) = partition(vec, &rules);

        println!("{}", correct.len());
        println!("{}", incorrect.len());

        todo!()

        // failed_sorted.into_iter().map(|v| v[v.len() / 2]).sum()
    }
}

#[derive(Debug)]
struct Rule {
    first: u32,
    second: u32,
}

impl From<&str> for Rule {
    fn from(value: &str) -> Self {
        let values = value.split("|").flat_map(|e| e.parse()).collect::<Vec<_>>();

        Self {
            first: values[0],
            second: values[1],
        }
    }
}

impl Rule {
    fn check(&self, change: &Change) -> bool {
        match (
            change.pages.get(&self.first),
            change.pages.get(&self.second),
        ) {
            (Some(i1), Some(i2)) => i1 < i2,
            _ => true,
        }
    }
}

#[derive(Debug)]
struct Change {
    values: Vec<u32>,
    pages: HashMap<u32, usize>,
    center: u32,
}

impl From<&str> for Change {
    fn from(value: &str) -> Self {
        let values = value
            .split(",")
            .flat_map(|e| e.parse::<u32>())
            .collect::<Vec<_>>();

        values.into()
    }
}

impl From<Vec<u32>> for Change {
    fn from(value: Vec<u32>) -> Self {
        let center = value[value.len() / 2];

        let pages = value
            .iter()
            .enumerate()
            .map(|(i, e)| (*e, i))
            .collect::<HashMap<_, _>>();

        Self {
            values: value,
            pages,
            center,
        }
    }
}

fn parse(input: &[String]) -> (Vec<Rule>, Vec<Change>) {
    let vec = input.split(|e| e.is_empty()).collect::<Vec<_>>();

    let rules = vec[0]
        .iter()
        .map(|e| Rule::from(e.as_str()))
        .collect::<Vec<_>>();
    let changes = vec[1]
        .iter()
        .map(|e| Change::from(e.as_str()))
        .collect::<Vec<_>>();
    (rules, changes)
}

fn partition(changes: Vec<Change>, rules: &[Rule]) -> (Vec<Change>, Vec<Change>) {
    changes
        .into_iter()
        .partition(|c| rules.iter().all(|r| r.check(c)))
}
