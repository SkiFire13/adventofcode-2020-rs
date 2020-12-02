#[allow(unused_imports)]
use super::prelude::*;
type Input = Vec<Policy>;

#[derive(FromStr)]
#[display("{min}-{max} {c}: {pass}")]
pub struct Policy {
    min: usize,
    max: usize,
    c: char,
    pass: String
}

pub fn input_generator(input: &str) -> Input {
    input.lines()
        .map(|line| line.parse().expect("Invalid input"))
        .collect()
}

pub fn part1(input: &Input) -> usize {
    input.iter()
        .filter(|policy| {
            let r = policy.pass.chars().filter(|&c| c == policy.c).count();
            policy.min <= r && r <= policy.max 
        })
        .count()
}

pub fn part2(input: &Input) -> usize {
    input.iter()
        .filter(|policy| {
            let c1 = policy.pass.chars().nth(policy.min - 1);
            let c2 = policy.pass.chars().nth(policy.max - 1);
            (Some(policy.c) == c1) ^ (Some(policy.c) == c2)
        })
        .count()
}
