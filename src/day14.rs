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
                    .rev()
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
                let new_val = mask
                    .iter()
                    .enumerate()
                    .map(|(pos, &bit)| match bit {
                        2 => val & (1 << pos),
                        i => (i as u64) << pos,
                    })
                    .fold(0, |acc, mask| acc | mask);
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
                for mut i in 0..(1 << xs) {
                    let addr = mask
                        .iter()
                        .enumerate()
                        .fold(addr, |mut acc, (pos, &bit)| {
                            if bit != 2 {
                                acc |= (bit as usize) << pos;
                            } else {
                                acc &= !(1 << pos);
                                acc |= (i & 1) << pos;
                                i >>= 1;
                            }
                            acc
                        });
                    memory.insert(addr, val);
                }
            }
        }
    }

    memory.values().copied().sum()
}
