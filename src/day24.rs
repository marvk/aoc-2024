use crate::harness::Day;
use crate::harness::Part;
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};
use std::mem::swap;
use std::ops::{BitAnd, BitOr, BitXor};

pub fn day24() -> Day<u64, String> {
    Day::new(24, Box::new(Part1 {}), Box::new(Part2 {}))
}

pub struct Part1;

impl Part<u64> for Part1 {
    fn expect_test(&self) -> u64 {
        2024
    }

    fn solve(&self, input: &[String]) -> u64 {
        let input = Input::from(input);

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
                ..
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

impl Part<String> for Part2 {
    fn expect_test(&self) -> String {
        "".to_string()
    }

    fn solve(&self, input: &[String]) -> String {
        if input.len() < 100 {
            return "".to_string();
        }
        let input = Input::from(input);

        let (mut broken_gates, adders) = solve(input.clone());

        for x in adders {
            if !x.is_complete() {
                x.or.and_then(|g| {
                    broken_gates.push(g);
                    Some(())
                });

                x.half_adder1.and.and_then(|g| {
                    broken_gates.push(g);
                    Some(())
                });

                x.half_adder1.xor.and_then(|g| {
                    broken_gates.push(g);
                    Some(())
                });

                x.half_adder2.and.and_then(|g| {
                    broken_gates.push(g);
                    Some(())
                });

                x.half_adder2.xor.and_then(|g| {
                    broken_gates.push(g);
                    Some(())
                });
            }
        }

        for x in broken_gates {
            println!("{}", x);
        }

        // for i in 0..broken_gates.len() {
        //     for j in 0..broken_gates.len() {
        //         if i == j {
        //             continue;
        //         }
        //
        //         let mut current_input = input.clone();
        //
        //         let id1 = broken_gates[i].id;
        //         let id2 = broken_gates[j].id;
        //
        //         current_input.gate(id1).result = broken_gates[j].result;
        //         current_input.gate(id2).result = broken_gates[i].result;
        //
        //         todo!()
        //     }
        // }
        //
        // for x in good_swaps {
        //     println!("{:?}", x);
        // }

        panic!();

        let mut broken = broken_gates.iter().map(|e| e.result).collect::<Vec<_>>();
        broken.sort();
        broken.join(",")
    }
}

fn solve(mut input: Input) -> (Vec<Gate>, Vec<FullAdder>) {
    // input.gate(152).result = "qdg";
    // input.gate(197).result = "z12";
    //
    // input.gate(145).result = "vvf";
    // input.gate(82).result = "z19";
    //
    // input.gate(184).result = "fgn";
    // input.gate(5).result = "dck";
    //
    // input.gate(178).result = "z37";
    // input.gate(120).result = "nvh";

    let mut adders = (0..46).map(|_| FullAdder::default()).collect::<Vec<_>>();

    let mut broken_gates = vec![];

    let mut i = 0;
    while i < input.gates.len() {
        let gate = &input.gates[i];

        if gate.operand1.starts_with('x') && gate.operand2.starts_with('y') {
            let id = gate.operand1[1..].parse::<usize>().unwrap();
            if gate.result.starts_with('z') {
                broken_gates.push(input.gates.remove(i));
            } else {
                match gate.operator {
                    Operator::AND => {
                        adders[id].half_adder1.and = Some(input.gates.remove(i));
                    }
                    Operator::XOR => {
                        adders[id].half_adder1.xor = Some(input.gates.remove(i));
                    }
                    Operator::OR => {
                        broken_gates.push(input.gates.remove(i));
                    }
                }
            }
        } else {
            i += 1;
        }
    }

    let mut i = 0;

    while i < input.gates.len() {
        let gate = &input.gates[i];

        if gate.result.starts_with('z') {
            let id = gate.result[1..].parse::<usize>().unwrap();

            if let Some(xor1) = &adders[id].half_adder1.xor {
                if xor1.result != gate.operand1 && xor1.result != gate.operand2 {
                    broken_gates.push(input.gates.remove(i));
                    continue;
                }
            }

            match gate.operator {
                Operator::XOR => {
                    adders[id].half_adder2.xor = Some(input.gates.remove(i));
                }
                _ => {
                    broken_gates.push(input.gates.remove(i));
                }
            }
        } else {
            i += 1;
        }
    }

    for x in &mut adders {
        if let Some(xor) = &x.half_adder1.xor {
            let position = input.gates.iter().position(|e| {
                matches!(e.operator, Operator::AND)
                    && (xor.result == e.operand1 || xor.result == e.operand2)
            });

            if let Some(position) = position {
                let gate2 = input.gates.remove(position);
                x.half_adder2.and = Some(gate2);
            }
        }
    }

    for x in &mut adders {
        if let Some(and) = &x.half_adder2.and {
            let position = input.gates.iter().position(|e| {
                matches!(e.operator, Operator::OR)
                    && (and.result == e.operand1 || and.result == e.operand2)
            });

            if let Some(position) = position {
                x.or = Some(input.gates.remove(position));
            }
        }
    }

    // println!("{}", input.gates.len());
    //
    // for x in &adders {
    //     println!("{}", x);
    //     if let Some(xor1) = &x.half_adder1.xor {
    //         if let Some(xor2) = &x.half_adder2.xor {
    //             if xor1.result != xor2.operand1 && xor1.result != xor2.operand2 {
    //                 panic!("AHH {}", &x);
    //             }
    //         }
    //     }
    // }

    // println!("---------");
    //
    // for x in broken_gates {
    //     println!("{}", x);
    // }
    //
    // println!("---------");
    //
    // for x in input.gates {
    //     println!("{}", x);
    // }<
    //
    // let mut result = ["qdg", "z12", "vvf", "z19", "fgn", "dck", "z37", "nvh"];
    // result.sort();
    // result.join(",")

    (
        input.gates.into_iter().chain(broken_gates).collect(),
        adders,
    )
}




#[derive(Default, Debug)]
struct HalfAdder<'a> {
    xor: Option<Gate<'a>>,
    and: Option<Gate<'a>>,
}

impl<'a> Display for HalfAdder<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{xor: {:?}, and: {:?}}}", self.xor, self.and)
    }
}

