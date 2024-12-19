use crate::day09::Block::{Data, Empty};
use crate::harness::Day;
use crate::harness::Part;

pub struct Part1;

pub fn day09() -> Day<u64, u64> {
    Day::new(9, Box::new(Part1 {}), Box::new(Part2 {}))
}

impl Part<u64> for Part1 {
    fn expect_test(&self) -> u64 {
        1928
    }

    fn solve(&self, input: &[String]) -> u64 {
        let mut data = Input::from(input.first().unwrap().as_str()).data;
        let mut previous_swap_index = data.len();

        for i in 0..data.len() {
            if let Empty = data[i] {
                let swap_index = data
                    .iter()
                    .enumerate()
                    .rev()
                    .skip(data.len() - previous_swap_index)
                    .filter(|&(j, _)| j > i)
                    .find(|(_, b)| matches!(b, Data { id: _ }))
                    .map(|(i, _)| i);

                if let Some(swap_index) = swap_index {
                    data.swap(i, swap_index);
                    previous_swap_index = swap_index;
                } else {
                    break;
                }
            }
        }

        data.iter()
            .enumerate()
            .map(|(i, block)| match block {
                Empty => 0,
                Data { id } => id * i,
            })
            .sum::<usize>() as u64
    }
}

pub struct Part2;

impl Part<u64> for Part2 {
    fn expect_test(&self) -> u64 {
        2858
    }

    fn solve(&self, input: &[String]) -> u64 {
        let mut data = Input::from(input.first().unwrap().as_str()).data2;

        let mut i = data.len() - 1;

        loop {
            let current = &data[i];

            if let Block2 { block: Data { .. }, .. } = current {
                let to_swap = data
                    .iter()
                    .take(i)
                    .enumerate()
                    .find(|(_, block)| matches!(block.block, Block::Empty) && block.size >= current.size);

                if let Some((swap_index, &Block2 { size: swap_block_size, .. }, )) = to_swap {
                    if swap_block_size == current.size {
                        data.swap(i, swap_index);
                    } else {
                        let size = current.size;
                        let current =
                            std::mem::replace(&mut data[i], Block2 { size, block: Empty });

                        data[swap_index] = current;

                        i += 1;

                        data.insert(
                            swap_index + 1,
                            Block2 {
                                size: swap_block_size - size,
                                block: Empty,
                            },
                        )
                    }
                }
            }

            if i == 0 {
                break;
            }

            i -= 1;
        }

        let mut cur_index = 0;

        data.iter()
            .map(|block| {
                let result = {
                    match block.block {
                        Empty => 0,
                        Data { id } => (cur_index..(cur_index + block.size)).map(|e| e * id).sum(),
                    }
                };

                cur_index += block.size;

                result
            })
            .sum::<usize>() as u64
    }
}

#[derive(Debug)]
enum Block {
    Empty,
    Data { id: usize },
}

#[derive(Debug)]
struct Input {
    data: Vec<Block>,
    data2: Vec<Block2>,
}

impl From<&str> for Input {
    fn from(value: &str) -> Self {
        let mut next_is_data = true;

        let mut data2 = vec![];

        let data = value
            .chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .enumerate()
            .flat_map(|(index, size)| {
                let result = if next_is_data {
                    data2.push(Block2 {
                        size,
                        block: Data { id: index / 2 },
                    });
                    (0..size)
                        .map(|_| Data { id: index / 2 })
                        .collect::<Vec<_>>()
                } else if size > 0 {
                    data2.push(Block2 { size, block: Empty });
                    (0..size).map(|_| Empty).collect::<Vec<_>>()
                } else {
                    vec![]
                };

                next_is_data = !next_is_data;

                result
            })
            .collect::<Vec<_>>();

        Input { data, data2 }
    }
}

#[derive(Debug)]
struct Block2 {
    size: usize,
    block: Block,
}
