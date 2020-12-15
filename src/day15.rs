#[allow(unused_imports)]
use super::prelude::*;
type Input = Vec<u32>;

pub fn input_generator(input: &str) -> Input {
    input
        .split(',')
        .map(|n| n.parse().expect("Invalid input"))
        .collect()
}

fn run(starting: &[u32], nth: u32) -> u32 {
    let mut history = vec![u32::MAX; nth as usize];
    starting
        .iter()
        .enumerate()
        .for_each(|(pos, &n)| history[n as usize] = pos as u32);

    let mut last = starting.last().copied().expect("Input is empty");
    let mut prev_spoken = u32::MAX;

    for i in starting.len() as u32..nth {
        last = (i - 1).saturating_sub(prev_spoken);
        prev_spoken = mem::replace(&mut history[last as usize], i);
    }

    last
}

pub fn part1(input: &Input) -> u32 {
    run(input, 2020)
}

pub fn part2(input: &Input) -> u32 {
    run(input, 30000000)
}
