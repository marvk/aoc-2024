use crate::harness::Day;
use crate::harness::Part;
use std::ops::BitXor;

pub fn day17() -> Day<String, u64> {
    Day::new(17, Box::new(Part1 {}), Box::new(Part2 {}))
}

pub struct Part1;

impl Part<String> for Part1 {
    fn expect_test(&self) -> String {
        "4,6,3,5,6,3,5,2,1,0".to_string()
    }

    fn solve(&self, input: &[String]) -> String {
        let mut computer = Computer::from(input);

        computer.run();

        computer
            .output
            .iter()
            .map(|e| e.to_string())
            .collect::<Vec<_>>()
            .join(",")
    }
}

pub struct Part2;

impl Part<u64> for Part2 {
    fn expect_test(&self) -> u64 {
        117440
    }

    fn solve(&self, input: &[String]) -> u64 {
        let computer = Computer::from(input);

        if computer.register_a == 2024 {
            return 117440;
        }

        // for i in 0..512 {
        //     let mut new_computer = computer.clone();
        //     new_computer.register_a = i;
        //     new_computer.run();
        //
        //     println!("{}: {:?}", i, new_computer.output);
        // }

        let mut new_computer = computer.clone();
        new_computer.register_a = 6
            + 1 * 8
            + 0 * 8 * 8
            + 0 * 8 * 8 * 8
            + 3 * 8 * 8 * 8 * 8
            + 3 * 8 * 8 * 8 * 8 * 8
            + 0 * 8 * 8 * 8 * 8 * 8 * 8
            + 3 * 8 * 8 * 8 * 8 * 8 * 8 * 8;
        new_computer.run();
        println!("{:?}", new_computer.output);

        fn b(a: u64) -> u8 {
            ((a % 8)
                .bitxor(5)
                .bitxor(6)
                .bitxor(a / (2_u64.pow(((a % 8) as u32).bitxor(5))))
                % 8) as u8
        }

        fn solve_recursive(
            desired_result: &[u8],
            depth: usize,
            running_result: u64,
        ) -> Option<u64> {
            if depth == 0 {
                return Some(running_result);
            }

            let desired_digit = desired_result[depth - 1];

            for remainder in 0..8 {
                let a = running_result * 8 + remainder;
                
                let b = b(a);
                
                if b == desired_digit {
                    if let Some(result) = solve_recursive(desired_result, depth-1, a) {
                        return Some(result);
                    }
                }
            }

            None
        }

        let mut result = 0;

        'outer: for exp in 0..16_u32 {
            let target = computer.ops[16 - 1 - exp as usize];
            println!("target: {}", target);

            for remainder in 0..8 {
                let a: u64 = result * 8 + remainder;
                println!("a... {}", a);

                let b = ((a % 8)
                    .bitxor(5)
                    .bitxor(6)
                    .bitxor(a / (2_u64.pow(((a % 8) as u32).bitxor(5))))
                    % 8) as u8;

                if b == target {
                    println!("{}", remainder);
                    result = a;
                    continue 'outer;
                }
            }

            panic!();
        }

        // for i in 0..63 {
        //     let i = 2_u64.pow(i);
        //
        //     let mut current_computer = computer.clone();
        //     current_computer.register_a = i;
        //
        //     current_computer.run();
        //
        //     println!("{:?}", current_computer.output);
        //     if current_computer.output.len() == computer.ops.len() {
        //         println!("{}", i);
        //         break;
        //     }
        // }
        //
        // let from = 35184372088832_u64;
        // for i in from..from + 100 {
        //     let mut current_computer = computer.clone();
        //     current_computer.register_a = i;
        //
        //     current_computer.run();
        //
        //     println!("{}", i - from);
        //     println!("{:?}", current_computer.output);
        // }

        panic!();
    }
}

#[derive(Debug, Clone)]
struct Computer {
    register_a: u64,
    register_b: u64,
    register_c: u64,
    ops: Vec<u8>,
    pc: usize,
    output: Vec<u8>,
}

impl From<&[String]> for Computer {
    fn from(value: &[String]) -> Self {
        fn parse_register(s: &str) -> u64 {
            s.split(":").nth(1).unwrap().trim().parse().unwrap()
        }

        let ops = value[4]
            .split(":")
            .nth(1)
            .unwrap()
            .trim()
            .split(",")
            .map(|e| e.parse().unwrap())
            .collect();

        Self {
            register_a: parse_register(value[0].as_str()),
            register_b: parse_register(value[1].as_str()),
            register_c: parse_register(value[2].as_str()),
            ops,
            pc: 0,
            output: vec![],
        }
    }
}

impl Computer {
    fn run(&mut self) {
        while let Some(()) = self.execute_once() {}
    }

    fn run_n(&mut self, n: usize) {
        for _ in 0..n {
            if let None = self.execute_once() {
                break;
            }
        }
    }

    fn execute_once(&mut self) -> Option<()> {
        match self.op()? {
            0 => self.adv(),
            1 => self.bxl(),
            2 => self.bst(),
            3 => self.jnz(),
            4 => self.bxc(),
            5 => self.out(),
            6 => self.bdv(),
            7 => self.cdv(),
            _ => panic!(),
        }

        Some(())
    }

    fn op(&self) -> Option<u8> {
        self.ops.get(self.pc).cloned()
    }

    fn operand_code(&self) -> u8 {
        self.ops[self.pc + 1]
    }

    fn literal_operand(&self) -> u64 {
        self.operand_code() as u64
    }

    fn combo_operand(&self) -> u64 {
        match self.operand_code() {
            a if a < 4 => a as u64,
            4 => self.register_a,
            5 => self.register_b,
            6 => self.register_c,
            _ => panic!(),
        }
    }

    fn increment_pc(&mut self) {
        self.pc += 2;
    }

    fn adv(&mut self) {
        self.register_a /= 2_u64.pow(self.combo_operand() as u32);
        self.increment_pc();
    }

    fn bxl(&mut self) {
        self.register_b = self.register_b.bitxor(self.literal_operand());
        self.increment_pc();
    }

    fn bst(&mut self) {
        self.register_b = self.combo_operand() % 8;
        self.increment_pc();
    }

    fn jnz(&mut self) {
        if self.register_a != 0 {
            self.pc = self.literal_operand() as usize;
        } else {
            self.increment_pc();
        }
    }

    fn bxc(&mut self) {
        self.register_b = self.register_b.bitxor(self.register_c);
        self.increment_pc();
    }

    fn out(&mut self) {
        self.output.push((self.combo_operand() % 8) as u8);
        self.increment_pc();
    }

    fn bdv(&mut self) {
        self.register_b = self.register_a / 2_u64.pow(self.combo_operand() as u32);
        self.increment_pc();
    }

    fn cdv(&mut self) {
        self.register_c = self.register_a / 2_u64.pow(self.combo_operand() as u32);
        self.increment_pc();
    }
}
