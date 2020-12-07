#[allow(unused_imports)]
use super::prelude::*;
type Input<'a> = Vec<Policy<'a>>;

pub struct Policy<'a> {
    min: usize,
    max: usize,
    c: char,
    pass: &'a str
}

pub fn input_generator(input: &str) -> Input {
    input.lines()
        .map(|line| {
            // line.parse().expect("Invalid input")
            let (min, max, c, pass) = line
                .split(&['-', ' ', ':'][..])
                .filter(|s| !s.is_empty())
                .collect_tuple()
                .expect("Invalid input");
            Policy {
                min: min.parse().expect("Invalid input"),
                max: max.parse().expect("Invalid input"),
                c: c.parse().expect("Invalid input"),
                pass
            }
        })
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
