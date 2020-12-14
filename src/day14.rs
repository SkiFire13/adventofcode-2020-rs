#[allow(unused_imports)]
use super::prelude::*;
type Input = Vec<Instr>;

#[derive(Copy, Clone)]
pub enum Instr {
    Mask(u64, u64, u64),
    Mem(u64, u64),
}

pub fn input_generator(input: &str) -> Input {
    input
        .lines()
        .map(|line| match &line[..3] {
            "mem" => {
                let (addr, val) = line[4..]
                    .split("] = ")
                    .collect_tuple()
                    .expect("Invalid input");
                Instr::Mem(
                    addr.parse().expect("Invalid input"),
                    val.parse().expect("Invalid input"),
                )
            }
            _ => {
                let (m0, m1, mx) =
                    line[7..]
                        .chars()
                        .fold((0, 0, 0), |(mut m0, mut m1, mut mx), c| {
                            assert!(matches!(c, '0' | '1' | 'X'), "Invalid input");
                            m0 = (m0 << 1) | (c == '0') as u64;
                            m1 = (m1 << 1) | (c == '1') as u64;
                            mx = (mx << 1) | (c == 'X') as u64;
                            (m0, m1, mx)
                        });
                Instr::Mask(m0, m1, mx)
            }
        })
        .collect()
}

pub fn part1(input: &Input) -> u64 {
    let instructions = input;

    let mut memory = HashMap::new();

    let mut m0 = 0;
    let mut m1 = 0;
    let mut mx = 0;

    for &instr in instructions {
        match instr {
            Instr::Mask(nm0, nm1, nmx) => {
                m0 = nm0;
                m1 = nm1;
                mx = nmx;
            }
            Instr::Mem(addr, val) => {
                let new_val = (val & mx & !m0) | m1;
                memory.insert(addr, new_val);
            }
        }
    }

    memory.values().copied().sum()
}

pub fn part2(input: &Input) -> u64 {
    let instructions = input;

    let mut memory = fxhash::FxHashMap::default();
    memory.reserve(130000);

    let mut m1 = 0;
    let mut mx = 0;
    let mut substs = Vec::new();

    for &instr in instructions {
        match instr {
            Instr::Mask(_, nm1, nmx) => {
                m1 = nm1;
                mx = nmx;

                substs.clear();
                substs.extend((0..(1 << mx.count_ones())).map(|mut i| {
                    let mut subst = 0;
                    let mut mx = mx;
                    for pos in 0..36 {
                        subst |= (i & mx & 1) << pos;
                        i >>= mx & 1;
                        mx >>= 1;
                    }
                    subst
                }))
            }
            Instr::Mem(addr, val) => {
                for &subst in substs.iter() {
                    let addr = ((addr | m1) & !mx) | subst;
                    memory.insert(addr, val);
                }
            }
        }
    }

    memory.values().copied().sum()
}
