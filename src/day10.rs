#[allow(unused_imports)]
use super::prelude::*;
type Input = Vec<u32>;

pub fn input_generator(input: &str) -> Input {
    input
        .lines()
        .map(|line| line.parse().expect("Invalid input"))
        .chain(iter::once(0))
        .sorted()
        .collect()
}

pub fn part1(input: &Input) -> usize {
    let joltages = input;

    let mut count1 = 0;
    let mut count3 = 1;

    for (&j1, &j2) in joltages.iter().zip(joltages.iter().skip(1)) {
        match j2 - j1 {
            1 => count1 += 1,
            3 => count3 += 1,
            _ => panic!("Invalid input"),
        }
    }

    count1 * count3
}

pub fn part2(input: &Input) -> usize {
    let joltages = input;

    let mut cmbs = [0, 0, 1];

    for (&j1, &j2) in joltages.iter().zip(joltages.iter().skip(1)) {
        cmbs = match j2 - j1 {
            1 => [cmbs[1], cmbs[2], cmbs[0] + cmbs[1] + cmbs[2]],
            3 => [0, 0, cmbs[2]],
            _ => panic!("Invalid input"),
        };
    }

    cmbs[2]
}
