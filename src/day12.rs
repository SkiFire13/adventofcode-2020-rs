#[allow(unused_imports)]
use super::prelude::*;
type Input = Vec<Instruction>;

#[derive(Copy, Clone)]
pub enum Instruction {
    Move((i32, i32)),
    Forward(i32),
    Rotate(i32),
}

pub fn input_generator(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            let c = line.chars().next().expect("Invalid input");
            let n = line
                .get(1..)
                .and_then(|n| n.parse().ok())
                .expect("Invalid input");
            match c {
                'N' => Instruction::Move((0, n)),
                'S' => Instruction::Move((0, -n)),
                'E' => Instruction::Move((n, 0)),
                'W' => Instruction::Move((-n, 0)),
                'F' => Instruction::Forward(n),
                'L' => Instruction::Rotate(n / 90 % 4),
                'R' => Instruction::Rotate(4 - n / 90 % 4),
                _ => panic!("Invalid input"),
            }
        })
        .collect()
}

pub fn part1(input: &Input) -> i32 {
    let mut pos = (0, 0);
    let mut dir = (1, 0);

    for &instr in input {
        match instr {
            Instruction::Move((x, y)) => {
                pos.0 += x;
                pos.1 += y;
            }
            Instruction::Forward(n) => {
                pos.0 += n * dir.0;
                pos.1 += n * dir.1;
            }
            Instruction::Rotate(n) => {
                for _ in 0..n {
                    dir = (-dir.1, dir.0);
                }
            }
        }
    }

    pos.0.abs() + pos.1.abs()
}

pub fn part2(input: &Input) -> i32 {
    let mut pos = (0, 0);
    let mut way = (10, 1);

    for &instr in input {
        match instr {
            Instruction::Move((x, y)) => {
                way.0 += x;
                way.1 += y;
            }
            Instruction::Forward(n) => {
                pos.0 += n * way.0;
                pos.1 += n * way.1;
            }
            Instruction::Rotate(n) => {
                for _ in 0..n {
                    way = (-way.1, way.0);
                }
            }
        }
    }

    pos.0.abs() + pos.1.abs()
}
