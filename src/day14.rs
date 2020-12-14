#[allow(unused_imports)]
use super::prelude::*;
type Input = Vec<Instr>;

#[derive(Copy, Clone)]
pub enum Instr {
    Mask([u8; 36]),
    Mem(usize, u64),
}

pub fn input_generator(input: &str) -> Input {
    input
        .lines()
        .map(|line| match &line[..3] {
            "mem" => {
                let (addr, val) = line[4..].split("] = ").collect_tuple().unwrap();
                Instr::Mem(addr.parse().unwrap(), val.parse().unwrap())
            }
            _ => Instr::Mask(
                line[7..]
                    .chars()
                    .map(|c| match c {
                        '0' => 0,
                        '1' => 1,
                        'X' => 2,
                        _ => panic!(),
                    })
                    .collect::<Vec<u8>>()
                    .try_into()
                    .unwrap(),
            ),
        })
        .collect()
}

pub fn part1(input: &Input) -> u64 {
    let instructions = input;
    let mut memory = HashMap::new();
    let mut mask = [2; 36];
    for &instr in instructions {
        match instr {
            Instr::Mask(new_mask) => mask = new_mask,
            Instr::Mem(addr, val) => {
                let new_val = (0..36).fold(0, |acc, pos| {
                    let bit = match mask[35 - pos] {
                        2 => val & (1 << pos),
                        i => (i as u64) << pos,
                    };
                    acc | bit
                });
                memory.insert(addr, new_val);
            }
        }
    }

    memory.values().copied().sum()
}

pub fn part2(input: &Input) -> u64 {
    let instructions = input;
    let mut memory = HashMap::new();
    let mut mask = [2; 36];
    for &instr in instructions {
        match instr {
            Instr::Mask(new_mask) => mask = new_mask,
            Instr::Mem(addr, val) => {
                let xs = mask.iter().filter(|&&b| b == 2).count();
                for i in 0..(1 << xs) {
                    let mut xpos = 0;
                    let addr = (0..36).fold(0, |acc, pos| {
                        let bit = match mask[35 - pos] {
                            0 => addr & (1 << pos),
                            1 => 1 << pos,
                            _ => {
                                let v = (i & (1 << xpos)) >> xpos;
                                xpos += 1;
                                v << pos
                            }
                        };
                        acc | bit
                    });
                    memory.insert(addr, val);
                }
            }
        }
    }

    memory.values().copied().sum()
}
