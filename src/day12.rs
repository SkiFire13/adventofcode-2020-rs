#[allow(unused_imports)]
use super::prelude::*;
type Input = Vec<Instruction>;

#[derive(Copy, Clone)]
pub enum Instruction {
    North(i32),
    South(i32),
    East(i32),
    West(i32),
    Forward(i32),
    Left(i32),
    Right(i32),
}

pub fn input_generator(input: &str) -> Input {
    input
        .lines()
        .map(|line| match line.chars().next() {
            Some('N') => Instruction::North(line[1..].parse().expect("Invalid input")),
            Some('S') => Instruction::South(line[1..].parse().expect("Invalid input")),
            Some('E') => Instruction::East(line[1..].parse().expect("Invalid input")),
            Some('W') => Instruction::West(line[1..].parse().expect("Invalid input")),
            Some('F') => Instruction::Forward(line[1..].parse().expect("Invalid input")),
            Some('L') => Instruction::Left(line[1..].parse().expect("Invalid input")),
            Some('R') => Instruction::Right(line[1..].parse().expect("Invalid input")),
            _ => panic!("Invalid input")
        })
        .collect()
}

pub fn part1(input: &Input) -> i32 {
    let mut pos = (0, 0);
    let mut dir = (1, 0);

    for &instr in input {
        match instr {
            Instruction::North(n) => pos.1 += n,
            Instruction::South(n) => pos.1 -= n,
            Instruction::East(n) => pos.0 += n,
            Instruction::West(n) => pos.0 -= n,
            Instruction::Forward(n) => {
                pos.0 += n * dir.0;
                pos.1 += n * dir.1;
            }
            Instruction::Left(n) => {
                for _ in 0..n/90 {
                    dir = (-dir.1, dir.0);
                }
            },
            Instruction::Right(n) => {
                for _ in 0..n/90 {
                    dir = (dir.1, -dir.0);
                }
            },
        }
    }

    pos.0.abs() + pos.1.abs()
}

pub fn part2(input: &Input) -> i32 {
    let mut pos = (0, 0);
    let mut way = (10, 1);

    for &instr in input {
        match instr {
            Instruction::North(n) => way.1 += n,
            Instruction::South(n) => way.1 -= n,
            Instruction::East(n) => way.0 += n,
            Instruction::West(n) => way.0 -= n,
            Instruction::Forward(n) => {
                pos.0 += n * way.0;
                pos.1 += n * way.1;
            }
            Instruction::Left(n) => {
                for _ in 0..n/90 {
                    way = (-way.1, way.0);
                }
            },
            Instruction::Right(n) => {
                for _ in 0..n/90 {
                    way = (way.1, -way.0);
                }
            },
        }
    }

    pos.0.abs() + pos.1.abs()
}
