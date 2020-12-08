#[allow(unused_imports)]
use super::prelude::*;
type Input = Vec<Instruction>;

#[derive(Clone, Copy)]
pub enum Instruction {
    Nop(i32),
    Jmp(i32),
    Acc(i32),
}

pub fn input_generator(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            let (op, prm) = line.split_ascii_whitespace().collect_tuple().expect("Invalid input");
            let prm = prm.parse().expect("Invalid input");
            match op {
                "nop" => Instruction::Nop(prm),
                "jmp" => Instruction::Jmp(prm),
                "acc" => Instruction::Acc(prm),
                _ => panic!("Invalid input")
            }
        })
        .collect()
}

fn run_code(instructions: &[Instruction], seen: &mut [bool]) -> (bool, i32) {
    assert!(instructions.len() == seen.len());
    let mut acc = 0;
    let mut idx = 0;

    while idx < instructions.len() && !replace(&mut seen[idx], true)  {
        match instructions[idx] {
            Instruction::Nop(_) => idx += 1,
            Instruction::Jmp(j) => idx = (idx as i32 + j) as usize,
            Instruction::Acc(d) => {
                acc += d;
                idx += 1;
            }
        }
    }

    (idx < instructions.len(), acc)
}

pub fn part1(input: &Input) -> i32 {
    let instructions = input;
    let mut seen = vec![false; instructions.len()];
    run_code(&instructions, &mut seen).1
}

pub fn part2(input: &Input) -> i32 {
    let mut instructions = input.clone();
    let mut seen = vec![false; instructions.len()];

    for jn_idx in (0..instructions.len()).rev() {
        instructions[jn_idx] = match instructions[jn_idx] {
            Instruction::Nop(n) => Instruction::Jmp(n),
            Instruction::Jmp(n) => Instruction::Nop(n),
            _ => continue,
        };

        let (pte, acc) = run_code(&instructions, &mut seen);
        if !pte {
            return acc;
        }

        seen.iter_mut().for_each(|b| *b = false);
        instructions[jn_idx] = match instructions[jn_idx] {
            Instruction::Nop(n) => Instruction::Jmp(n),
            Instruction::Jmp(n) => Instruction::Nop(n),
            _ => unreachable!(),
        };
    }

    panic!("There's no solution");
}
