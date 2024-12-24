use crate::harness::Day;
use crate::harness::Part;
use std::collections::{HashMap, HashSet};
use std::fmt::format;
use std::ops::{BitAnd, BitOr, BitXor};

pub fn day24() -> Day<u64, u64> {
    Day::new(24, Box::new(Part1 {}), Box::new(Part2 {}))
}

pub struct Part1;

impl Part<u64> for Part1 {
    fn expect_test(&self) -> u64 {
        2024
    }

    fn solve(&self, input: &[String]) -> u64 {
        let input = Input::from(input);

        // for x in &input.initial {
        //     println!("{:?}", x);
        // }
        //
        // for x in &input.gates {
        //     println!("{:?}", x);
        // }

        let mut values = input.initial.clone();

        let mut result_names = input
            .gates
            .iter()
            .map(|g| g.result)
            .filter(|s| s.starts_with('z'))
            .collect::<Vec<_>>();

        result_names.sort();

        while !result_names.iter().all(|s| values.contains_key(s)) {
            for Gate {
                operand1,
                operand2,
                result,
                operator,
            } in &input.gates
            {
                if !values.contains_key(result) {
                    if let Some(operand1) = values.get(operand1) {
                        if let Some(operand2) = values.get(operand2) {
                            let result_value = match operator {
                                Operator::AND => operand1.bitand(operand2),
                                Operator::OR => operand1.bitor(operand2),
                                Operator::XOR => operand1.bitxor(operand2),
                            };

                            values.insert(result, result_value);
                        }
                    }
                }
            }
        }

        result_names
            .iter()
            .map(|s| values[s])
            .enumerate()
            .map(|(i, value)| (value as u64) << i)
            .sum()
    }
}

pub struct Part2;

impl Part<u64> for Part2 {
    fn expect_test(&self) -> u64 {
        0
    }

    fn solve(&self, input: &[String]) -> u64 {
        let input = Input::from(input);

        let mut nodes = HashSet::new();

        for &Gate {
            operand1,
            operand2,
            result,
            ..
        } in &input.gates
        {
            nodes.insert(operand1);
            nodes.insert(operand2);
            nodes.insert(result);
        }

        let nodes = nodes
            .into_iter()
            .enumerate()
            .map(|(a, b)| (b.to_string(), a))
            .collect::<HashMap<_, _>>();

        let mut edges = vec![];

        let mut id = nodes.len();

        let mut additional_nodes = vec![];

        for Gate {
            operand1,
            operand2,
            result,
            operator,
        } in input.gates
        {
            let operator = format!("{:?}", operator);
            let operator_id = id;
            id += 1;
            additional_nodes.push((operator, operator_id));
            edges.push((nodes[operand1], operator_id));
            edges.push((nodes[operand2], operator_id));
            edges.push((operator_id, nodes[result]));
        }

        let mut result = String::new();

        let mut nodes = nodes.into_iter().collect::<Vec<_>>();

        nodes.sort_by_key(|e| e.1);
        
        for x in nodes {
            result.push_str(&format!("{} {}\n", x.1, x.0));
        }
        
        for x in additional_nodes {
            result.push_str(&format!("{} {}\n", x.1, x.0));
        }

        result.push_str("#\n");

        for x in edges {
            result.push_str(&format!("{} {} {}\n", x.0, x.1, "e"));
        }

        println!("{}", result);

        0
    }
}

#[derive(Debug)]
enum Operator {
    AND,
    OR,
    XOR,
}

impl TryFrom<&str> for Operator {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "AND" => Ok(Operator::AND),
            "OR" => Ok(Operator::OR),
            "XOR" => Ok(Operator::XOR),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
struct Gate<'a> {
    operand1: &'a str,
    operand2: &'a str,
    result: &'a str,
    operator: Operator,
}

#[derive(Debug)]
struct Input<'a> {
    initial: HashMap<&'a str, bool>,
    gates: Vec<Gate<'a>>,
}

impl<'a> From<&'a [String]> for Input<'a> {
    fn from(value: &'a [String]) -> Self {
        let mut split = value.split(|s| s.is_empty());
        let first = split.next().unwrap();
        let second = split.next().unwrap();

        drop(split);

        let initial = first
            .iter()
            .map(|s| {
                let mut split = s.split(":");
                (
                    split.next().unwrap(),
                    split.next().unwrap().trim().parse::<u8>().unwrap() != 0,
                )
            })
            .collect();

        let gates = second
            .iter()
            .map(|s| {
                let mut split = s.split(" -> ");
                let first = split.next().unwrap();
                let result = split.next().unwrap();

                let mut split = first.split(" ");
                let operand1 = split.next().unwrap();
                let operator = split.next().unwrap().try_into().unwrap();
                let operand2 = split.next().unwrap();

                Gate {
                    operand1,
                    operand2,
                    operator,
                    result,
                }
            })
            .collect();

        Self { initial, gates }
    }
}