#[derive(Default, Debug)]
struct FullAdder<'a> {
    half_adder1: HalfAdder<'a>,
    half_adder2: HalfAdder<'a>,
    or: Option<Gate<'a>>,
}

impl<'a> HalfAdder<'a> {
    fn is_compete(&self) -> bool {
        self.xor.is_some() && self.and.is_some()
    }
}

impl<'a> FullAdder<'a> {
    fn is_complete(&self) -> bool {
        self.or.is_some() && self.half_adder1.is_compete() && self.half_adder2.is_compete()
    }
}

impl<'a> Display for FullAdder<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{ha1: {}, ha2: {}, or: {:?}}}",
            self.half_adder1, self.half_adder2, self.or
        )
    }
}

#[derive(Debug, Copy, Clone)]
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

#[derive(Clone)]
struct Gate<'a> {
    id: usize,
    operand1: &'a str,
    operand2: &'a str,
    result: &'a str,
    operator: Operator,
}

impl<'a> Debug for Gate<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl<'a> Display for Gate<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({}) {} {:?} {} -> {}",
            self.id, self.operand1, &self.operator, self.operand2, self.result
        )
    }
}

#[derive(Debug, Clone)]
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

        let mut next_id = 0;

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
                let mut operands = [operand1, operand2];
                operands.sort();
                let operand1 = operands[0];
                let operand2 = operands[1];

                let id = next_id;
                next_id += 1;

                Gate {
                    id,
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

impl<'a> Input<'a> {
    fn gate(&mut self, id: usize) -> &mut Gate<'a> {
        self.gates.iter_mut().find(|g| g.id == id).unwrap()
    }
}
